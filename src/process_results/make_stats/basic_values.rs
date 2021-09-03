use const_format::concatcp;

use crate::process_results::make_stats::STATS_OUTPUT_FILES_DIRECTORY;

const BASIC_VALUES_OUTPUT_FILES_DIRECTORY: &'static str =
    concatcp!(STATS_OUTPUT_FILES_DIRECTORY, "/basic_values") as &'static str;

pub(crate) fn make_basic_values(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
}
