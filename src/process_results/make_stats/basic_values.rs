use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::make_stats::STATS_OUTPUT_FILES_DIRECTORY;
use crate::process_results::make_visualisations::Variable;

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

    [
        (Variable::TimeTaken, time_taken_values),
        (Variable::ProcessingSpeed, processing_speed_values),
    ]
    .into_par_iter()
    .for_each(|var_values_pair: (Variable, &Vec<Vec<f64>>)| {
        gen_basic_values(algorithm_names, var_values_pair.1, &var_values_pair.0)
    })
}

fn gen_basic_values(
    algorithm_names: &Vec<String>,
    algorithm_values: &Vec<Vec<f64>>,
    variable: &Variable,
) {
}
