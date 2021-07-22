use std::collections::hash_map::{HashMap, RandomState};
use priority_queue::PriorityQueue;
use rayon::prelude::*;

pub struct WordAndCount {
    word: String,
    count: BigInt
}

pub async fn process_tweets(tweets: Vec<String>) -> PriorityQueue<WordAndCount> {
    processed_tweets_to_priority_queue(
    tweets.par_iter()
        .map(|tweet: String| process_tweet(tweet))
        .reduce(|a: HashMap<String, i128, RandomState>, b: HashMap<String, i128, RandomState>| combine_processed_tweets(a, b))
    )
}

async fn process_tweet(tweet: String) -> HashMap<String, i128, RandomState> {
    const WORDS: Vec<String> = tweet.split_whitespace();
    let mut res: HashMap<String, i128> = HashMap::new();

    for word in WORDS {
        const COUNT: i128 = WORDS.into_iter().filter(|s: String| s == word).count() as i128;
        if ! res.contains_key(word.as_str()) {
            res.insert(word, COUNT);
        }
    }

    res
}

fn combine_processed_tweets(a: HashMap<String, i128, RandomState>, b: HashMap<String, i128, RandomState>) -> HashMap<String, i128, RandomState> {
    const KEYS: Vec<String> = a.keys().chain(b.keys()).collect();
    let mut res: HashMap<String, i128, RandomState> = HashMap::new();

    for key in KEYS {
        if b.contains_key(key.as_str()) {
            const B_COUNT: i128 = b.get(key).unwrap();
            if a.contains_key(key.as_str()) {
                const A_COUNT: i128 = a.get(key).unwrap();
                res.insert(key, A_COUNT + B_COUNT);
            }
        }
         else {
             match a.get(key.as_str()) {
                 Some(val) => res.insert(key, val.clone()),
                 _ => continue
             }
         }
    }

    res
}

fn processed_tweets_to_priority_queue(pt: HashMap<String, i128>) -> PriorityQueue<WordAndCount> {
    let res: PriorityQueue<WordAndCount> = PriorityQueue::new();
    pt.into_par_iter().for_each(|word: String, count: i128| {
        res.insert({word: word; count: count});
    });
    res
}