use std::sync::Mutex;

use polars::frame::DataFrame;
use polars::io::SerReader;
use polars::prelude::CsvReader;
use polars::series::Series;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn process_dataframe(df: DataFrame) -> Vec<String> {
    println!("process_dataframe()");
    let column: &Series = df.column("text").unwrap();

    let res: Mutex<Vec<String>> = Mutex::new(Vec::new());

    let indices: Vec<usize> = vec![0, 1, 2];

    indices.into_par_iter()
        .map(|val: usize| column.str_value(val).to_string())
        .for_each(|str: String| {
            let mut v = res.lock().unwrap();
            v.push(str);
        });

    println!("process_dataframe(), res = {:#?}", res);

    res.into_inner().unwrap()
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