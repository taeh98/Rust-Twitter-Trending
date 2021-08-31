use std::fs::read_to_string;
use std::path::Path;

use md5::compute;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use verbose_file_download::{download_data_files, file_name_to_filepath, DataFileMetaData};

mod verbose_file_download;

pub fn check_or_get_tweets_data() {
    let data_files: [DataFileMetaData; 3] = [
        DataFileMetaData::new(
            "full_who_dataset1.csv",
            "259389f2f6c1b232fe248c91107eeccd",
            "https://zenodo.org/record/3928240/files/full_who_dataset1.csv?download=1",
        ),
        DataFileMetaData::new(
            "full_who_dataset2.csv",
            "ea266ada5b1b817638ab89388138d95e",
            "https://zenodo.org/record/3928240/files/full_who_dataset2.csv?download=1",
        ),
        DataFileMetaData::new(
            "full_who_dataset3.csv",
            "fc4b898f8d7c81293a776bf116668bab",
            "https://zenodo.org/record/3928240/files/full_who_dataset3.csv?download=1",
        ),
    ];

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
