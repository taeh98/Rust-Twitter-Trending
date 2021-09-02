/*
   VISUALISATIONS

   bar chart of mean, median, and mode time taken and tweets per second for each algorithm
   box plots of each value of times taken and tweets per second rates for each algorithm
   dot plots of each value of times taken and tweets per second rates for each algorithm
   scatter plot of test number and time taken and test number and tweets/second for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

const CHART_WIDTH_PIXELS: isize = 1000;
const CHART_HEIGHT_PIXELS: isize = 750;
const OUTPUT_FILES_DIRECTORY: &'static str = "./out/visualisations";

mod bar_charts;

//TODO: implement this with plotters (?)
pub fn make_visualisations(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    if !Path::new(OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/visualisations/ directory.");
    }
    bar_charts::make_bar_charts(algorithm_names, time_taken_values, processing_speed_values);
}
