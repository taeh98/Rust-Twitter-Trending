/*
   STATS (indented are non-parametric alternatives to above parametric tests)

       Wilcoxon Rank-Sum tests between times taken and tweets per second rates of all algorithms
   one-way anova tests between times taken and tweets per second rates of all algorithms
       Kruskal Wallis H Tests between times taken and tweets per second rates of all algorithms
   Chi-squared test to see how dependent the categorical variables (Rust or Python and serial or parallel) are
   G-test to see how dependent the categorical variables (Rust or Python and serial or parallel) are (?)
   Phi coefficient to see how associated the categorical variables (Rust or Python and serial or parallel) are
   F-test to compare the variances of the samples
   pearson and spearman correlation coefficients for test number vs time taken for all algorithms
   Skewness of values of time taken and processing speed values for all algorithms - Pearson’s first and second coefficients of skewness, Bowley’s method of measuring skewness, Pearson's moment coefficient of skewness, Groeneveld and Meeden's coefficient
*/

use std::fs::create_dir;
use std::path::Path;

mod basic_values;
mod t_tests;

const STATS_OUTPUT_FILES_DIRECTORY: &'static str = "./out/stats";

//TODO: implement this with statrs (?)
pub fn make_stats(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    if !Path::new(STATS_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(STATS_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/stats/ directory.");
    }
    basic_values::make_basic_values(algorithm_names, time_taken_values, processing_speed_values);
    t_tests::make_t_tests(algorithm_names, time_taken_values, processing_speed_values);
}
