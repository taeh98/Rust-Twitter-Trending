/*
   box plots of each value of times taken and tweets per second rates for each algorithm
*/
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, prelude::*, BufReader};

use const_format::concatcp;
use plotters::data::fitting_range;
use plotters::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::make_visualisations::{
    variable_to_string, Variable, CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};

const BOX_PLOTS_OUTPUT_FILES_DIRECTORY: &'static str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/box_plots") as &'static str;
const OUT_FILE_NAME: &'static str = "boxplot.svg";

pub(crate) fn make_box_plots(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    processing_speed_values: &Vec<Vec<f64>>,
) {
    [
        (Variable::TimeTaken, time_taken_values),
        (Variable::ProcessingSpeed, processing_speed_values),
    ]
    .into_par_iter()
    .for_each(|var_values_pair: (Variable, &Vec<Vec<f64>>)| {
        gen_box_plot(algorithm_names, var_values_pair.1, var_values_pair.0)
    })
}

fn gen_box_plot(
    algorithm_names: &Vec<String>,
    time_taken_values: &Vec<Vec<f64>>,
    variable: Variable,
) {
    let root = SVGBackend::new(
        OUT_FILE_NAME,
        (CHART_WIDTH_PIXELS as u32, CHART_HEIGHT_PIXELS as u32),
    )
    .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let quartiles_a = Quartiles::new(&[
        6.0, 7.0, 15.9, 36.9, 39.0, 40.0, 41.0, 42.0, 43.0, 47.0, 49.0,
    ]);
    let quartiles_b = Quartiles::new(&[16.0, 17.0, 50.0, 60.0, 40.2, 41.3, 42.7, 43.3, 47.0]);

    let ab_axis: Vec<&str> = algorithm_names
        .into_par_iter()
        .map(|val: &String| val.as_str())
        .collect::<Vec<&str>>();

    let values_range = fitting_range(
        quartiles_a
            .values()
            .iter()
            .chain(quartiles_b.values().iter()),
    );
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(variable_to_string(variable), ("sans-serif", 20))
        .build_cartesian_2d(
            ab_axis[..].into_segmented(),
            values_range.start - 10.0..values_range.end + 10.0,
        )
        .unwrap();

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()
        .unwrap();
    chart
        .draw_series(vec![
            Boxplot::new_vertical(SegmentValue::CenterOf(&"a"), &quartiles_a),
            Boxplot::new_vertical(SegmentValue::CenterOf(&"b"), &quartiles_b),
        ])
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
}
