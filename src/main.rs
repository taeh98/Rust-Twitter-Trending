use std::fs::File;
use std::io::Write;
use std::path::Path;

use priority_queue::PriorityQueue;
use rayon::prelude::*;

mod get_tweets;
mod process_tweets;

const NUMBER_TO_SHOW: usize = 10;

fn main() {
    match get_tweets::get_tweets() {
        Some(tweets) => {
            let num_tweets = tweets.len();

            let counts: PriorityQueue<String, i128> = process_tweets::process_tweets(tweets);
            let top_hashtags: Vec<(String, i128)> = get_top_words(&counts, true);
            let top_words: Vec<(String, i128)> = get_top_words(&counts, false);
            print_top_words(top_words, top_hashtags, num_tweets);
        }
        _ => println!("Couldn't get tweets data."),
    }
}

fn get_top_words(
    counts_in: &PriorityQueue<String, i128>,
    hashtag_not_word: bool,
) -> Vec<(String, i128)> {
    let mut res: Vec<(String, i128)> = Vec::new();
    let mut counts: PriorityQueue<String, i128> = counts_in.clone();

    while res.len() < NUMBER_TO_SHOW && (!counts.is_empty()) {
        let current_tc = counts.pop().unwrap();
        if current_tc.0.starts_with("#") == hashtag_not_word {
            res.push((current_tc.0.clone(), current_tc.1.clone()));
        }
    }

    res
}

fn print_top_words(
    top_words: Vec<(String, i128)>,
    top_hashtags: Vec<(String, i128)>,
    num_tweets: usize,
) {
    let res = format!(
        "Processed {} tweets.\r\n\r\nTop words:\r\n{}\r\n\r\nTop hashtags:\r\n{}",
        num_tweets,
        top_word_list_to_string(top_words),
        top_word_list_to_string(top_hashtags)
    );

    println!("{}", res);

    let path = Path::new("out.txt");
    let mut file = File::create(&path).unwrap();
    file.write_all(res.as_bytes()).unwrap();
}

fn top_word_list_to_string(list: Vec<(String, i128)>) -> String {
    list.into_par_iter()
        .map(|val: (String, i128)| format!("{} {}", val.0, val.1))
        .reduce_with(|a: String, b: String| format!("{}\r\n{}", a, b))
        .unwrap()
}
