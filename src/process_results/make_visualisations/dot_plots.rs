/*
   dot plots of each value of times taken and tweets per second rates for each algorithm
*/

use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use plotters::prelude::{
    BitMapBackend, BuildNestedCoord, ChartBuilder, Circle, Color, IntoDrawingArea, LineSeries,
    BLACK, RED, WHITE,
};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::make_visualisations::OUTPUT_FILES_DIRECTORY;
use crate::process_results::Variable;

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

const OUT_FILE_NAME: &'static str = "nested_coord.png";

fn gen_dot_plot(algorithm_names: &[String], algorithm_values: &[Vec<f64>], variable: &Variable) {
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Nested Coord", ("sans-serif", 50.0))
        .build_cartesian_2d(
            ["Linear", "Quadratic"].nested_coord(|_| 0.0..10.0),
            0.0..10.0,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    chart
        .draw_series(
            (0..10)
                .map(|x| x as f64 / 1.0)
                .map(|x| Circle::new(((&"Linear", x).into(), x), 2, BLACK.filled())),
        )
        .unwrap();

    chart
        .draw_series(
            (0..10)
                .map(|x| x as f64 / 1.0)
                .map(|x| Circle::new(((&"Quadratic", x).into(), x), 2, BLACK.filled())),
        )
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().unwrap();
}
