/*
   independent samples t-tests between times taken and tweets per second rates of all algorithms
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use mathru::{
    self,
    statistics::{
        distrib::{Distribution, Normal},
        test::{Test, T},
    },
};

use crate::process_results::make_stats::STATS_OUTPUT_FILES_DIRECTORY;

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

    let rv1: Vec<f64> = Normal::new(1.0, 0.5).random_sequence(100);
    let rv2: Vec<f64> = Normal::new(1.0, 0.5).random_sequence(100);

    //Test with sample with identical means
    let mut measure: T<f64> = T::test_independence_unequal_variance(&rv1, &rv2);
    println!("{}", measure.value());
    measure = T::test_independence_equal_variance(&rv1, &rv2);
    println!("{}", measure.value());

    // Test with different equal mean, but unequal variances
    let rv3: Vec<f64> = Normal::new(1.0, 1.5).random_sequence(100);
    measure = T::test_independence_unequal_variance(&rv1, &rv3);
    println!("{}", measure.value());
    measure = T::test_independence_equal_variance(&rv1, &rv3);
    println!("{}", measure.value());

    // When the sample size is not equal anymore
    //the equal variance t-statistic is no longer equal to the unequal variance t-statistic:
    let rv4: Vec<f64> = Normal::new(2.0, 0.5).random_sequence(300);
    measure = T::test_independence_unequal_variance(&rv1, &rv4);
    println!("{}", measure.value());
    measure = T::test_independence_equal_variance(&rv1, &rv4);
    println!("{}", measure.value());

    //t-Test with different mean, variance and sample size
    let rv5: Vec<f64> = Normal::new(2.0, 1.0).random_sequence(300);
    measure = T::test_independence_unequal_variance(&rv1, &rv5);
    println!("{}", measure.value());
    measure = T::test_independence_equal_variance(&rv1, &rv5);
    println!("{}", measure.value());
}
