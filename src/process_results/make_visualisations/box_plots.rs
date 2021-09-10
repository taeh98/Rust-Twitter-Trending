/*
   box plots of each value of times taken and tweets per second rates for each algorithm
*/
use std::fs::create_dir;
use std::path::Path;

use const_format::concatcp;
use plotters::coord::Shift;
use plotters::data::fitting_range;
use plotters::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::make_visualisations::{
    CHART_HEIGHT_PIXELS, CHART_WIDTH_PIXELS, OUTPUT_FILES_DIRECTORY,
};
use crate::process_results::{
    variable_to_axis_label, variable_to_lowercase_underscored_string, variable_to_string, Variable,
};

const BOX_PLOTS_OUTPUT_FILES_DIRECTORY: &str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/box_plots") as &str;

pub(crate) fn make_box_plots(
    algorithm_names: &[String],
    time_taken_values: &[Vec<f64>],
    processing_speed_values: &[Vec<f64>],
) {
    if !Path::new(BOX_PLOTS_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(BOX_PLOTS_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/visualisations/box_plots/ directory.");
    }

    [
        (Variable::TimeTaken, time_taken_values),
        (Variable::ProcessingSpeed, processing_speed_values),
    ]
    .into_par_iter()
    .for_each(|var_values_pair: (Variable, &[Vec<f64>])| {
        gen_box_plot(algorithm_names, var_values_pair.1, &var_values_pair.0)
    })
}

fn gen_box_plot(algorithm_names: &[String], algorithm_values: &[Vec<f64>], variable: &Variable) {
    let output_file_path: String = format!(
        "{}/{}.svg",
        BOX_PLOTS_OUTPUT_FILES_DIRECTORY,
        variable_to_lowercase_underscored_string(variable)
    );

    let root: DrawingArea<SVGBackend, Shift> = SVGBackend::new(
        output_file_path.as_str(),
        (CHART_WIDTH_PIXELS as u32, CHART_HEIGHT_PIXELS as u32),
    )
    .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let quartiles: Vec<Quartiles> = algorithm_values
        .into_par_iter()
        .map(|values: &Vec<f64>| Quartiles::new(values.as_slice()))
        .collect();

    let ab_axis: Vec<&str> = algorithm_names
        .into_par_iter()
        .map(|val: &String| val.as_str())
        .collect::<Vec<&str>>();

    let values_range = fitting_range(
        (&quartiles)
            .into_par_iter()
            .map(|quartiles: &Quartiles| quartiles.values().to_vec())
            .reduce_with(|a: Vec<f32>, b: Vec<f32>| {
                let mut mut_a: Vec<f32> = a.clone();
                let mut mut_b: Vec<f32> = b;
                mut_a.append(mut_b.as_mut());
                a
            })
            .unwrap()
            .as_slice(),
    );

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(
            format!("{} of different algorithms", variable_to_string(variable)),
            ("sans-serif", 20),
        )
        .build_cartesian_2d(
            ab_axis[..].into_segmented(),
            values_range.start - 10.0..values_range.end + 10.0,
        )
        .unwrap();

    let y_desc: String = variable_to_axis_label(variable);

    chart
        .configure_mesh()
        .x_desc("Algorithm")
        .y_desc(y_desc)
        .light_line_style(&WHITE)
        .draw()
        .unwrap();

    let str_algorithm_names: Vec<&str> = algorithm_names
        .into_par_iter()
        .map(|name: &String| name.as_str())
        .collect();

    let names_quartiles: Vec<(&&str, &Quartiles)> =
        str_algorithm_names.iter().zip(&quartiles).collect();

    chart
        .draw_series(
            names_quartiles
                .iter()
                .map(|n_q_pair: &(&&str, &Quartiles)| {
                    Boxplot::new_vertical(SegmentValue::CenterOf(n_q_pair.0), n_q_pair.1)
                }),
        )
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
}
