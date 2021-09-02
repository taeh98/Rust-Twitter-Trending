/*
   VISUALISATIONS

   bar chart of mean, median, and mode time taken and tweets per second for each algorithm
   box plots of each value of times taken and tweets per second rates for each algorithm
   dot plots of each value of times taken and tweets per second rates for each algorithm
   scatter plot of test number and time taken and test number and tweets/second for each algorithm
*/

use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use statrs::statistics::Distribution;
use statrs::statistics::{Data, OrderStatistics};

const CHART_WIDTH_PIXELS: isize = 1000;
const CHART_HEIGHT_PIXELS: isize = 750;

//TODO: implement this with plotters (?)
pub fn make_visualisations(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    make_bar_charts(algorithm_names, time_taken_values, processing_speed_values);
}

fn find_mean(values: &Vec<f64>) -> f64 {
    let mut clone = values.clone();
    let slice = clone.as_mut_slice();
    Data::new(slice).mean().unwrap()
}

fn find_median(values: &Vec<f64>) -> f64 {
    let mut clone = values.clone();
    let slice = clone.as_mut_slice();
    Data::new(slice).median()
}

fn find_mode(values: &Vec<f64>) -> f64 {
    //TODO: do this properly
    find_mean(values)
}

fn make_bar_charts(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    let mean_time_taken_values: Vec<f64> = time_taken_values
        .into_par_iter()
        .map(|values: &Vec<f64>| find_mean(values))
        .collect();
    let median_time_taken_values: Vec<f64> = time_taken_values
        .into_par_iter()
        .map(|values: &Vec<f64>| find_median(values))
        .collect();
    let mode_time_taken_values: Vec<f64> = time_taken_values
        .into_par_iter()
        .map(|values: &Vec<f64>| find_mode(values))
        .collect();

    gen_time_taken_bar_chart(algorithm_names, &mean_time_taken_values, "mean");

    let mean_processing_speed_values: Vec<f64> = processing_speed_values
        .into_par_iter()
        .map(|values: &Vec<f64>| find_mean(values))
        .collect();
    let median_processing_speed_values: Vec<f64> = processing_speed_values
        .into_par_iter()
        .map(|values: &Vec<f64>| find_median(values))
        .collect();
    let mode_processing_speed_values: Vec<f64> = processing_speed_values
        .into_par_iter()
        .map(|values: &Vec<f64>| find_mode(values))
        .collect();
}

fn gen_time_taken_bar_chart(
    category_names: &Vec<String>,
    values_in: &Vec<f64>,
    average_type: &str,
) {
    gen_bar_chart(
        category_names,
        values_in,
        format!(
            "./out/{}_time_taken_values.svg",
            average_type.to_lowercase()
        )
        .as_str(),
        format!(
            "{}{} time taken values for different algorithms",
            average_type.get(0..=0).unwrap().to_uppercase(),
            average_type.get(1..).unwrap()
        )
        .as_str(),
        "Time taken (seconds)",
        "Algorithm",
    );
}

fn gen_bar_chart(
    category_names: &Vec<String>,
    values_in: &Vec<f64>,
    filepath: &str,
    title: &str,
    y_axis_label: &str,
    x_axis_label: &str,
) {
    // Define chart related sizes.
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let values: Vec<f32> = values_in
        .into_par_iter()
        .map(|val: &f64| val.clone() as f32)
        .collect();

    assert_eq!(category_names.len(), (&values).len());

    let max_value: f32 = values
        .clone()
        .into_par_iter()
        .reduce_with(|a: f32, b: f32| if a > b { a } else { b })
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
        .set_domain(vec![0.0, (1.1 * max_value).round()])
        .set_range(vec![CHART_HEIGHT_PIXELS - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    let data: Vec<(String, f32)> = category_names
        .iter()
        .zip(values.clone().iter())
        .collect::<Vec<(&String, &f32)>>()
        .into_par_iter()
        .map(|val: (&String, &f32)| (val.0.clone(), val.1.clone()))
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
