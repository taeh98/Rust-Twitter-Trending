/*
   VISUALISATIONS

   bar chart of mean, median, and mode time taken and tweets per second for each algorithm
   box plots of each value of times taken and tweets per second rates for each algorithm
   dot plots of each value of times taken and tweets per second rates for each algorithm
   scatter plot of test number and time taken and test number and tweets/second for each algorithm
*/

use std::cmp::max;

use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelSliceMut;

use crate::TweetProcessingResult;

const CHART_WIDTH_PIXELS: isize = 2000;
const CHART_HEIGHT_PIXELS: isize = 1500;

//TODO: implement this with plotters (?)
pub fn make_visualisations(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    make_bar_charts(algorithm_names, time_taken_values, processing_speed_values);
}

fn make_bar_charts(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    println!("make_visualisations()");
}

fn gen_bar_chart(category_names: &Vec<String>, values: &Vec<f64>) {
    // Define chart related sizes.
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let max_value: f64 = values
        .into_par_iter()
        .reduce_with(|a: &f64, b: &f64| if a > b { a } else { b })
        .unwrap()
        .clone();

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(category_names.clone())
        .set_range(vec![0, CHART_WIDTH_PIXELS - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, 100.0])
        .set_range(vec![CHART_HEIGHT_PIXELS - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    let data = vec![("A", 90), ("B", 10), ("C", 30)];

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
        .add_title(String::from("Bar Chart"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Units of Measurement")
        .add_bottom_axis_label("Categories")
        .save("vertical-bar-chart.svg")
        .unwrap();
}
