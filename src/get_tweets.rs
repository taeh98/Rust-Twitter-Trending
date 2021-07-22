use std::ops::Deref;

use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::CsvReader;
use polars::series::Series;

fn process_dataframe(df: DataFrame) -> Vec<String> {
    println!("process_dataframe()");
    let column: &Series = df.column("text").unwrap();
    let first = column.chunks().first().unwrap();

    println!("first = {:#?}", &first);

    println!("first.deref().deref().data() = {:#?}", first.deref().deref().data());

    Vec::new()
}

pub fn get_tweets() -> Option<Vec<String>> {
    println!("get_tweets()");
    return match CsvReader::from_path("data/covid19_tweets.csv") {
        Ok(file_data) => {
            match file_data.infer_schema(None)
                .has_header(true)
                .finish() {
                Ok(df) => {
                    println!("get_tweets(), going to process_dataframe()");
                    Some(process_dataframe(df))
                }
                _ => {
                    None
                }
            }
        }
        _ => None
    };
}