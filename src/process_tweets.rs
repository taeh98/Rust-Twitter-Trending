use std::collections::HashMap;
use std::sync::Mutex;

use priority_queue::PriorityQueue;
use rayon::prelude::*;
use std::cmp::Ordering;

//TODO: replace PriorityQueue with std::collections::BinaryHeap
//TODO: abstract word-count pairs to a separate data type
//TODO: try to debug errors that came from using reduce() rather than reduce_with() throughout project
//TODO: integrate the use of "if let" throughout the project
//TODO: replace uses of unwrap() with expect() throughout the project
//TODO: check which pub functions really need to be

#[derive(Eq)]
pub(crate) struct WordAndCount {
    word: String,
    count: isize
}

impl WordAndCount {
    pub(crate) fn new(word: &str, count: isize) -> WordAndCount {
        WordAndCount {
            word: String::from(word),
            count: count.clone()
        }
    }
    pub(crate) fn get_word(&self) -> &String {
        &self.word
    }
    pub(crate) fn get_count(&self) -> isize {
        self.count
    }
    pub(crate) fn increment_count(&mut self) {
        self.count += 1;
    }
}

impl Ord for WordAndCount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for WordAndCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.count.cmp(&other.count))
    }
}

impl PartialEq for WordAndCount {
    fn eq(&self, other: &Self) -> bool {
        self.word.eq(other.get_word()) && self.count == other.count
    }
}

pub fn process_tweets(tweets: &Vec<String>, parallel: bool) -> PriorityQueue<String, i128> {
    if parallel {
        processed_tweets_to_priority_queue(
            tweets
                .par_iter()
                .map(|tweet: &String| process_tweet(tweet))
                .reduce_with(|a: HashMap<String, i128>, b: HashMap<String, i128>| {
                    combine_processed_tweets(&a, &b, parallel)
                })
                .unwrap(),
            parallel,
        )
    } else {
        processed_tweets_to_priority_queue(
            tweets
                .iter()
                .map(|tweet: &String| process_tweet(tweet))
                .reduce(|a: HashMap<String, i128>, b: HashMap<String, i128>| {
                    combine_processed_tweets(&a, &b, parallel)
                })
                .unwrap(),
            parallel,
        )
    }
}

fn process_tweet(tweet: &String) -> HashMap<String, i128> {
    let words: Vec<String> = tweet
        .clone()
        .split_whitespace()
        .into_iter()
        .map(|val: &str| String::from(val))
        .collect();
    let res: HashMap<String, i128> = HashMap::new();

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

fn get_hashmap_keys(a: &HashMap<String, i128>) -> Vec<String> {
    a.into_par_iter()
        .map(|value| value.0.clone())
        .collect::<Vec<String>>()
}

fn combine_processed_tweets(
    a: &HashMap<String, i128>,
    b: &HashMap<String, i128>,
    parallel: bool,
) -> HashMap<String, i128> {
    let keys: Vec<String> = get_hashmap_keys(a)
        .into_iter()
        .chain(get_hashmap_keys(b).into_iter())
        .collect();
    let res: Mutex<HashMap<String, i128>> = Mutex::new(HashMap::new());

    if parallel {
        keys.into_par_iter().for_each(|key: String| {
            let key_str = key.as_str();

            match b.get(key_str) {
                Some(b_count) => match a.get(key_str) {
                    Some(a_count) => {
                        res.lock()
                            .unwrap()
                            .insert(key, a_count.clone() + b_count.clone());
                    }
                    _ => {
                        res.lock().unwrap().insert(key, b_count.clone());
                    }
                },
                _ => {
                    match a.get(key_str) {
                        Some(val) => {
                            res.lock().unwrap().insert(key, val.clone());
                        }
                        _ => {}
                    };
                }
            }
        });
    } else {
        keys.into_iter().for_each(|key: String| {
            let key_str = key.as_str();

            match b.get(key_str) {
                Some(b_count) => match a.get(key_str) {
                    Some(a_count) => {
                        res.lock()
                            .unwrap()
                            .insert(key, a_count.clone() + b_count.clone());
                    }
                    _ => {
                        res.lock().unwrap().insert(key, b_count.clone());
                    }
                },
                _ => {
                    match a.get(key_str) {
                        Some(val) => {
                            res.lock().unwrap().insert(key, val.clone());
                        }
                        _ => {}
                    };
                }
            }
        });
    }

    res.into_inner().unwrap()
}

fn processed_tweets_to_priority_queue(
    pt: HashMap<String, i128>,
    parallel: bool,
) -> PriorityQueue<String, i128> {
    if parallel {
        let res: Mutex<PriorityQueue<String, i128>> = Mutex::new(PriorityQueue::new());

        pt.into_par_iter().for_each(|tuple_val: (String, i128)| {
            let mut q = res.lock().unwrap();
            q.push(tuple_val.0, tuple_val.1);
        });

        res.into_inner().unwrap()
    } else {
        let mut q = PriorityQueue::new();
        pt.into_iter().for_each(|tuple_val: (String, i128)| {
            q.push(tuple_val.0, tuple_val.1);
        });

        q
    }
}
