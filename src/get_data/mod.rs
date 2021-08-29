use std::fs::read_to_string;
use std::path::Path;

use md5::compute;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use verbose_file_download::{download_data_files, name_to_filepath, DataFileMetaData};

mod verbose_file_download;

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

    let dfs_to_get: Vec<DataFileMetaData> = data_files
        .into_par_iter()
        .filter(|df: &DataFileMetaData| {
            !check_file_is_present_and_intact(&name_to_filepath(df.name), df.md5_digest)
        })
        .collect();

    download_data_files(&dfs_to_get);

    for df in dfs_to_get {
        if !check_file_is_present_and_intact(&name_to_filepath(df.name), df.md5_digest) {
            panic!(
                "The data file {} was not intact after downloading.",
                df.name
            );
        }
    }
}

fn check_file_is_present_and_intact(filepath: &String, expected_md5_digest: &str) -> bool {
    if Path::new(filepath.as_str()).exists() {
        let actual_md5_digest: String = gen_file_md5_digest(&filepath).unwrap();
        return actual_md5_digest.as_str().eq(expected_md5_digest);
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
