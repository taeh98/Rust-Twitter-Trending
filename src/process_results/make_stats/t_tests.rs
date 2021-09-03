/*
   independent samples t-tests between times taken and tweets per second rates of all algorithms
*/

use std::fs::create_dir;
use std::path::Path;

const T_TESTS_OUTPUT_FILES_DIRECTORY: &'static str =
    concatcp!(STATS_OUTPUT_FILES_DIRECTORY, "/t_tests") as &'static str;

pub(crate) fn make_t_tests(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    if !Path::new(T_TESTS_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(T_TESTS_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/stats/t_tests/ directory.");
    }
}
