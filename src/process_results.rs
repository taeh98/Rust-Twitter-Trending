use std::fs::{create_dir, File};
use std::path::Path;

use polars::frame::DataFrame;
use polars::io::SerWriter;
use polars::prelude::CsvWriter;
use polars::prelude::NamedFrom;
use polars::series::Series;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::{TimeTakenTweetProcessingSpeedValuePair, TweetProcessingResult};

const OUTPUT_FILES_DIRECTORY: &str = "./out";
const RAW_RESULTS_FILE_NAME: &str = "results.csv";

pub fn process_results(algorithm_results: Vec<TweetProcessingResult>) {
    //TODO: write the results to a csv (use polars), make stats measurements (use statrs), and make visualisations of them (use plotters)
    //TODO: figure out how to do this analysis in an environment like a Jupyter notebook - use Evcxr with Jupyter? Or Google Colab?
    write_results_csv(&algorithm_results);

    /*

    VISUALISATIONS
    bar chart of mean time taken and tweets per second for each algorithm
    box plots of each value of times taken and tweets per second rates for each algorithm
    dot plots of each value of times taken and tweets per second rates for each algorithm
    scatter plot of test number and time taken and test number and tweets/second for each algorithm

    STATS (indented are non-parametric alternatives to above parametric tests)
    mean, median, mode, std dev, variance, IQR of times taken and processing speeds for each algorithm
    independent samples t-tests between times taken and tweets per second rates of all algorithms
        Wilcoxon Rank-Sum tests between times taken and tweets per second rates of all algorithms
    one-way anova tests between times taken and tweets per second rates of all algorithms
        Kruskal Wallis H Tests between times taken and tweets per second rates of all algorithms
    Chi-squared test to see how dependent the categorical variables (Rust or Python and serial or parallel) are
    Phi coefficient to see how associated the categorical variables (Rust or Python and serial or parallel) are
    F-test to compare the variances of the samples
    pearson and spearman correlation coefficients for test number vs time taken for all algorithms

    */
}

fn gen_time_taken_or_processing_speed_series(
    time_taken: bool,
    results: &Vec<TweetProcessingResult>,
) -> Series {
    let column_title: &str = if time_taken {
        "Time taken values (seconds)"
    } else {
        "Tweet processing speed values (tweets/second)"
    };

    let values: Vec<Vec<f64>> = results.into_par_iter()
        .map(|val: &TweetProcessingResult|
            val.get_time_taken_tweets_per_sec_values()
                .into_par_iter()
                .map(|time_taken_tweets_per_sec_value_pair: &TimeTakenTweetProcessingSpeedValuePair|
                    if time_taken { time_taken_tweets_per_sec_value_pair.get_processing_speed_tweets_per_second() } else { time_taken_tweets_per_sec_value_pair.get_time_taken_seconds() })
                .collect()
        )
        .collect();

    let series_vec: Vec<Series> = values
        .into_par_iter()
        .map(|values: Vec<f64>| Series::new("a", values))
        .collect();

    Series::new(column_title, series_vec)
}

fn write_results_csv(results: &Vec<TweetProcessingResult>) {
    // write results to csv: algorithm name, time taken values (seconds), tweet processing speed values (tweets/second)
    let algorithm_names: Vec<String> = results
        .into_par_iter()
        .map(|res: &TweetProcessingResult| res.get_name().clone())
        .collect();
    let algorithm_names_series: Series = Series::new("Algorithm name", algorithm_names);

    let time_taken_values_series: Series = gen_time_taken_or_processing_speed_series(true, results);
    let processing_speed_values_series: Series =
        gen_time_taken_or_processing_speed_series(false, results);

    let df: DataFrame = DataFrame::new(vec![algorithm_names_series, time_taken_values_series, processing_speed_values_series])
        .expect("Failed to generate a dataframe to save the results in write_results_csv() in process_results.rs.");

    if !Path::new(OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(OUTPUT_FILES_DIRECTORY).expect("Couldn't create the out/ directory.");
    }

    let file_path: String = format!("{}/{}", OUTPUT_FILES_DIRECTORY, RAW_RESULTS_FILE_NAME);
    let mut output_file: File = File::create(file_path).expect("could not create file");

    CsvWriter::new(&mut output_file)
        .has_headers(true)
        .finish(&df)
        .expect("Failed to write the CSV file of raw results in write_results_csv() in process_results.rs.");
}
