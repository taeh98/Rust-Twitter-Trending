/*
   min, max, mean, median, mode, std dev, variance, Q1, Q3, IQR of times taken and processing speeds for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::make_stats::STATS_OUTPUT_FILES_DIRECTORY;
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

fn gen_basic_values(algorithm_name: &String, values: &Vec<f64>, variable: &Variable) {
    println!(
        "gen_basic_values(). algorithm_name = {}. values = {:#?}. variable = {}.",
        algorithm_name,
        values,
        variable_to_string(variable)
    )
}
