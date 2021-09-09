/*
   TODO: create source files implementing the below stats
   TODO: implement one-way ANOVA tests

   Wilcoxon Rank-Sum tests (aka Mann–Whitney U test) between times taken and tweets per second rates of all algorithms: non-parametric alternative to t-test
   one-way ANOVA tests between times taken and tweets per second rates of all algorithms
   Kruskal Wallis H Tests between times taken and tweets per second rates of all algorithms: non-parametric alternative to anova
   Chi-squared test to see how dependent the categorical variables (Rust or Python and serial or parallel) are
   G-test to see how dependent the categorical variables (Rust or Python and serial or parallel) are (?)
   Phi coefficient to see how associated the categorical variables (Rust or Python and serial or parallel) are
   F-test to compare the variances of the samples
   pearson and spearman correlation coefficients for test number vs time taken for all algorithms
   Skewness of time taken and processing speed values for all algorithms - Pearson’s first and second coefficients of skewness, Bowley’s method of measuring skewness, Pearson's moment coefficient of skewness, Groeneveld and Meeden's coefficient
   Kurtosis of time taken and processing speed values for all algorithms - kurtosis and excess kurtosis (aka alpha/beta kurtosis) from Pearson's 4th moment, percentile coefficient of kurtosis (aka quartile measure of kurtosis)
*/

use std::fs::create_dir;
use std::path::Path;

mod basic_values;
mod t_tests;

const STATS_OUTPUT_FILES_DIRECTORY: &str = "./out/stats";

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
