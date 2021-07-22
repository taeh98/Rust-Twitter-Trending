use std::ops::Deref;

use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::CsvReader;
use polars::series::Series;

fn process_dataframe(df: DataFrame) -> Vec<String> {
    println!("process_dataframe()");
    let column: &Series = df.column("text").unwrap();

    let mut res: Vec<String> = Vec::new();

    for i in 0..column.len() {
        let cow = column.str_value(i);
        let string = cow.to_string();
        res.push(string);
    }

    println!("process_dataframe(), res = {:#?}", res);

    res
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