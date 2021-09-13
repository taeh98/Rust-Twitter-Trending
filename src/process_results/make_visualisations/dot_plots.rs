/*
   dot plots of each value of times taken and tweets per second rates for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use plotters::coord::types::{RangedCoordf64, RangedSlice};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::{
    BuildNestedCoord, Cartesian2d, ChartBuilder, ChartContext, Color, Cross, IntoDrawingArea,
    NestedRange, SVGBackend, BLACK, WHITE,
};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

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
        make_dot_plot(algorithm_names, var_values_pair.1, &var_values_pair.0)
    })
}

fn make_dot_plot(
    algorithm_names: &[String],
    algorithm_values_list: &[Vec<f64>],
    variable: &Variable,
) {
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

    gen_dot_plot(
        algorithm_names,
        algorithm_values_list,
        file_path.as_str(),
        title.as_str(),
        y_axis_label.as_str(),
        "Algorithm",
    );
}

fn gen_dot_plot(
    algorithm_names: &[String],
    algorithm_values_list: &[Vec<f64>],
    file_path: &str,
    title: &str,
    y_axis_label: &str,
    x_axis_label: &str,
) {
    let root: DrawingArea<SVGBackend, Shift> = SVGBackend::new(
        &file_path,
        (CHART_WIDTH_PIXELS as u32, CHART_HEIGHT_PIXELS as u32),
    )
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    gen_dot_plot_chart(
        algorithm_names,
        algorithm_values_list,
        &root,
        title,
        y_axis_label,
        x_axis_label,
    );

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().unwrap();
}

fn gen_dot_plot_chart(
    algorithm_names: &[String],
    algorithm_values_list: &[Vec<f64>],
    root: &DrawingArea<SVGBackend, Shift>,
    title: &str,
    y_axis_label: &str,
    x_axis_label: &str,
) {
    let mut chart: ChartContext<
        SVGBackend,
        Cartesian2d<NestedRange<RangedSlice<String>, RangedCoordf64>, RangedCoordf64>,
    > = ChartBuilder::on(root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(title, ("sans-serif", 30.0))
        .build_cartesian_2d(algorithm_names.nested_coord(|_| 0.0..10.0), 0.0..10.0)
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .axis_desc_style(("sans-serif", 15))
        .y_desc(y_axis_label)
        .x_desc(x_axis_label)
        .draw()
        .unwrap();

    for (algorithm_name, values) in algorithm_names.iter().zip(algorithm_values_list.iter()) {
        chart
            .draw_series(values.iter().map(|value: &f64| {
                Cross::new(((algorithm_name).into(), *value), 2, BLACK.filled())
            }))
            .unwrap();
    }
}
