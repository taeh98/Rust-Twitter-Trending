/*
   min, max, mean, median, mode, std dev, variance, Q1, Q3, IQR of times taken and processing speeds for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use statrs::statistics::{Data, Max, Min};
use statrs::statistics::{OrderStatistics, Statistics};

use crate::process_results::make_stats::STATS_OUTPUT_FILES_DIRECTORY;
use crate::process_results::make_visualisations::bar_charts::{find_mean, find_median, find_mode};
use crate::process_results::make_visualisations::{variable_to_string, Variable};

const BASIC_VALUES_OUTPUT_FILES_DIRECTORY: &'static str =
    concatcp!(STATS_OUTPUT_FILES_DIRECTORY, "/basic_values") as &'static str;

pub(crate) fn make_basic_values(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    if !Path::new(STATS_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(STATS_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/stats/basic_values/ directory.");
    }

    let combined_values: Vec<(&String, (&Vec<f64>, &Vec<f64>))> = algorithm_names
        .iter()
        .zip(time_taken_values.iter().zip(processing_speed_values.iter()))
        .collect();

    let time_taken: Variable = Variable::TimeTaken;
    let processing_speed: Variable = Variable::ProcessingSpeed;

    combined_values.into_par_iter().for_each(
        |(algorithm_name, (time_taken_values, processing_speed_values))| {
            gen_basic_values(algorithm_name, time_taken_values, &time_taken);
            gen_basic_values(algorithm_name, processing_speed_values, &processing_speed);
        },
    );
}

fn find_max(values: &Vec<f64>) -> f64 {
    values.max()
}

fn find_min(values: &Vec<f64>) -> f64 {
    values.min()
}

fn find_std_dev(values: &Vec<f64>) -> f64 {
    values.std_dev()
}

fn find_variance(values: &Vec<f64>) -> f64 {
    values.variance()
}

fn find_q1(values: &Vec<f64>) -> f64 {
    let mut clone: Vec<f64> = values.clone();
    let slice: &mut [f64] = clone.as_mut_slice();
    let mut data: Data<&mut [f64]> = Data::new(slice);
    data.lower_quartile()
}

fn find_q3(values: &Vec<f64>) -> f64 {
    let mut clone: Vec<f64> = values.clone();
    let slice: &mut [f64] = clone.as_mut_slice();
    let mut data: Data<&mut [f64]> = Data::new(slice);
    data.upper_quartile()
}

fn find_iqr(values: &Vec<f64>) -> f64 {
    let mut clone: Vec<f64> = values.clone();
    let slice: &mut [f64] = clone.as_mut_slice();
    let mut data: Data<&mut [f64]> = Data::new(slice);
    data.interquartile_range()
}

fn gen_basic_values(algorithm_name: &String, values: &Vec<f64>, variable: &Variable) {
    let mean: f64 = find_mean(values);
    let median: f64 = find_median(values);
    let mode: f64 = find_mode(values);
    let min: f64 = find_min(values);
    let max: f64 = find_max(values);
    let std_dev: f64 = find_std_dev(values);
    let variance: f64 = find_variance(values);
    let q1: f64 = find_q1(values);
    let q3: f64 = find_q3(values);
    let iqr: f64 = find_q3(values);
}
