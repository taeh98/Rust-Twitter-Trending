use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use std::collections::HashMap;
use std::sync::atomic::AtomicI64;
use std::sync::Mutex;

use rayon::prelude::*;

//TODO: try to debug errors that came from using reduce() rather than reduce_with() throughout project
//TODO: integrate the use of "if let" throughout the project
//TODO: replace uses of unwrap() with expect() throughout the project
//TODO: check which pub functions really need to be

#[derive(Eq, Clone)]
pub(crate) struct WordAndCount {
    word: String,
    count: i64,
}

impl WordAndCount {
    pub(crate) fn new(word: &str, count: i64) -> WordAndCount {
        WordAndCount {
            word: String::from(word),
            count: count.clone(),
        }
    }
    pub(crate) fn get_word(&self) -> &String {
        &self.word
    }
    pub(crate) fn get_count(&self) -> i64 {
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

pub(crate) fn process_tweets(tweets: &Vec<String>, parallel: bool) -> BinaryHeap<WordAndCount> {
    let res_hashmap: HashMap<String, WordAndCount> = if parallel {
        tweets
            .par_iter()
            .map(|tweet: &String| process_tweet(tweet))
            .reduce_with(
                |a: HashMap<String, WordAndCount>, b: HashMap<String, WordAndCount>| {
                    combine_processed_tweets(&a, &b, parallel)
                },
            )
            .unwrap()
    } else {
        tweets
            .iter()
            .map(|tweet: &String| process_tweet(tweet))
            .reduce(
                |a: HashMap<String, WordAndCount>, b: HashMap<String, WordAndCount>| {
                    combine_processed_tweets(&a, &b, parallel)
                },
            )
            .unwrap()
    };
    processed_tweets_to_priority_queue(res_hashmap)
}

fn process_tweet(tweet: &String) -> HashMap<String, WordAndCount> {
    let words: Vec<String> = tweet
        .clone()
        .split_whitespace()
        .into_iter()
        .map(|val: &str| String::from(val))
        .collect();
    let mut res: HashMap<String, WordAndCount> = HashMap::new();

    for word in words.clone() {
        let count: i64 = words
            .clone()
            .into_iter()
            .filter(|s: &String| s.clone() == word)
            .count() as i64;
        let word_str: &str = word.as_str();
        if !res.contains_key(word_str) {
            res.insert(word.clone(), WordAndCount::new(word_str, count));
        }
    }

    res
}

fn get_hashmap_keys(a: &HashMap<String, WordAndCount>) -> Vec<String> {
    a.into_par_iter()
        .map(|value| value.0.clone())
        .collect::<Vec<String>>()
}

fn combine_processed_tweets(
    a: &HashMap<String, WordAndCount>,
    b: &HashMap<String, WordAndCount>,
    parallel: bool,
) -> HashMap<String, WordAndCount> {
    let keys: Vec<String> = get_hashmap_keys(a)
        .into_iter()
        .chain(get_hashmap_keys(b).into_iter())
        .collect();
    let res: Mutex<HashMap<String, WordAndCount>> = Mutex::new(HashMap::new());

    let hms: [&HashMap<String, WordAndCount>; 2] = [a, b];

    if parallel {
        keys.into_par_iter().for_each(|key: String| {
            let key_str: &str = key.as_str();

            let total_count: AtomicI64 = AtomicI64::new(0);

            hms.into_par_iter()
                .for_each(|hm: &HashMap<String, WordAndCount>| match hm.get(key_str) {
                    Some(word_and_count) => {
                        total_count.fetch_add(
                            word_and_count.get_count() as i64,
                            std::sync::atomic::Ordering::SeqCst,
                        );
                    }
                    _ => {}
                });

            res.lock().unwrap().insert(
                key.clone(),
                WordAndCount::new(key_str, total_count.into_inner() as i64),
            );
        });
    } else {
        keys.into_iter().for_each(|key: String| {
            let key_str: &str = key.as_str();

            let mut total_count: i64 = 0;

            hms.into_iter()
                .for_each(
                    |&hm: &&HashMap<String, WordAndCount>| match hm.get(key_str) {
                        Some(word_and_count) => {
                            total_count += word_and_count.get_count();
                        }
                        _ => {}
                    },
                );

            res.lock()
                .unwrap()
                .insert(key.clone(), WordAndCount::new(key_str, total_count));
        });
    }

    res.into_inner().unwrap()
}

fn processed_tweets_to_priority_queue(
    pt: HashMap<String, WordAndCount>,
) -> BinaryHeap<WordAndCount> {
    // BinaryHeap::from(pt.values().collect::<Vec<&WordAndCount>>())
    //TODO: do this properly
    BinaryHeap::new()
}
