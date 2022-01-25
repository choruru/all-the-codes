#[macro_use]
extern crate maplit;

use anyhow::Result;
use cached::proc_macro::cached;
use chrono::prelude::*;
use flexi_logger::{opt_format, Duplicate, FileSpec, Logger};
use lazy_static::lazy_static;
use log::info;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use tokio::spawn;
use tokio::time::{sleep, Duration};

const DISCORD_WEBHOOK_KEY: &str = if cfg!(feature = "dev") {
    "DISCORD_WEBHOOK_DEV"
} else {
    "DISCORD_WEBHOOK_PROD"
};

#[derive(Debug)]
struct Post {
    username: &'static str,
    content: &'static str,
}

lazy_static! {
    static ref CHORE: HashMap<&'static str, Post> = hashmap! {
        "laundry" => Post {username: "Chore Master Dog", content: "https://storage.googleapis.com/leonard_meme/laundry.png"},
        "dishes" => Post {username: "Chore Master Dog", content: "https://storage.googleapis.com/leonard_meme/dish.png"},
        "bathroom" => Post {username: "Chore Master Dog", content: "https://storage.googleapis.com/leonard_meme/bathroom.png"},
        "payment" => Post {username: "Chore Master Dog", content: "https://storage.googleapis.com/leonard_meme/pay.png"},
        "rest" => Post {username: "Chore Master Dog", content: "https://storage.googleapis.com/leonard_meme/potty.png"},
    };
    static ref DOG_CARE: HashMap<&'static str, Post> = hashmap! {
        "food" => Post {username: "Hungry Dog", content: "https://storage.googleapis.com/leonard_meme/food.png"},
        "potty" => Post {username: "Poopy Dog", content: "https://storage.googleapis.com/leonard_meme/potty.png"},
        "shower" => Post {username: "Smelly Dog", content: "https://storage.googleapis.com/leonard_meme/shower.png"},
        "play" => Post {username: "Lonely Dog", content: "https://storage.googleapis.com/leonard_meme/play.png"},
    };
}

#[derive(Deserialize)]
struct SecretManagerResponse {
    payload: HashMap<String, String>,
}

#[cached(result = true)]
async fn get_discord_webhook_url() -> Result<String> {
    let auth_token = env::var("GCP_AUTH_TOKEN").expect("GCP_AUTH_TOKEN not found.");
    info!("Getting discord webhook url...");
    let secret_manager = format!("https://secretmanager.googleapis.com/v1/projects/demandy-dog-bot/secrets/{}/versions/latest:access", DISCORD_WEBHOOK_KEY);
    let res = reqwest::Client::new()
        .get(secret_manager)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", auth_token))
        .send()
        .await?;
    let body = res.json::<SecretManagerResponse>().await?;
    Ok(String::from_utf8(base64::decode(&body.payload["data"])?)?)
}

async fn send(post: &Post) -> Result<()> {
    info!("Submitting {:?}...", post);
    reqwest::Client::new()
        .post(get_discord_webhook_url().await?)
        .form(&[("username", post.username), ("content", post.content)])
        .send()
        .await?;
    Ok(())
}

async fn daily_chore_alert(weekday: Weekday) -> Result<()> {
    let post = match weekday {
        Weekday::Mon => &CHORE["dishes"],
        Weekday::Tue => &CHORE["laundry"],
        Weekday::Wed => &CHORE["dishes"],
        Weekday::Thu => &CHORE["laundry"],
        Weekday::Fri => &CHORE["dishes"],
        Weekday::Sat => &CHORE["laundry"],
        Weekday::Sun => &CHORE["rest"],
    };

    send(post).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_env_or_str("info")?
        .format(opt_format)
        .log_to_file(FileSpec::default().suppress_timestamp())
        .duplicate_to_stderr(Duplicate::Info) // print warnings and errors also to the console
        .start()?;

    // let it cache
    let _ = get_discord_webhook_url().await?;

    loop {
        let now = FixedOffset::west(6 * 3600).from_utc_datetime(&Utc::now().naive_utc());
        info!("Woke up at {}", now);
        let (weekday, month, day, hour, min) = (
            now.weekday(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
        );

        // chore
        if let (21, 0) = (hour, min) {
            spawn(daily_chore_alert(weekday));
        }
        if let 26..=28 = day {
            if let (10, 0) | (15, 0) | (20, 0) = (hour, min) {
                spawn(send(&CHORE["payment"]));
            }
        }

        // dog care
        if let (8, 30) | (15, 30) = (hour, min) {
            spawn(send(&DOG_CARE["food"]));
        }
        if let (11, 0) | (14, 0) | (20, 0) | (23, 0) = (hour, min) {
            spawn(send(&DOG_CARE["potty"]));
        }
        if let (17, 0) = (hour, min) {
            spawn(send(&DOG_CARE["play"]));
        }

        sleep(Duration::from_secs(60)).await;
    }
}
