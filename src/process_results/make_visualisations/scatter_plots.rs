/*
   scatter plot of test number and time taken and test number and tweets/second for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;

use crate::process_results::make_visualisations::{Variable, OUTPUT_FILES_DIRECTORY};

const SCATTER_PLOTS_OUTPUT_FILES_DIRECTORY: &'static str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/scatter_plots") as &'static str;

pub(crate) fn make_scatter_plots(
    algorithm_names: &Vec<String>,
    time_taken_values_list: &Vec<Vec<f64>>,
    processing_speed_values_list: &Vec<Vec<f64>>,
) {
    if !Path::new(SCATTER_PLOTS_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(SCATTER_PLOTS_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/visualisations/scatter_plots/ directory.");
    }

    algorithm_names
        .iter()
        .zip(time_taken_values_list.iter())
        .zip(processing_speed_values_list.iter())
        .for_each(
            |((algorithm_name, time_taken_values), processing_speed_values)| {
                gen_scatter_plot(algorithm_name, time_taken_values, Variable::TimeTaken);
                gen_scatter_plot(
                    algorithm_name,
                    processing_speed_values,
                    Variable::ProcessingSpeed,
                );
            },
        );
}

fn gen_scatter_plot(algorithm_name: &String, values: &Vec<f64>, variable: Variable) {}
