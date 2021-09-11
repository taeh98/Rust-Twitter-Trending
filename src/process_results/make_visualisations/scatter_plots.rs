/*
   scatter plot of test number and time taken and test number and tweets/second for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::{ChartBuilder, Color, Cross, IntoDrawingArea, SVGBackend, BLACK, WHITE};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::make_visualisations::{
    CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};
use crate::process_results::{
    algorithm_name_to_lowercase_underscored, find_max, variable_to_axis_label,
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

    let root: DrawingArea<SVGBackend, Shift> = SVGBackend::new(
        &file_path,
        (CHART_WIDTH_PIXELS as u32, CHART_HEIGHT_PIXELS as u32),
    )
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let random_points: Vec<(f64, f64)> = values
        .into_par_iter()
        .enumerate()
        .map(|(index, value)| (((index + 1) as f64), *value))
        .collect::<Vec<(f64, f64)>>();

    let areas = root.split_by_breakpoints([944], [80]);

    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(title, ("sans-serif", 20))
        .build_cartesian_2d(
            0f64..(values.len() as f64) * 1.2,
            0f64..find_max(values) * 1.2,
        )
        .unwrap();
    scatter_ctx
        .configure_mesh()
        .x_desc("Iteration number")
        .y_desc(y_axis_label)
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .unwrap();
    scatter_ctx
        .draw_series(
            random_points
                .iter()
                .map(|(x, y)| Cross::new((*x, *y), 2, BLACK.filled())),
        )
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().unwrap();
}
