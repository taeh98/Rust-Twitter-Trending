/*
   independent samples t-tests between times taken and tweets per second rates of all algorithms
*/

use std::fs::create_dir;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use const_format::concatcp;
use mathru::{
    self,
    statistics::test::{Test, T},
};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::process_results::make_stats::STATS_OUTPUT_FILES_DIRECTORY;
use crate::process_results::{Variable, ALL_VARIABLE_VALUES};

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

    ALL_VARIABLE_VALUES
        .into_par_iter()
        .for_each(|variable: Variable| {
            let algorithm_names_variable_values: Vec<(&String, &Vec<f64>)> = algorithm_names
                .iter()
                .zip(
                    (match variable {
                        Variable::ProcessingSpeed => processing_speed_values,
                        _ => time_taken_values,
                    })
                    .iter(),
                )
                .collect();
            run_t_tests_for_variable(&variable, &algorithm_names_variable_values);
        });
}

fn run_t_tests_for_variable(
    variable: &Variable,
    algorithm_names_variable_values: &Vec<(&String, &Vec<f64>)>,
) {
    let res_mutex: Mutex<Vec<(&String, &String, f64, f64)>> = Mutex::new(Vec::new());

    let index_algorithm_names_variable_values: Vec<(usize, &(&String, &Vec<f64>))> =
        algorithm_names_variable_values
            .into_par_iter()
            .enumerate()
            .collect::<Vec<(usize, &(&String, &Vec<f64>))>>();

    index_algorithm_names_variable_values
        .into_par_iter()
        .for_each(|(index_a, (algorithm_name_a, values_a))| {
            let other_algorithm_names_variable_values: Vec<(&String, &Vec<f64>)> =
                algorithm_names_variable_values
                    .split_at(index_a + 1)
                    .1
                    .to_vec();

            other_algorithm_names_variable_values
                .into_par_iter()
                .for_each(|(algorithm_name_b, values_b)| {
                    let student_t_test: T<f64> =
                        T::test_independence_equal_variance(values_a, values_b);
                    let welch_t_test: T<f64> =
                        T::test_independence_unequal_variance(values_a, values_b);

                    let mut inner_results: MutexGuard<Vec<(&String, &String, f64, f64)>> =
                        res_mutex.lock().unwrap();

                    inner_results.push((
                        algorithm_name_a,
                        algorithm_name_b,
                        student_t_test.p_value(),
                        welch_t_test.p_value(),
                    ));
                });
        });

    let result: Vec<(&String, &String, f64, f64)> = res_mutex.into_inner().ok().unwrap();
}
