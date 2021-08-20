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
            println!("Getting the top words text.");
            let counts: PriorityQueue<String, i128> = process_tweets::process_tweets(&tweets, true);
            processed_tweets_output::print_top_words_text_from_counts(&counts);

            println!("Running tweet processing algorithms.");
        }
        _ => panic!("Couldn't get tweets data."),
    }
}

fn run_rust_tweet_processing_algorithm(
    tweets: &Vec<String>,
    num_tweets: usize,
    parallel: bool,
) -> TweetProcessingResult {
    let mut sum_time_taken_secs: f64 = 0.0;
    let mut sum_tweets_per_sec: f64 = 0.0;
    let algorithm_name = format!(
        "Rust {} map-reduce",
        if parallel {
            "parallelised"
        } else {
            "non-parallelised"
        }
    );

    println!("Running the {} algorithm.", algorithm_name);

    for repeat in 1..=NUM_REPEATS_BEFORE_MEAN {
        println!("Starting repeat {} of {}.", repeat, NUM_REPEATS_BEFORE_MEAN);
        let start_time: Instant = Instant::now();
        process_tweets::process_tweets(&tweets, true);
        let time_taken_secs: f64 = (start_time.elapsed().as_millis() as f64) / 1000.0;
        let tweets_per_sec: f64 = (num_tweets as f64) / time_taken_secs;
        sum_tweets_per_sec += tweets_per_sec;
        sum_time_taken_secs += time_taken_secs;
    }

    TweetProcessingResult {
        name: algorithm_name,
        time_taken_secs: sum_time_taken_secs / NUM_REPEATS_BEFORE_MEAN as f64,
        tweets_processed_per_sec: sum_tweets_per_sec / NUM_REPEATS_BEFORE_MEAN as f64,
    }
}
