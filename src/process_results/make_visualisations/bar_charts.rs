/*
  bar chart of mean, median, and mode time taken and tweets per second for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};
use const_format::concatcp;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::process_results::make_visualisations::{
    CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};
use crate::process_results::{find_mean, find_median, find_mode, variable_to_axis_label, Variable};

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
    // Define chart related sizes.
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let values: Vec<f32> = values_in
        .into_par_iter()
        .map(|val: &f64| *val as f32)
        .collect();

    assert_eq!(category_names.len(), (&values).len());

    let max_value: f32 = values
        .clone()
        .into_par_iter()
        .reduce_with(|a: f32, b: f32| if a > b { a } else { b })
        .unwrap();

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(category_names.to_vec())
        .set_range(vec![0, CHART_WIDTH_PIXELS - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, (1.1 * max_value).round()])
        .set_range(vec![CHART_HEIGHT_PIXELS - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    let data: Vec<(String, f32)> = category_names
        .iter()
        .zip(values.iter())
        .collect::<Vec<(&String, &f32)>>()
        .into_par_iter()
        .map(|val: (&String, &f32)| (val.0.clone(), *val.1))
        .collect();

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .load_data(&data)
        .unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(CHART_WIDTH_PIXELS)
        .set_height(CHART_HEIGHT_PIXELS)
        .set_margins(top, right, bottom, left)
        .add_title(String::from(title))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label(y_axis_label)
        .add_bottom_axis_label(x_axis_label)
        .save(filepath)
        .unwrap();
}
