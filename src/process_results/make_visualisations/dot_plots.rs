/*
   dot plots of each value of times taken and tweets per second rates for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use charts::{Chart, Color, MarkerType, PointLabelPosition, ScaleBand, ScaleLinear, ScatterView};
use const_format::concatcp;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use statrs::statistics::Data;
use statrs::statistics::Max;

use crate::process_results::make_visualisations::{
    CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};
use crate::process_results::{
    variable_to_axis_label, variable_to_lowercase_underscored_string, variable_to_string, Variable,
};

const DOT_PLOTS_OUTPUT_FILES_DIRECTORY: &str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/dot_plots") as &str;

pub(crate) fn make_dot_plots(
    algorithm_names: &[String],
    time_taken_values: &[Vec<f64>],
    processing_speed_values: &[Vec<f64>],
) {
    if !Path::new(DOT_PLOTS_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(DOT_PLOTS_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/visualisations/dot_plots/ directory.");
    }

    [
        (Variable::TimeTaken, time_taken_values),
        (Variable::ProcessingSpeed, processing_speed_values),
    ]
    .into_par_iter()
    .for_each(|var_values_pair: (Variable, &[Vec<f64>])| {
        gen_dot_plot(algorithm_names, var_values_pair.1, &var_values_pair.0)
    })
}

fn gen_dot_plot(algorithm_names: &[String], algorithm_values: &[Vec<f64>], variable: &Variable) {
    // Define chart related sizes.
    let width: isize = CHART_WIDTH_PIXELS;
    let height: isize = CHART_HEIGHT_PIXELS;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let max: f64 = algorithm_values
        .into_par_iter()
        .map(|values: &Vec<f64>| {
            let mut_values: Vec<f64> = values.clone();
            let data: Data<Vec<f64>> = Data::new(mut_values);
            data.max()
        })
        .reduce_with(|a: f64, b: f64| if a > b { a } else { b })
        .unwrap();

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x: ScaleBand = ScaleBand::new()
        .set_domain(algorithm_names.to_vec())
        .set_range(vec![0, width - left - right]);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y: ScaleLinear = ScaleLinear::new()
        .set_domain(vec![0.0, (max * 1.1).round() as f32])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `PointDatum` trait.
    let mut scatter_data: Vec<(String, f32)> = Vec::new();

    for (algorithm_name, algorithm_values) in algorithm_names.iter().zip(algorithm_values.iter()) {
        for value in algorithm_values {
            scatter_data.push((algorithm_name.clone(), *value as f32));
        }
    }

    // Create Scatter view that is going to represent the data as points.
    let scatter_view = ScatterView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_position(PointLabelPosition::NE)
        .set_marker_type(MarkerType::Circle)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#FF4700"]))
        .load_data(&scatter_data)
        .unwrap();

    let file_path: String = format!(
        "{}/{}.svg",
        DOT_PLOTS_OUTPUT_FILES_DIRECTORY,
        variable_to_lowercase_underscored_string(variable)
    );

    let y_axis_label: String = variable_to_axis_label(variable);
    let title: String = format!(
        "{} values of different algorithms",
        variable_to_string(variable)
    );

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(title)
        .add_view(&scatter_view) // <-- add scatter view
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label(y_axis_label.as_str())
        .add_bottom_axis_label("Algorithms")
        .save(file_path.as_str())
        .unwrap();
}
