use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use polars::datatypes::AnyValue;
use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::CsvReader;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const DATAFILE_PATHS: [&str; 3] = [
    "data/full_who_dataset1.csv",
    "data/full_who_dataset2.csv",
    "data/full_who_dataset3.csv",
];

fn add_df_row_to_hash_map(values: Vec<AnyValue>, hm_mutex: &Mutex<HashMap<String, String>>) {
    assert_eq!(values.len(), 2);

    let id: String = values.get(0).unwrap().to_string();
    let text: String = values.get(1).unwrap().to_string();

    let mut hm: MutexGuard<HashMap<String, String>> = hm_mutex.lock().unwrap();
    if !hm.contains_key(id.as_str()) {
        hm.insert(id.clone(), text);
    }
}

fn process_dataframe(df: DataFrame, res: &Mutex<HashMap<String, String>>, path: &str) {
    assert_eq!(df.width(), 2);

    let indices: Vec<usize> = (0..df.height()).collect();

    indices
        .into_par_iter()
        .map(|idx: usize| df.get(idx))
        .for_each(|values_option: Option<Vec<AnyValue>>| {
            if let Some(values) = values_option {
                add_df_row_to_hash_map(values, res);
            }
        });
}

fn get_tweets_from_filepath(path: &str, res: &Mutex<HashMap<String, String>>) {
    match CsvReader::from_path(path) {
        Ok(file_data) => match file_data.infer_schema(None).has_header(true).finish() {
            Ok(df) => match df.select(("id_str", "text")).ok() {
                Some(filtered_df) => process_dataframe(filtered_df, res, path),
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}

pub fn get_tweets() -> Option<Vec<String>> {
    let res_mutex: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    DATAFILE_PATHS
        .to_vec()
        .into_par_iter()
        .for_each(|path: &str| get_tweets_from_filepath(path, &res_mutex));

    match res_mutex.into_inner().ok() {
        Some(res) => {
            if res.len() < 1 {
                return None;
            }

            let tweets: Vec<&String> = res.values().collect::<Vec<&String>>();
            Some(
                tweets
                    .into_par_iter()
                    .map(|str: &String| str.clone())
                    .collect::<Vec<String>>(),
            )
        }
        _ => None,
    }
}
