use std::time::Instant;

use priority_queue::PriorityQueue;
use serde::{Deserialize, Serialize};

mod get_data;
mod get_tweets;
mod process_results;
mod process_tweets;
mod processed_tweets_output;

const NUM_REPEATS_BEFORE_MEAN: usize = 200;

#[derive(Serialize, Deserialize)]
pub struct TimeTakenTweetProcessingSpeedValuePair {
    time_taken_seconds: f64,
    processing_speed_tweets_per_second: f64,
}

impl TimeTakenTweetProcessingSpeedValuePair {
    pub fn new(
        time_taken_seconds: f64,
        processing_speed_tweets_per_second: f64,
    ) -> TimeTakenTweetProcessingSpeedValuePair {
        TimeTakenTweetProcessingSpeedValuePair {
            time_taken_seconds,
            processing_speed_tweets_per_second,
        }
    }
    pub fn get_time_taken_seconds(&self) -> f64 {
        self.time_taken_seconds
    }
    pub fn get_processing_speed_tweets_per_second(&self) -> f64 {
        self.processing_speed_tweets_per_second
    }
}

#[derive(Serialize, Deserialize)]
pub struct TweetProcessingResult {
    name: String,
    time_taken_tweets_per_sec_values: Vec<TimeTakenTweetProcessingSpeedValuePair>,
}

impl TweetProcessingResult {
    pub fn new(
        name: String,
        time_taken_tweets_per_sec_values: Vec<TimeTakenTweetProcessingSpeedValuePair>,
    ) -> TweetProcessingResult {
        TweetProcessingResult {
            name,
            time_taken_tweets_per_sec_values,
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_time_taken_tweets_per_sec_values(
        &self,
    ) -> &Vec<TimeTakenTweetProcessingSpeedValuePair> {
        &self.time_taken_tweets_per_sec_values
    }
}

fn main() {
    println!("Getting the tweets data files or checking they are already saved.");
    get_data::check_or_get_tweets_data();
    println!(
        "The tweets data files are all present and intact. Proceeding to extract tweets data from them."
    );

    match get_tweets::get_tweets() {
        Some(tweets) => {
            println!("Getting the top words text.");
            let counts: PriorityQueue<String, i128> = process_tweets::process_tweets(&tweets, true);
            let num_tweets: usize = counts.len();

            processed_tweets_output::print_top_words_text_from_counts(&counts);

            println!("Running tweet processing algorithms.");
            let algorithm_results: Vec<TweetProcessingResult> = vec![
                run_rust_tweet_processing_algorithm(&tweets, num_tweets, true),
                run_rust_tweet_processing_algorithm(&tweets, num_tweets, false),
            ];

            //TODO: implement parallelised and non-parallelised algorithms in Python to compare to

            println!("Done running tweet processing algorithms.");
            process_results::process_results(algorithm_results);
        }
        _ => panic!("Couldn't get tweets data."),
    }
}

fn run_rust_tweet_processing_algorithm(
    tweets: &Vec<String>,
    num_tweets: usize,
    parallel: bool,
) -> TweetProcessingResult {
    let mut time_taken_tweets_per_sec_values: Vec<TimeTakenTweetProcessingSpeedValuePair> =
        Vec::new();

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
        time_taken_tweets_per_sec_values.push(TimeTakenTweetProcessingSpeedValuePair::new(
            time_taken_secs,
            tweets_per_sec,
        ));
    }

    TweetProcessingResult::new(algorithm_name, time_taken_tweets_per_sec_values)
}
