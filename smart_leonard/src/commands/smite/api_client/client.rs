use anyhow::Result;
use cached::proc_macro::cached;
use chrono::prelude::*;
use chrono::Duration;
use log::debug;
use md5;
use reqwest;
use serde::de::DeserializeOwned;
use std::env;

use super::model::{god::*, match_stat::*, session::*};

const BASE_URL: &'static str = "https://api.smitegame.com/smiteapi.svc";

#[cached]
fn get_dev_id() -> String {
    env::var("HIREZ_DEV_ID").expect("HIREZ_DEV_ID not found.")
}

#[cached]
fn get_auth_key() -> String {
    env::var("HIREZ_AUTH_TOKEN").expect("HIREZ_AUTH_TOKEN not found.")
}

#[cached(result = true, time = 890)] // 15 mins - 10 secs
pub async fn get_sesssion_id() -> Result<String> {
    Ok(createsession().await?.session_id)
}

fn get_time() -> String {
    Utc::now().format("%Y%m%d%H%M%S").to_string()
}

fn get_yesterday() -> String {
    (Utc::now() - Duration::days(1))
        .format("%Y%m%d")
        .to_string()
}

fn get_signature(method: &str, time: &str) -> String {
    let data = format!("{}{}{}{}", get_dev_id(), method, get_auth_key(), time);
    format!("{:x}", md5::compute(data.into_bytes()))
}

async fn call_api<T: DeserializeOwned>(url: &str) -> Result<T> {
    debug!("{:#?}", reqwest::get(url).await?.text().await?);
    Ok(reqwest::get(url).await?.json::<T>().await?)
}

pub async fn createsession() -> Result<CreateSessionRes> {
    let method = "createsession";
    let time = get_time();
    let url = format!(
        "{}/{}Json/{}/{}/{}",
        BASE_URL,
        method,
        get_dev_id(),
        get_signature(method, &time),
        time
    );
    call_api::<CreateSessionRes>(&url).await
}

pub async fn getgods(sess_id: &str) -> Result<Vec<God>> {
    let method = "getgods";
    let time = get_time();
    let url = format!(
        "{}/{}Json/{}/{}/{}/{}/1",
        BASE_URL,
        method,
        get_dev_id(),
        get_signature(method, &time),
        sess_id,
        time
    );
    call_api::<Vec<God>>(&url).await
}

pub async fn getleagueleseasons(sess_id: &str) -> Result<Vec<GetLeagueSeasonsRes>> {
    let method = "getleagueseasons";
    let time = get_time();
    let queue = 451;
    let url = format!(
        "{}/{}Json/{}/{}/{}/{}/{}",
        BASE_URL,
        method,
        get_dev_id(),
        get_signature(method, &time),
        sess_id,
        time,
        queue
    );
    call_api::<Vec<GetLeagueSeasonsRes>>(&url).await
}

pub async fn getleagueleaderboard(
    sess_id: &str,
    round: i32,
) -> Result<Vec<GetLeagueLeaderBoardRes>> {
    let method = "getleagueleaderboard";
    let time = get_time();
    let queue = 451;
    let tier = 20;
    let url = format!(
        "{}/{}Json/{}/{}/{}/{}/{}/{}/{}",
        BASE_URL,
        method,
        get_dev_id(),
        get_signature(method, &time),
        sess_id,
        time,
        queue,
        tier,
        round
    );
    call_api::<Vec<GetLeagueLeaderBoardRes>>(&url).await
}

pub async fn getmatchidsbyqueue(sess_id: &str) -> Result<Vec<GetMatchIdsByQueueRes>> {
    let method = "getmatchidsbyqueue";
    let time = get_time();
    let queue = 451;
    let url = format!(
        "{}/{}Json/{}/{}/{}/{}/{}/{}/{}",
        BASE_URL,
        method,
        get_dev_id(),
        get_signature(method, &time),
        sess_id,
        time,
        queue,
        get_yesterday(),
        -1,
    );
    call_api::<Vec<GetMatchIdsByQueueRes>>(&url).await
}

pub async fn getmatchdetailsbatch(
    sess_id: &str,
    match_ids: Vec<String>,
) -> Result<Vec<GetMatchDetailsRes>> {
    let method = "getmatchdetailsbatch";
    let time = get_time();
    let url = format!(
        "{}/{}Json/{}/{}/{}/{}/{}",
        BASE_URL,
        method,
        get_dev_id(),
        get_signature(method, &time),
        sess_id,
        time,
        match_ids.join(",")
    );
    call_api::<Vec<GetMatchDetailsRes>>(&url).await
}

mod tests {
    #[tokio::test]
    async fn get_gods_works() {
        let sess_id = super::get_sesssion_id().await.unwrap();
        let gods = super::getgods(&sess_id).await.unwrap();
        println!("{:#?}", gods)
    }

    #[tokio::test]
    async fn get_league_leaderboard_works() {
        let sess_id = super::get_sesssion_id().await.unwrap();
        let round = super::getleagueleseasons(&sess_id).await.unwrap()[0].round;
        let leaderboards = super::getleagueleaderboard(&sess_id, round).await.unwrap();
        println!("{:#?}", leaderboards)
    }

    #[tokio::test]
    async fn get_match_detail_batch_works() {
        let sess_id = super::get_sesssion_id().await.unwrap();
        let matches = super::getmatchidsbyqueue(&sess_id).await.unwrap();
        let match_ids = matches
            .into_iter()
            .take(10)
            .map(|m| m.match_id)
            .collect::<Vec<String>>();
        let matches = super::getmatchdetailsbatch(&sess_id, match_ids)
            .await
            .unwrap();
        println!("{:#?}", matches)
    }
}
