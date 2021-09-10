use std::fs::{create_dir, File};
use std::path::Path;

use const_format::concatcp;
use csv::{Writer, WriterBuilder};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::process_results::{algorithm_name_to_lowercase_underscored, OUTPUT_FILES_DIRECTORY};
use crate::{TimeTakenTweetProcessingSpeedValuePair, TweetProcessingResult};

const RAW_RESULTS_OUTPUT_FILES_DIRECTORY: &str =
    concatcp!(OUTPUT_FILES_DIRECTORY, "/results") as &str;
const CSV_HEADERS: [&str; 3] = [
    "Iteration number",
    "Time taken values (seconds)",
    "Tweet processing speed values (tweets/second)",
];

pub(crate) fn write_results_csv_files(results: &[TweetProcessingResult]) {
    if !Path::new(RAW_RESULTS_OUTPUT_FILES_DIRECTORY).exists() {
        create_dir(RAW_RESULTS_OUTPUT_FILES_DIRECTORY)
            .expect("Couldn't create the out/stats/t_tests/ directory.");
    }

    results
        .into_par_iter()
        .for_each(|res: &TweetProcessingResult| {
            write_results_csv(res.get_name(), res.get_time_taken_tweets_per_sec_values())
        });
}

fn write_results_csv(
    algorithm_name: &str,
    time_taken_values: &[TimeTakenTweetProcessingSpeedValuePair],
) {
    // write results to csv: iteration number, time taken values (seconds), tweet processing speed values (tweets/second)

    let file_path: String = format!(
        "{}/{}.csv",
        RAW_RESULTS_OUTPUT_FILES_DIRECTORY,
        algorithm_name_to_lowercase_underscored(algorithm_name)
    );

    let output_file: File = File::create(file_path).expect("could not create file");

    let mut csv_writer: Writer<File> = WriterBuilder::new().from_writer(output_file);

    csv_writer.write_record(&CSV_HEADERS).unwrap();
    csv_writer.flush().unwrap();

    time_taken_values.iter().enumerate().for_each(
        |(index, time_taken_processing_speed_value_pair)| {
            csv_writer
                .serialize((
                    index,
                    time_taken_processing_speed_value_pair.get_time_taken_seconds(),
                    time_taken_processing_speed_value_pair.get_processing_speed_tweets_per_second(),
                ))
                .unwrap();
            csv_writer.flush().unwrap();
        },
    );
}
