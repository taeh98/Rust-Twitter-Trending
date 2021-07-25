use std::sync::Mutex;

use dashmap::mapref::multiple::RefMulti;
use dashmap::DashMap;
use priority_queue::PriorityQueue;
use rayon::prelude::*;

//TODO: replace PriorityQueue with std::collections::BinaryHeap
//TODO: replace DashMap with std::collections::HashMap
//TODO: abstract word-count pairs to a separate data type
//TODO: try to debug errors that came from using reduce() rather than reduce_with() throughout project
//TODO: integrate the use of "if let" throughout the project

pub fn process_tweets(tweets: Vec<String>) -> PriorityQueue<String, i128> {
    processed_tweets_to_priority_queue(
        tweets
            .par_iter()
            .map(|tweet: &String| process_tweet(tweet))
            .reduce_with(|a: DashMap<String, i128>, b: DashMap<String, i128>| {
                combine_processed_tweets(&a, &b)
            })
            .unwrap(),
    )
}

fn process_tweet(tweet: &String) -> DashMap<String, i128> {
    let words: Vec<String> = tweet
        .clone()
        .split_whitespace()
        .into_iter()
        .map(|val: &str| String::from(val))
        .collect();
    let res: DashMap<String, i128> = DashMap::new();

    for word in words.clone() {
        let count: i128 = words
            .clone()
            .into_iter()
            .filter(|s: &String| s.clone() == word)
            .count() as i128;
        if !res.contains_key(word.as_str()) {
            res.insert(word, count);
        }
    }

    res
}

fn get_dashmap_keys(a: &DashMap<String, i128>) -> Vec<String> {
    a.into_par_iter()
        .map(|a: RefMulti<String, i128>| a.key().clone())
        .collect()
}

fn combine_processed_tweets(
    a: &DashMap<String, i128>,
    b: &DashMap<String, i128>,
) -> DashMap<String, i128> {
    let keys: Vec<String> = get_dashmap_keys(a)
        .into_iter()
        .chain(get_dashmap_keys(b).into_iter())
        .collect();
    let res: DashMap<String, i128> = DashMap::new();

    keys.into_par_iter().for_each(|key: String| {
        let key_str = key.as_str();

        match b.get(key_str) {
            Some(b_count) => match a.get(key_str) {
                Some(a_count) => {
                    res.insert(key, a_count.clone() + b_count.clone());
                }
                _ => {
                    res.insert(key, b_count.clone());
                }
            },
            _ => {
                match a.get(key_str) {
                    Some(val) => {
                        res.insert(key, val.clone());
                    }
                    _ => {}
                };
            }
        }
    });

    res
}

fn processed_tweets_to_priority_queue(pt: DashMap<String, i128>) -> PriorityQueue<String, i128> {
    let res: Mutex<PriorityQueue<String, i128>> = Mutex::new(PriorityQueue::new());

    pt.into_par_iter().for_each(|tuple_val: (String, i128)| {
        let mut q = res.lock().unwrap();
        q.push(tuple_val.0, tuple_val.1);
    });

    res.into_inner().unwrap()
}
