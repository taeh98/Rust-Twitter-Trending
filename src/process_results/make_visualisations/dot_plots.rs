/*
   dot plots of each value of times taken and tweets per second rates for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::{
    ChartBuilder, Circle, Color, Histogram, IntoDrawingArea, IntoSegmentedCoord, SVGBackend, BLACK,
    WHITE,
};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::make_visualisations::{
    CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};
use crate::process_results::{
    find_max, variable_to_axis_label, variable_to_lowercase_underscored_string, variable_to_string,
    Variable,
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
    // x axis: "Algorithm"

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
