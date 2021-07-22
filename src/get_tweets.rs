use chrono::{DateTime, Duration, Utc};
use reqwest::{get};
use std::ops::Sub;

const GET_TWEETS_API_ENDPOINT: &str = "https://api.twitter.com/2/tweets/search/recent";

fn get_one_hour_ago_iso_string() -> String {
    let time_period: Duration = Duration::hours(1);
    let now: DateTime<Utc> = Utc::now();
    now.sub(time_period).to_rfc3339()
}

async fn get_tweets_from_endpoint(start_time_string: String) {
    println!("get_tweets_from_endpoint()");

    let body = get("https://news.ycombinator.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("body = {:?}", body);
}

pub async fn get_recent_tweets() -> Vec<String> {
    println!("get_recent_tweets()");

    let start_time_string: String = get_one_hour_ago_iso_string();
    let tweets: Vec<String> = Vec::new();

    get_tweets_from_endpoint(start_time_string).await;

    tweets
}