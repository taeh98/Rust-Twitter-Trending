/*
   dot plots of each value of times taken and tweets per second rates for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use plotters::data::fitting_range;
use plotters::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::make_visualisations::{
    variable_to_string, Variable, CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};

const SCATTER_PLOTS_OUTPUT_FILES_DIRECTORY: &'static str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/scatter_plots") as &'static str;

pub(crate) fn make_scatter_plots(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    if !Path::new(SCATTER_PLOTS_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(SCATTER_PLOTS_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/visualisations/scatter_plots/ directory.");
    }

    [
        (Variable::TimeTaken, time_taken_values),
        (Variable::ProcessingSpeed, processing_speed_values),
    ]
    .into_par_iter()
    .for_each(|var_values_pair: (Variable, &Vec<Vec<f64>>)| {
        gen_scatter_plot(algorithm_names, var_values_pair.1, var_values_pair.0)
    })
}

fn gen_scatter_plot(
    algorithm_names: &Vec<String>,
    algorithm_values: &Vec<Vec<f64>>,
    variable: Variable,
) {
}
