/*
  bar chart of mean, median, and mode time taken and tweets per second for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::{
    ChartBuilder, Color, Histogram, IntoDrawingArea, IntoSegmentedCoord, SVGBackend, BLACK, WHITE,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::process_results::make_visualisations::{
    CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};
use crate::process_results::{
    find_max, find_mean, find_median, find_mode, variable_to_axis_label, Variable,
};

//TODO: add greater margins between axis labels and axis values

#[derive(Clone, Copy, PartialEq)]
enum Average {
    Mean,
    Median,
    Mode,
}

fn average_to_string(av: Average) -> String {
    match av {
        Average::Mean => String::from("mean"),
        Average::Median => String::from("median"),
        _ => String::from("mode"),
    }
}

const ALL_AVERAGES: [Average; 3] = [Average::Mean, Average::Median, Average::Mode];
const BAR_CHART_OUTPUT_FILES_DIRECTORY: &str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/bar_charts") as &str;

fn gen_value_lists_averages(values_list: &[Vec<f64>], average_type: Average) -> Option<Vec<f64>> {
    if values_list.is_empty() {
        return None;
    }

    if average_type == Average::Mean || average_type == Average::Mode {
        return Some(
            values_list
                .into_par_iter()
                .map(|values: &Vec<f64>| match average_type {
                    Average::Mean => find_mean(values),
                    _ => find_median(values),
                })
                .collect(),
        );
    }

    let averages: Vec<f64> = values_list
        .into_par_iter()
        .map(|values: &Vec<f64>| find_mode(values))
        .filter(|value: &Option<f64>| value.is_some())
        .map(|value: Option<f64>| value.unwrap())
        .collect();

    if averages.len() == values_list.len() {
        Some(averages)
    } else {
        None
    }
}

pub(crate) fn make_bar_charts(
    algorithm_names: &[String],
    time_taken_values: &[Vec<f64>],
    processing_speed_values: &[Vec<f64>],
) {
    if !Path::new(BAR_CHART_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(BAR_CHART_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/visualisations/ directory.");
    }

    [
        (Variable::TimeTaken, time_taken_values),
        (Variable::ProcessingSpeed, processing_speed_values),
    ]
    .into_par_iter()
    .for_each(|var_name_val_pair: (Variable, &[Vec<f64>])| {
        ALL_AVERAGES
            .into_par_iter()
            .for_each(|average_type: Average| {
                let values_option: Option<Vec<f64>> =
                    gen_value_lists_averages(var_name_val_pair.1, average_type);
                if let Some(values_vec) = values_option {
                    match var_name_val_pair.0 {
                        Variable::ProcessingSpeed => gen_processing_speed_bar_chart(
                            algorithm_names,
                            &values_vec,
                            average_type,
                        ),
                        _ => gen_time_taken_bar_chart(algorithm_names, &values_vec, average_type),
                    }
                }
            });
    });
}

fn gen_time_taken_bar_chart(algorithm_names: &[String], values_in: &[f64], average_type: Average) {
    let average_string: String = average_to_string(average_type);
    gen_bar_chart(
        algorithm_names,
        values_in,
        format!(
            "{}/{}_time_taken_values.svg",
            BAR_CHART_OUTPUT_FILES_DIRECTORY,
            average_string.to_lowercase()
        )
        .as_str(),
        format!(
            "{}{} time taken values for different algorithms",
            average_string.get(0..=0).unwrap().to_uppercase(),
            average_string.get(1..).unwrap().to_lowercase()
        )
        .as_str(),
        variable_to_axis_label(&Variable::TimeTaken).as_str(),
        "Algorithm",
    );
}

fn gen_processing_speed_bar_chart(
    algorithm_names: &[String],
    values_in: &[f64],
    average_type: Average,
) {
    let average_string: String = average_to_string(average_type);
    gen_bar_chart(
        algorithm_names,
        values_in,
        format!(
            "{}/{}_processing_speed_values.svg",
            BAR_CHART_OUTPUT_FILES_DIRECTORY,
            average_string.to_lowercase()
        )
        .as_str(),
        format!(
            "{}{} tweet processing speeds for different algorithms",
            average_string.get(0..=0).unwrap().to_uppercase(),
            average_string.get(1..).unwrap().to_lowercase()
        )
        .as_str(),
        "Processing speed (tweets/second)",
        "Algorithm",
    );
}

fn gen_bar_chart(
    category_names: &[String],
    values_in: &[f64],
    filepath: &str,
    title: &str,
    y_axis_label: &str,
    x_axis_label: &str,
) {
    let root: DrawingArea<SVGBackend, Shift> = SVGBackend::new(
        filepath,
        (CHART_WIDTH_PIXELS as u32, CHART_HEIGHT_PIXELS as u32),
    )
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(title, ("sans-serif", 30.0))
        .build_cartesian_2d(
            category_names.into_segmented(),
            0f64..(1.2 * find_max(values_in)),
        )
        .unwrap();

    chart
        .configure_mesh()
        // .disable_x_mesh()
        // .bold_line_style(&WHITE.mix(0.3))
        .y_desc(y_axis_label)
        .x_desc(x_axis_label)
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    chart
        .draw_series(
            Histogram::vertical(&chart).style(BLACK.filled()).data(
                values_in
                    .iter()
                    .zip(category_names.iter())
                    .map(|(value, category)| (category, *value)),
            ),
        )
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().unwrap();
}
