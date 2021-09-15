use std::sync::{Mutex, MutexGuard};

use polars::datatypes::AnyValue;
use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::CsvReader;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const DATA_FILE_PATHS: [&str; 18] = [
    "data/out-0.csv",
    "data/out-1.csv",
    "data/out-2.csv",
    "data/out-3.csv",
    "data/out-4.csv",
    "data/out-5.csv",
    "data/out-6.csv",
    "data/out-7.csv",
    "data/out-8.csv",
    "data/out-9.csv",
    "data/out-10.csv",
    "data/out-11.csv",
    "data/out-12.csv",
    "data/out-13.csv",
    "data/out-14.csv",
    "data/out-15.csv",
    "data/out-16.csv",
    "data/out-17.csv",
];

const NUM_DATA_FILES_TO_USE: usize = 3; // 1 to 18, aim for all 18

fn add_df_row_to_hash_map(values: Vec<AnyValue>, hm_mutex: &Mutex<Vec<String>>) {
    assert_eq!(values.len(), 1);

    let text: String = values.get(0).unwrap().to_string();

    let mut hm: MutexGuard<Vec<String>> = hm_mutex.lock().unwrap();
    hm.push(text);
}

fn process_dataframe(df: DataFrame, res: &Mutex<Vec<String>>, path: &str) {
    assert_eq!(df.width(), 1);

    let indices: Vec<usize> = (0..df.height()).collect();

    println!("Started processing the data from the dataset file {}", path);

    indices
        .into_par_iter()
        .map(|idx: usize| df.get(idx))
        .for_each(|values_option: Option<Vec<AnyValue>>| {
            if let Some(values) = values_option {
                add_df_row_to_hash_map(values, res);
            }
        });

    println!(
        "Finished processing the data from the dataset file {}",
        path
    );
}

fn get_tweets_from_filepath(path: &str, res: &Mutex<Vec<String>>) {
    println!("Reading in the data from the dataset file {}", path);
    if let Ok(file_data) = CsvReader::from_path(path) {
        if let Ok(df) = file_data.infer_schema(None).has_header(true).finish() {
            if let Ok(filtered_df) = df.select("text") {
                process_dataframe(filtered_df, res, path);
            }
        }
    }
}

pub fn get_tweets() -> Option<Vec<String>> {
    let res_mutex: Mutex<Vec<String>> = Mutex::new(Vec::new());

    assert!(NUM_DATA_FILES_TO_USE >= 1 as usize);
    assert!(NUM_DATA_FILES_TO_USE <= 18 as usize);

    DATA_FILE_PATHS[0..NUM_DATA_FILES_TO_USE]
        .into_par_iter()
        .for_each(|path: &&str| get_tweets_from_filepath(*path, &res_mutex));

    res_mutex.into_inner().ok()
}
