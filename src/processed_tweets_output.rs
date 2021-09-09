use std::collections::binary_heap::BinaryHeap;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

use rayon::prelude::*;

use crate::process_tweets::WordAndCount;

const NUMBER_TO_SHOW: usize = 10;
const TOP_WORDS_HASHTAGS_OUTPUT_FILEPATH: &str = "out/top_words_hashtags.txt";

pub(crate) fn get_top_words_text_from_counts(counts: &BinaryHeap<WordAndCount>) -> String {
    let top_hashtags: BinaryHeap<WordAndCount> = get_top_words(counts, true);
    let top_words: BinaryHeap<WordAndCount> = get_top_words(counts, false);
    get_top_words_text(top_words, top_hashtags)
}

pub(crate) fn print_top_words_text_from_counts(counts: &BinaryHeap<WordAndCount>) {
    let top_hashtags: BinaryHeap<WordAndCount> = get_top_words(counts, true);
    let top_words: BinaryHeap<WordAndCount> = get_top_words(counts, false);
    print_top_words_text(top_words, top_hashtags)
}

fn get_top_words(
    counts_in: &BinaryHeap<WordAndCount>,
    hashtag_not_word: bool,
) -> BinaryHeap<WordAndCount> {
    let mut res: BinaryHeap<WordAndCount> = BinaryHeap::new();
    let mut counts: BinaryHeap<WordAndCount> = counts_in.clone();

    while res.len() < NUMBER_TO_SHOW && (!counts.is_empty()) {
        let current_wc: WordAndCount = counts.pop().unwrap();
        if current_wc.get_word().starts_with("#") == hashtag_not_word {
            res.push(current_wc.clone());
        }
    }

    res
}

fn get_top_words_text(
    top_words: BinaryHeap<WordAndCount>,
    top_hashtags: BinaryHeap<WordAndCount>,
) -> String {
    format!(
        "Top words:\r\n{}\r\n\r\nTop hashtags:\r\n{}",
        top_word_list_to_string(top_words),
        top_word_list_to_string(top_hashtags)
    )
}

fn print_top_words_text(
    top_words: BinaryHeap<WordAndCount>,
    top_hashtags: BinaryHeap<WordAndCount>,
) {
    let text: String = get_top_words_text(top_words, top_hashtags);

    println!("{}", text);

    if !Path::new("out/").exists() {
        create_dir("out").expect("Couldn't create the out/ directory.");
    }

    let path = Path::new(TOP_WORDS_HASHTAGS_OUTPUT_FILEPATH);
    let mut file = File::create(&path).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}

fn top_word_list_to_string(list: BinaryHeap<WordAndCount>) -> String {
    list.into_par_iter()
        .map(|val: WordAndCount| format!("{} {}", val.get_word(), val.get_count()))
        .reduce_with(|a: String, b: String| format!("{}\r\n{}", a, b))
        .unwrap()
}
