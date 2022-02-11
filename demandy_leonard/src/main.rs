use anyhow::Result;
use cached::proc_macro::cached;
use chrono::prelude::*;
use chrono::Utc;
use cron_clock::{Schedule, ScheduleIteratorOwned};
use flexi_logger::{opt_format, Duplicate, FileSpec, Logger};
use futures::future::FutureExt;
use futures::future::Shared;
use log::info;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::iter::Peekable;
use std::process::Output;
use std::str::FromStr;
use std::vec;
use tokio::spawn;
use tokio::time::{sleep, Duration};

const DISCORD_WEBHOOK_KEY: &str = if cfg!(feature = "dev") {
    "DISCORD_WEBHOOK_DEV"
} else {
    "DISCORD_WEBHOOK_PROD"
};

struct Job<F: Future> {
    name: String,
    schedule: Peekable<ScheduleIteratorOwned<FixedOffset>>,
    future: Shared<F>,
}

impl<F: Future> Job<F> {
    fn new(name: &str, cron_format_schedule: &str, future: Shared<F>) -> Job<F> {
        let job = Job {
            name: name.to_string(),
            schedule: get_schedule_iterator(cron_format_schedule).unwrap(),
            future: future,
        };

        let upcoming_3 = job
            .get_upcoming_n(3)
            .into_iter()
            .map(|x| x.format("%Y-%m-%d %H:%M:%S (%a)").to_string())
            .collect::<Vec<String>>()
            .join(", ");

        info!(
            "Created Job '{}'. The next 3 execution times are; {}.",
            job.name, upcoming_3
        );

        job
    }
    fn get_upcoming_n(&self, n: usize) -> Vec<DateTime<FixedOffset>> {
        self.schedule
            .clone()
            .take(n)
            .collect::<Vec<DateTime<FixedOffset>>>()
    }
}

#[derive(Debug)]
struct Post {
    username: &'static str,
    content: &'static str,
}

enum Chore {
    Laundry,
    Dishes,
    Bathroom,
    Payment,
    Rest,
}

impl Chore {
    fn get_post(&self) -> Post {
        let content = match self {
            Chore::Laundry => "https://storage.googleapis.com/leonard_meme/laundry.png",
            Chore::Dishes => "https://storage.googleapis.com/leonard_meme/dish.png",
            Chore::Bathroom => "https://storage.googleapis.com/leonard_meme/dish.png",
            Chore::Payment => "https://storage.googleapis.com/leonard_meme/pay.png",
            Chore::Rest => "https://storage.googleapis.com/leonard_meme/rest.png",
        };

        Post {
            username: "Chore Master Dog",
            content: content,
        }
    }
}

#[derive(Debug)]
enum DogCare {
    Food,
    Potty,
    Shower,
    Play,
}

impl DogCare {
    fn get_post(&self) -> Post {
        let content = match self {
            DogCare::Food => "https://storage.googleapis.com/leonard_meme/food.png",
            DogCare::Potty => "https://storage.googleapis.com/leonard_meme/potty.png",
            DogCare::Shower => "https://storage.googleapis.com/leonard_meme/shower.png",
            DogCare::Play => "https://storage.googleapis.com/leonard_meme/play.png",
        };

        Post {
            username: "Demandy Dog",
            content: content,
        }
    }
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

async fn send(post: Post) -> () {
    info!("Submitting {:?}...", post);
    reqwest::Client::new()
        .post(get_discord_webhook_url().await.unwrap())
        .form(&[("username", post.username), ("content", post.content)])
        .send()
        .await
        .unwrap();
    ()
}

fn get_schedule_iterator(
    cron_format_str: &str,
) -> Result<Peekable<ScheduleIteratorOwned<FixedOffset>>> {
    // Example
    // sec  min   hour   day of month   month   day of week   year
    // "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
    let shed = Schedule::from_str(cron_format_str)?
        .upcoming_owned(FixedOffset::west(6 * 3600))
        .peekable();

    Ok(shed)
}

fn define_jobs() -> Result<Vec<Job<impl Future<Output = ()>>>> {
    let mut daily_chore = vec![
        Job::new(
            "Dishes",
            "0 0 21 * * Mon,Wed,Fri *",
            send(Chore::Dishes.get_post()).shared(),
        ),
        Job::new(
            "Laundry",
            "0 0 21 * * Tue,Thu,Sat *",
            send(Chore::Laundry.get_post()).shared(),
        ),
        Job::new(
            "Rest",
            "0 0 21 * * Sun *",
            send(Chore::Rest.get_post()).shared(),
        ),
    ];

    let mut other_chore = vec![Job::new(
        "Payment",
        "0 0 10,15,20 26-28 * * *",
        send(Chore::Payment.get_post()).shared(),
    )];

    let mut dog_cares = vec![
        Job::new(
            "Dog Food",
            "0 30 8,15 * * * *",
            send(DogCare::Food.get_post()).shared(),
        ),
        Job::new(
            "Dog Potty",
            "0 0 11,14,20 * * * *",
            send(DogCare::Potty.get_post()).shared(),
        ),
        Job::new(
            "Dog Play",
            "0 0 17 * * * *",
            send(DogCare::Play.get_post()).shared(),
        ),
    ];

    let mut jobs = vec![];
    jobs.append(&mut daily_chore);
    jobs.append(&mut other_chore);
    jobs.append(&mut dog_cares);

    Ok(jobs)
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

    let mut jobs = define_jobs()?;
    let offset = FixedOffset::west(6 * 3600);
    loop {
        let now = offset.from_utc_datetime(&Utc::now().naive_utc());
        info!("Woke up at {}", now);

        for job in jobs.iter_mut() {
            if job.schedule.peek().unwrap() < &now {
                spawn(job.future.clone());
            }
            while job.schedule.peek().unwrap() < &now {
                job.schedule.next();
            }
        }

        sleep(Duration::from_secs(60)).await;
    }
}
