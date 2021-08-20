use std::time::Instant;

use priority_queue::PriorityQueue;
use serde::{Deserialize, Serialize};

mod get_data;
mod get_tweets;
mod process_tweets;
mod processed_tweets_output;

const NUM_REPEATS_BEFORE_MEAN: i8 = 15;

#[derive(Serialize, Deserialize)]
struct TweetProcessingResult {
    name: String,
    time_taken_secs: f64,
    tweets_processed_per_sec: f64,
}

fn main() {
    println!("Getting the tweets data or checking they are already saved.");
    get_data::check_or_get_tweets_data();
    println!("The tweets data are present, proceeding to process them.");

    match get_tweets::get_tweets() {
        Some(tweets) => {
            let num_tweets: usize = tweets.len();
            let start_time: Instant = Instant::now();

            let counts: PriorityQueue<String, i128> = process_tweets::process_tweets(&tweets, true);

            let time_taken_secs: f64 = (start_time.elapsed().as_millis() as f64) / 1000.0;

            processed_tweets_output::print_top_words_from_counts(&counts);
        }
        _ => panic!("Couldn't get tweets data."),
    }
}
