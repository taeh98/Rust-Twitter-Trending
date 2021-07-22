use tokio;
use priority_queue::PriorityQueue;
mod get_tweets;
mod process_tweets;

const NUMBER_TO_SHOW: usize = 10;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let tweets: Vec<String> = get_tweets::get_recent_tweets().await;
    const COUNTS: PriorityQueue<process_tweets::WordAndCount> = process_tweets::process_tweets(tweets);
    const TOP_HASHTAGS: Vec<process_tweets::WordAndCount> = get_top_words(COUNTS, true);
    const TOP_WORDS: Vec<process_tweets::WordAndCount> = get_top_words(COUNTS, false);

    print_top_words(TOP_WORDS, TOP_HASHTAGS)
}

fn get_top_words(counts: PriorityQueue<process_tweets::WordAndCount>, hashtag_not_word: bool) -> Vec<process_tweets::WordAndCount> {
    let res: Vec<process_tweets::WordAndCount> = Vec::new();

    for (let i = 0; i < counts.len() && res.len() < NUMBER_TO_SHOW; i += 1) {
        const current_tc: process_tweets::WordAndCount = counts[i];
        if current_tc["tweet"].startsWith("#") == hashtag_not_word:
            res.push(current_tc);
    }

    res
}

fn print_top_words(top_words: Vec<process_tweets::WordAndCount>, top_hashtags: Vec<process_tweets::WordAndCount>) {

}