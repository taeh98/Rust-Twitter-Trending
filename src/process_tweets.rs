use std::collections::hash_map::{HashMap, RandomState};

use priority_queue::PriorityQueue;
use rayon::prelude::*;

pub fn process_tweets(tweets: Vec<String>) -> PriorityQueue<String, i128> {
    processed_tweets_to_priority_queue(
        tweets.par_iter()
            .map(|tweet: &String| process_tweet(tweet))
            .reduce(|| HashMap::new(),
                    |a: HashMap<String, i128, RandomState>, b: HashMap<String, i128, RandomState>| {
                        combine_processed_tweets(&a, &b);
                        a
                    })
    )
}

fn process_tweet(tweet: &String) -> HashMap<String, i128, RandomState> {
    let words: Vec<String> = tweet.clone().split_whitespace().into_iter().map(|val: &str| String::from(val)).collect();
    let mut res: HashMap<String, i128> = HashMap::new();

    for word in words.clone() {
        let count: i128 = words.clone().into_iter().filter(|s: &String| s.clone() == word).count() as i128;
        if !res.contains_key(word.as_str()) {
            res.insert(word, count);
        }
    }

    res
}

fn combine_processed_tweets(a: &HashMap<String, i128, RandomState>, b: &HashMap<String, i128, RandomState>) -> HashMap<String, i128, RandomState> {
    let keys: Vec<String> = a.keys().chain(b.keys()).map(|s: &String| s.clone()).collect();
    let mut res: HashMap<String, i128, RandomState> = HashMap::new();

    for key in keys {
        let key_str = key.as_str();

        if b.contains_key(key_str) {
            let b_count: i128 = b.get(key_str).unwrap().clone();
            if a.contains_key(key_str) {
                let a_count: i128 = a.get(key_str).unwrap().clone();
                res.insert(key, a_count + b_count);
            }
        } else {
            match a.get(key_str) {
                Some(val) => res.insert(key, val.clone()),
                _ => continue
            };
        }
    }

    res
}

fn processed_tweets_to_priority_queue(pt: HashMap<String, i128>) -> PriorityQueue<String, i128> {
    let mut res: PriorityQueue<String, i128> = PriorityQueue::new();

    for tuple_val in pt.into_iter() {
        res.push(tuple_val.0, tuple_val.1);
    }

    res
}