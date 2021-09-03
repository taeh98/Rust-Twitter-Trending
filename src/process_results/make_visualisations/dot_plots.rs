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
    variable_to_string, Variable, CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};

const DOT_PLOTS_OUTPUT_FILES_DIRECTORY: &'static str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/dot_plots") as &'static str;

pub(crate) fn make_dot_plots(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
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
    .for_each(|var_values_pair: (Variable, &Vec<Vec<f64>>)| {
        gen_dot_plot(algorithm_names, var_values_pair.1, var_values_pair.0)
    })
}

fn gen_dot_plot(
    algorithm_names: &Vec<String>,
    algorithm_values: &Vec<Vec<f64>>,
    variable: Variable,
) {
    // Define chart related sizes.
    let width: isize = CHART_WIDTH_PIXELS;
    let height: isize = CHART_HEIGHT_PIXELS;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let max: f64 = algorithm_values
        .into_par_iter()
        .map(|values: &Vec<f64>| {
            let mut mut_values = values.clone();
            let data = Data::new(mut_values);
            return data.max();
        })
        .reduce_with(|a: f64, b: f64| if a > b { a } else { b })
        .unwrap();

    println!("max = {}", max);

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ])
        .set_range(vec![0, width - left - right]);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, 100.0])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `PointDatum` trait.
    let scatter_data = vec![
        (String::from("A"), 90.3),
        (String::from("B"), 20.1),
        (String::from("C"), 10.8),
    ];

    // Create Scatter view that is going to represent the data as points.
    let scatter_view = ScatterView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_position(PointLabelPosition::NE)
        .set_marker_type(MarkerType::Circle)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#FF4700"]))
        .load_data(&scatter_data)
        .unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Composite Bar + Scatter Chart"))
        .add_view(&scatter_view) // <-- add scatter view
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Units of Measurement")
        .add_bottom_axis_label("Categories")
        .save("composite-bar-and-scatter-chart.svg")
        .unwrap();
}
