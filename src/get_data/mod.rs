use std::fs::{create_dir, read_to_string, remove_dir_all};
use std::path::Path;

use md5::compute;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::blocking::get;
use scraper::html::Select;
use scraper::node::Attrs;
use scraper::{ElementRef, Html, Selector};

mod verbose_file_download;

#[derive(Clone, Debug)]
struct DataFileMetaData<'a> {
    pub name: &'a str,
    pub md5_digest: &'a str,
    pub uri: &'a str,
}

const DATA_DIRECTORY_PATH: &str = "data";

pub fn check_or_get_tweets_data() {
    let data_files: [DataFileMetaData; 3] = [
        DataFileMetaData {
            name: "full_who_dataset1.csv",
            md5_digest: "259389f2f6c1b232fe248c91107eeccd",
            uri: "https://zenodo.org/record/3928240/files/full_who_dataset1.csv?download=1",
        },
        DataFileMetaData {
            name: "full_who_dataset2.csv",
            md5_digest: "ea266ada5b1b817638ab89388138d95e",
            uri: "https://zenodo.org/record/3928240/files/full_who_dataset2.csv?download=1",
        },
        DataFileMetaData {
            name: "full_who_dataset3.csv",
            md5_digest: "fc4b898f8d7c81293a776bf116668bab",
            uri: "https://zenodo.org/record/3928240/files/full_who_dataset3.csv?download=1",
        },
    ];

    data_files
        .to_vec()
        .into_par_iter()
        .for_each(|df: DataFileMetaData| check_or_get_data_file(df));
}

fn name_to_filepath(name: &str) -> String {
    format!("{}/{}", DATA_DIRECTORY_PATH, name)
}

fn check_or_get_data_file(df: DataFileMetaData) {
    let filepath: String = name_to_filepath(df.name);
    let mut path: &Path = Path::new(filepath.as_str());

    if path.exists() {
        if check_file_integrity(&filepath, df.md5_digest) {
            println!("The data file {} was already present and intact.", df.name);
            return;
        }
    }

    verbose_file_download::download_file_with_progress(df.uri, filepath.as_str());
    path = Path::new(filepath.as_str());

    if path.exists() {
        if check_file_integrity(&filepath, df.md5_digest) {
            println!("The data file {} was downloaded successfully.", df.name);
            return;
        } else {
            panic!(
                "The data file {} was not intact after downloading.",
                df.name
            );
        }
    } else {
        panic!("The data file {} could not be downloaded.", df.name);
    }
}

fn check_file_integrity(filepath: &String, expected_md5_digest: &str) -> bool {
    if Path::new(filepath.as_str()).exists() {
        let actual_md5_digest: String = gen_file_md5_digest(&filepath).unwrap();
        actual_md5_digest.as_str().eq(expected_md5_digest)
    }
    false
}

fn file_to_string(filepath: &String) -> Option<String> {
    read_to_string(filepath).ok()
}

fn file_to_u8_vec(filepath: &String) -> Option<Vec<u8>> {
    match file_to_string(filepath) {
        Some(contents) => Some(Vec::from(contents.as_bytes())),
        _ => None,
    }
}

fn gen_file_md5_digest(filepath: &String) -> Option<String> {
    match file_to_u8_vec(filepath) {
        Some(bytes) => Some(format!("{:x}", compute(bytes))),
        _ => None,
    }
}
