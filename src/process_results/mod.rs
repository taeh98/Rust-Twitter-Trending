use std::collections::HashMap;
use std::fs::create_dir;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use statrs::statistics::{Data, Distribution, OrderStatistics};

use crate::{TimeTakenTweetProcessingSpeedValuePair, TweetProcessingResult};

mod make_stats;
mod make_visualisations;
mod write_results;

const OUTPUT_FILES_DIRECTORY: &str = "./out";
const RAW_RESULTS_FILE_NAME: &str = "results.csv";

fn gen_algorithm_names(algorithm_results: &[TweetProcessingResult]) -> Vec<String> {
    algorithm_results
        .into_par_iter()
        .map(|res: &TweetProcessingResult| res.get_name().clone())
        .collect()
}

fn gen_time_taken_or_processing_speed_values(
    algorithm_results: &[TweetProcessingResult],
    time_taken_not_processing_speed: bool,
) -> Vec<Vec<f64>> {
    algorithm_results
        .into_par_iter()
        .map(|res: &TweetProcessingResult| {
            res.get_time_taken_tweets_per_sec_values()
                .into_par_iter()
                .map(|value_pair: &TimeTakenTweetProcessingSpeedValuePair| {
                    if time_taken_not_processing_speed {
                        value_pair.get_time_taken_seconds()
                    } else {
                        value_pair.get_processing_speed_tweets_per_second()
                    }
                })
                .collect()
        })
        .collect()
}

pub fn process_results(algorithm_results: Vec<TweetProcessingResult>) {
    if !Path::new(OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(OUTPUT_FILES_DIRECTORY).expect("Couldn't create the out/ directory.");
    }

    let algorithm_names: Vec<String> = gen_algorithm_names(&algorithm_results);
    let time_taken_values: Vec<Vec<f64>> =
        gen_time_taken_or_processing_speed_values(&algorithm_results, true);
    let processing_speed_values: Vec<Vec<f64>> =
        gen_time_taken_or_processing_speed_values(&algorithm_results, false);

    assert_eq!(algorithm_names.len(), time_taken_values.len());
    assert_eq!(processing_speed_values.len(), time_taken_values.len());

    write_results::write_results_csv(&algorithm_results);
    make_visualisations::make_visualisations(
        &algorithm_names,
        &time_taken_values,
        &processing_speed_values,
    );
    make_stats::make_stats(
        &algorithm_names,
        &time_taken_values,
        &processing_speed_values,
    );
}

pub(crate) enum Variable {
    TimeTaken,
    ProcessingSpeed,
}

const ALL_VARIABLE_VALUES: [Variable; 2] = [Variable::TimeTaken, Variable::ProcessingSpeed];

pub(crate) fn variable_to_string(var: &Variable) -> String {
    match var {
        Variable::TimeTaken => String::from("Time taken"),
        _ => String::from("Processing speed"),
    }
}

pub(crate) fn variable_to_lowercase_underscored_string(var: &Variable) -> String {
    match var {
        Variable::TimeTaken => String::from("time_taken"),
        _ => String::from("processing_speed"),
    }
}

pub(crate) fn variable_to_axis_label(var: &Variable) -> String {
    match var {
        Variable::TimeTaken => String::from("Time taken (seconds)"),
        _ => String::from("Processing speed (tweets/second)"),
    }
}

pub(crate) fn find_mean(values: &[f64]) -> f64 {
    let mut clone: Vec<f64> = values.to_vec();
    let slice: &mut [f64] = clone.as_mut_slice();
    Data::new(slice).mean().unwrap()
}

pub(crate) fn find_median(values: &[f64]) -> f64 {
    let mut clone: Vec<f64> = values.to_vec();
    let slice: &mut [f64] = clone.as_mut_slice();
    Data::new(slice).median()
}

pub(crate) fn find_mode(values: &[f64]) -> Option<f64> {
    let counts_map_mutex: Mutex<HashMap<u64, i32>> = Mutex::new(HashMap::new());

    values.into_par_iter().for_each(|&value: &f64| {
        let mut counts_map_mutex_guard: MutexGuard<HashMap<u64, i32>> =
            counts_map_mutex.lock().unwrap();
        let count: &mut i32 = counts_map_mutex_guard.entry(value.to_bits()).or_insert(0);
        *count += 1;
    });

    let counts_map: HashMap<u64, i32> = counts_map_mutex.into_inner().unwrap();

    let max_count: i32 = counts_map.values().cloned().max().unwrap_or(0);

    match max_count <= 0 {
        true => None,
        _ => {
            let values: Vec<f64> = counts_map
                .into_iter()
                .filter(|&(_, count)| count == max_count)
                .map(|(value, _)| f64::from_bits(value))
                .collect();
            Some(find_median(&values))
        }
    }
}

fn algorithm_name_to_lowercase_underscored(algorithm_name: &str) -> String {
    algorithm_name.to_lowercase().replace(" ", "_")
}
