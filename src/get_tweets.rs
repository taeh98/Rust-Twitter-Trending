use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use polars::datatypes::AnyValue;
use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::CsvReader;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::get_data::DATA_FILES_INFO;

const DATA_DIRECTORY_PATH: &str = "data";

fn add_df_row_to_hash_map(values: Vec<AnyValue>, hm_mutex: &Mutex<HashMap<String, String>>) {
    assert_eq!(values.len(), 2);

    let id: String = values.get(0).unwrap().to_string();
    let text: String = values.get(1).unwrap().to_string();

    let mut hm: MutexGuard<HashMap<String, String>> = hm_mutex.lock().unwrap();
    if !hm.contains_key(id.as_str()) {
        hm.insert(id, text);
    }
}

fn process_dataframe(df: DataFrame, res: &Mutex<HashMap<String, String>>, path: &str) {
    assert_eq!(df.width(), 2);

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

fn get_tweets_from_filepath(path: &str, res: &Mutex<HashMap<String, String>>) {
    println!("Reading in the data from the dataset file {}", path);
    if let Ok(file_data) = CsvReader::from_path(path) {
        if let Ok(df) = file_data.infer_schema(None).has_header(true).finish() {
            if let Ok(filtered_df) = df.select(("id_str", "text")) {
                process_dataframe(filtered_df, res, path);
            }
        }
    }
}

pub(crate) fn file_name_to_filepath(name: &str) -> String {
    format!("{}/{}", DATA_DIRECTORY_PATH, name)
}

pub fn get_tweets() -> Option<Vec<String>> {
    let res_mutex: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

    let data_file_paths: Vec<String> = DATA_FILES_INFO
        .iter()
        .map(|(filename, _md5digest, _url)| file_name_to_filepath(*filename))
        .collect();

    data_file_paths
        .into_par_iter()
        .for_each(|path: String| get_tweets_from_filepath(&path, &res_mutex));

    match res_mutex.into_inner().ok() {
        Some(res) => {
            if res.is_empty() {
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
