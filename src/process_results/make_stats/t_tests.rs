/*
   independent samples t-tests between times taken and tweets per second rates of all algorithms
*/

use std::fs::{create_dir, File};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use const_format::concatcp;
use mathru::{
    self,
    statistics::test::{Test, T},
};
use polars::frame::DataFrame;
use polars::io::csv::CsvWriter;
use polars::io::SerWriter;
use polars::series::{NamedFrom, Series};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::process_results::make_stats::STATS_OUTPUT_FILES_DIRECTORY;
use crate::process_results::{
    variable_to_lowercase_underscored_string, Variable, ALL_VARIABLE_VALUES,
};

const T_TESTS_OUTPUT_FILES_DIRECTORY: &str =
    concatcp!(STATS_OUTPUT_FILES_DIRECTORY, "/t_tests") as &str;

//TODO: check that t test results are mathematically correct - seems weird that 1 t-test shows 0% of diff being due to chance and other only ~25% in my-out

pub(crate) fn make_t_tests(
    algorithm_names: &[String],
    time_taken_values: &[Vec<f64>],
    processing_speed_values: &[Vec<f64>],
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
    algorithm_names_variable_values: &[(&String, &Vec<f64>)],
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

    let results: Vec<(&String, &String, f64, f64)> = res_mutex.into_inner().ok().unwrap();
    print_t_test_results(results, variable);
}

fn print_t_test_results(results: Vec<(&String, &String, f64, f64)>, variable: &Variable) {
    let algorithm_a_names: Mutex<Vec<&String>> = Mutex::new(Vec::new());
    let algorithm_b_names: Mutex<Vec<&String>> = Mutex::new(Vec::new());
    let student_t_test_values: Mutex<Vec<f64>> = Mutex::new(Vec::new());
    let welch_t_test_values: Mutex<Vec<f64>> = Mutex::new(Vec::new());

    results
        .into_par_iter()
        .for_each(|value: (&String, &String, f64, f64)| {
            algorithm_a_names.lock().unwrap().push(value.0);
            algorithm_b_names.lock().unwrap().push(value.1);
            student_t_test_values.lock().unwrap().push(value.2);
            welch_t_test_values.lock().unwrap().push(value.3);
        });

    let first_algorithm_series: Series = Series::new(
        "Name of first algorithm",
        algorithm_a_names
            .into_inner()
            .ok()
            .unwrap()
            .into_par_iter()
            .map(|val: &String| val.clone())
            .collect::<Vec<String>>(),
    );
    let second_algorithm_series: Series = Series::new(
        "Name of second algorithm",
        algorithm_b_names
            .into_inner()
            .ok()
            .unwrap()
            .into_par_iter()
            .map(|val: &String| val.clone())
            .collect::<Vec<String>>(),
    );
    let student_t_test_p_value_series: Series = Series::new(
        "P-value of Student's t-test",
        student_t_test_values.into_inner().ok().unwrap(),
    );
    let welch_t_test_p_value_series: Series = Series::new(
        "P-value of Welch's t-test",
        welch_t_test_values.into_inner().ok().unwrap(),
    );

    let df: DataFrame = DataFrame::new(vec![
        first_algorithm_series,
        second_algorithm_series,
        student_t_test_p_value_series,
        welch_t_test_p_value_series,
    ])
    .expect("Failed to generate a dataframe.");

    let filepath: String = format!(
        "{}/{}.csv",
        T_TESTS_OUTPUT_FILES_DIRECTORY,
        variable_to_lowercase_underscored_string(variable)
    );

    let mut output_file: File = File::create(filepath).expect("Failed to create an output file.");

    let writer: CsvWriter<File> = CsvWriter::new(&mut output_file).has_headers(true);

    writer.finish(&df).expect("Failed to write the CSV file.");
}
