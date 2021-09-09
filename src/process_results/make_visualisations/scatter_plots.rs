/*
   scatter plot of test number and time taken and test number and tweets/second for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use charts::{Chart, MarkerType, PointLabelPosition, ScaleLinear, ScatterView};
use const_format::concatcp;

use crate::process_results::make_visualisations::{
    CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};
use crate::process_results::{
    algorithm_name_to_lowercase_underscored, variable_to_axis_label,
    variable_to_lowercase_underscored_string, variable_to_string, Variable,
};

const SCATTER_PLOTS_OUTPUT_FILES_DIRECTORY: &str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/scatter_plots") as &str;

pub(crate) fn make_scatter_plots(
    algorithm_names: &[String],
    time_taken_values_list: &[Vec<f64>],
    processing_speed_values_list: &[Vec<f64>],
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
                gen_scatter_plot(algorithm_name, time_taken_values, &Variable::TimeTaken);
                gen_scatter_plot(
                    algorithm_name,
                    processing_speed_values,
                    &Variable::ProcessingSpeed,
                );
            },
        );
}

fn gen_scatter_plot(algorithm_name: &str, values: &[f64], variable: &Variable) {
    // Define chart related sizes.
    let width: isize = CHART_WIDTH_PIXELS;
    let height: isize = CHART_HEIGHT_PIXELS;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that will interpolate values in [0, 200] to values in the
    // [0, availableWidth] range (the width of the chart without the margins).
    let x: ScaleLinear = ScaleLinear::new()
        .set_domain(vec![0.0, 200.0])
        .set_range(vec![0, width - left - right]);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y: ScaleLinear = ScaleLinear::new()
        .set_domain(vec![0.0, 100.0])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `PointDatum` trait.
    let scatter_data: Vec<(f32, f32)> = values
        .iter()
        .enumerate()
        .map(|(index, value)| (index as f32, *value as f32))
        .collect();

    // Create Scatter view that is going to represent the data as points.
    let scatter_view = ScatterView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_position(PointLabelPosition::E)
        .set_marker_type(MarkerType::Circle)
        .load_data(&scatter_data)
        .unwrap();

    let file_path: String = format!(
        "{}/{}_{}.svg",
        SCATTER_PLOTS_OUTPUT_FILES_DIRECTORY,
        variable_to_lowercase_underscored_string(variable),
        algorithm_name_to_lowercase_underscored(algorithm_name)
    );

    let y_axis_label: String = variable_to_axis_label(variable);
    let title: String = format!(
        "{} values of the {} algorithm in each iteration",
        variable_to_string(variable),
        algorithm_name
    );

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(title)
        .add_view(&scatter_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label(y_axis_label)
        .add_bottom_axis_label("Iteration number")
        .save(file_path)
        .unwrap();
}
