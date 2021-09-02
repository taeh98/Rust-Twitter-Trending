/*
   box plots of each value of times taken and tweets per second rates for each algorithm
*/
use const_format::concatcp;

use crate::process_results::make_visualisations::OUTPUT_FILES_DIRECTORY;

const BOX_PLOTS_OUTPUT_FILES_DIRECTORY: &'static str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/box_plots") as &'static str;

pub(crate) fn make_box_plots(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
}
