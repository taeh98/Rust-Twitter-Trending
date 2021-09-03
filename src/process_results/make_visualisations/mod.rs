use std::fs::create_dir;
use std::path::Path;

const CHART_WIDTH_PIXELS: isize = 1000;
const CHART_HEIGHT_PIXELS: isize = 750;
const OUTPUT_FILES_DIRECTORY: &'static str = "./out/visualisations";

mod bar_charts;
mod box_plots;
mod dot_plots;
mod scatter_plots;

enum Variable {
    TimeTaken,
    ProcessingSpeed,
}

fn variable_to_string(var: &Variable) -> String {
    match var {
        Variable::TimeTaken => String::from("Time taken"),
        _ => String::from("Processing speed"),
    }
}

fn variable_to_axis_label(var: &Variable) -> String {
    match var {
        Variable::TimeTaken => String::from("Time taken (seconds)"),
        _ => String::from("Processing speed (tweets/second)"),
    }
}

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
    box_plots::make_box_plots(algorithm_names, time_taken_values, processing_speed_values);
    dot_plots::make_dot_plots(algorithm_names, time_taken_values, processing_speed_values);
    scatter_plots::make_scatter_plots(algorithm_names, time_taken_values, processing_speed_values);
}
