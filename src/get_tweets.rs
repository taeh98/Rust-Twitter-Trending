use chrono::{DateTime, Duration, Utc};
use std::ops::Sub;

const GET_TWEETS_API_ENDPOINT: &str = "https://api.twitter.com/2/tweets/search/recent";
const TIME_PERIOD: Duration = Duration::hours(1);

#[derive(Deserialize)]
struct EndpointResponse {
    origin: String,
}

fn get_one_hour_ago_iso_string() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.sub(TIME_PERIOD);
    now.to_rfc3339()
}

fn get_tweets_from_endpoint() {
    const START_TIME_STRING: String = get_one_hour_ago_iso_string();

    reqwest::get(GET_TWEETS_API_ENDPOINT).json::<EndpointResponse>()
        .await?;
}

fn parse_tweets() -> Vec<String> {

}

fn parse_tweet() -> Option<String> {

}

fn get_recent_tweets() -> Vec<String> {
    parse_tweets(get_tweets_from_endpoint())
}