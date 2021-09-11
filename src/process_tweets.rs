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
//TODO: give dot plots, scatter plots, and bar charts white (not transparent) backgrounds
//TODO: make all plots making "dots" use crosses rather than circles for better precision

#[derive(Eq, Clone)]
pub(crate) struct WordAndCount {
    word: String,
    count: i64,
}

impl WordAndCount {
    pub(crate) fn new(word: &str, count_in: i64) -> WordAndCount {
        WordAndCount {
            word: String::from(word),
            count: count_in,
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

pub(crate) fn process_tweets(tweets: &[String], parallel: bool) -> BinaryHeap<WordAndCount> {
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
    processed_tweets_to_priority_queue(res_hashmap, parallel)
}

fn process_tweet(tweet: &str) -> HashMap<String, WordAndCount> {
    let words: Vec<String> = tweet
        .split_whitespace()
        .into_iter()
        .map(String::from)
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

fn get_hashmap_keys(a: &HashMap<String, WordAndCount>, parallel: bool) -> Vec<String> {
    return if parallel {
        a.into_par_iter()
            .map(|value| value.0.clone())
            .collect::<Vec<String>>()
    } else {
        a.iter()
            .map(|value| value.0.clone())
            .collect::<Vec<String>>()
    };
}

fn combine_processed_tweets(
    a: &HashMap<String, WordAndCount>,
    b: &HashMap<String, WordAndCount>,
    parallel: bool,
) -> HashMap<String, WordAndCount> {
    let keys: Vec<String> = get_hashmap_keys(a, parallel)
        .into_iter()
        .chain(get_hashmap_keys(b, parallel).into_iter())
        .collect();
    let hms: [&HashMap<String, WordAndCount>; 2] = [a, b];

    if parallel {
        let res: Mutex<HashMap<String, WordAndCount>> = Mutex::new(HashMap::new());
        keys.into_par_iter().for_each(|key: String| {
            let key_str: &str = key.as_str();

            let total_count: AtomicI64 = AtomicI64::new(0);

            hms.into_par_iter()
                .for_each(|hm: &HashMap<String, WordAndCount>| {
                    if let Some(word_and_count) = hm.get(key_str) {
                        total_count.fetch_add(
                            word_and_count.get_count() as i64,
                            std::sync::atomic::Ordering::SeqCst,
                        );
                    }
                });

            res.lock().unwrap().insert(
                key.clone(),
                WordAndCount::new(key_str, total_count.into_inner() as i64),
            );
        });

        res.into_inner().unwrap()
    } else {
        let mut res: HashMap<String, WordAndCount> = HashMap::new();

        keys.into_iter().for_each(|key: String| {
            let key_str: &str = key.as_str();

            let mut total_count: i64 = 0;

            hms.iter().for_each(|&hm: &&HashMap<String, WordAndCount>| {
                if let Some(word_and_count) = hm.get(key_str) {
                    total_count += word_and_count.get_count();
                }
            });

            res.insert(key.clone(), WordAndCount::new(key_str, total_count));
        });

        res
    }
}

fn processed_tweets_to_priority_queue(
    pt: HashMap<String, WordAndCount>,
    parallel: bool,
) -> BinaryHeap<WordAndCount> {
    return if parallel {
        let res_mutex: Mutex<BinaryHeap<WordAndCount>> = Mutex::new(BinaryHeap::new());
        pt.into_par_iter().for_each(|(_, word_and_count)| {
            res_mutex.lock().unwrap().push(WordAndCount::new(
                word_and_count.get_word().as_str(),
                word_and_count.get_count(),
            ));
        });
        res_mutex.into_inner().unwrap()
    } else {
        let mut res: BinaryHeap<WordAndCount> = BinaryHeap::new();
        pt.iter().for_each(|(_, word_and_count)| {
            res.push(WordAndCount::new(
                word_and_count.get_word().as_str(),
                word_and_count.get_count(),
            ));
        });
        res
    };
}
