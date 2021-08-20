use std::fs::File;
use std::io::Write;
use std::path::Path;

use rayon::iter::IntoParallelIterator;

const NUMBER_TO_SHOW: usize = 10;

pub fn print_top_words_from_counts(counts: &PriorityQueue<String, i128>) {
    let top_hashtags: Vec<(String, i128)> = get_top_words(counts, true);
    let top_words: Vec<(String, i128)> = get_top_words(counts, false);
    print_top_words(top_words, top_hashtags);
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

fn print_top_words(top_words: Vec<(String, i128)>, top_hashtags: Vec<(String, i128)>) {
    let res = format!(
        "Top words:\r\n{}\r\n\r\nTop hashtags:\r\n{}",
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
