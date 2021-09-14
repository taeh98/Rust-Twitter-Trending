use std::fs::read_to_string;
use std::path::Path;

use md5::compute;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use verbose_file_download::{download_data_files, DataFileMetaData};

use crate::get_tweets::file_name_to_filepath;

mod verbose_file_download;

pub const DATA_FILES_INFO: [(&str, &str, &str); 3] = [
    (
        "full_who_dataset1.csv",
        "259389f2f6c1b232fe248c91107eeccd",
        "https://zenodo.org/record/3928240/files/full_who_dataset1.csv?download=1",
    ),
    (
        "full_who_dataset2.csv",
        "ea266ada5b1b817638ab89388138d95e",
        "https://zenodo.org/record/3928240/files/full_who_dataset2.csv?download=1",
    ),
    (
        "full_who_dataset3.csv",
        "fc4b898f8d7c81293a776bf116668bab",
        "https://zenodo.org/record/3928240/files/full_who_dataset3.csv?download=1",
    ),
];

pub fn check_or_get_tweets_data() {
    let data_files: Vec<DataFileMetaData> = DATA_FILES_INFO
        .iter()
        .map(|(filename, md5digest, url)| DataFileMetaData::new(*filename, *md5digest, *url))
        .collect();

    let dfs_to_get: Vec<DataFileMetaData> = data_files
        .into_par_iter()
        .filter(|df: &DataFileMetaData| {
            !check_file_is_present_and_intact(
                &file_name_to_filepath(df.get_file_name()),
                df.get_md5_digest(),
            )
        })
        .collect();

    download_data_files(&dfs_to_get);

    for df in dfs_to_get {
        let name: &str = df.get_file_name();
        if !check_file_is_present_and_intact(&file_name_to_filepath(name), df.get_md5_digest()) {
            panic!("The data file {} was not intact after downloading.", name);
        }
    }
}

fn check_file_is_present_and_intact(filepath: &str, expected_md5_digest: &str) -> bool {
    if Path::new(filepath).exists() {
        let actual_md5_digest: String = gen_file_md5_digest(filepath).unwrap();
        return actual_md5_digest.as_str().eq(expected_md5_digest);
    }
    false
}

fn file_to_string(filepath: &str) -> Option<String> {
    read_to_string(filepath).ok()
}

fn file_to_u8_vec(filepath: &str) -> Option<Vec<u8>> {
    file_to_string(filepath).map(|contents| Vec::from(contents.as_bytes()))
}

fn gen_file_md5_digest(filepath: &str) -> Option<String> {
    file_to_u8_vec(filepath).map(|bytes| format!("{:x}", compute(bytes)))
}
