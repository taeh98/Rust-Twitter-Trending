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

mod make_stats;
mod make_visualisations;

const OUTPUT_FILES_DIRECTORY: &str = "./out";
const RAW_RESULTS_FILE_NAME: &str = "results.csv";

pub fn process_results(algorithm_results: Vec<TweetProcessingResult>) {
    if !Path::new(OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(OUTPUT_FILES_DIRECTORY).expect("Couldn't create the out/ directory.");
    }

    write_results_csv(&algorithm_results);
    make_visualisations::make_visualisations(&algorithm_results);
    make_stats::make_stats(&algorithm_results);
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
        .expect("Failed to generate a dataframe to save the results in write_results_csv() in the process_results module");

    let file_path: String = format!("{}/{}", OUTPUT_FILES_DIRECTORY, RAW_RESULTS_FILE_NAME);
    let mut output_file: File = File::create(file_path).expect("could not create file");

    CsvWriter::new(&mut output_file)
        .has_headers(true)
        .finish(&df)
        .expect("Failed to write the CSV file of raw results in write_results_csv() in the process_results module");
}
