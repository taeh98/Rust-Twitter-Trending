/*
   from terminal commands:

   docker run curlimages/curl:7.78.0 -L <url_src> -o <filepath_dest> --name curl-<filepath_dest>
   docker wait curl-<filepath_dest>
   docker rm curl-<filepath_dest>


   or in code:

   https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
   https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
*/

use std::fs::File;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use reqwest::blocking::{get, Response};

use crate::get_tweets::file_name_to_filepath;

#[derive(Clone, Debug)]
pub struct DataFileMetaData<'a> {
    file_name: &'a str,
    md5_digest: &'a str,
    url: &'a str,
}

impl<'a> DataFileMetaData<'a> {
    pub fn new<'b>(file_name: &'b str, md5_digest: &'b str, url: &'b str) -> DataFileMetaData<'b> {
        DataFileMetaData {
            file_name,
            md5_digest,
            url,
        }
    }
    pub fn get_file_name(&self) -> &str {
        self.file_name
    }
    pub fn get_file_path(&self) -> String {
        file_name_to_filepath(self.get_file_name())
    }
    pub fn get_md5_digest(&self) -> &str {
        self.md5_digest
    }
    pub fn get_url(&self) -> &str {
        self.url
    }
}

pub fn download_data_files(dfs: &[DataFileMetaData]) {
    if dfs.is_empty() {
        return;
    }

    println!("Downloading data files.");

    dfs.into_par_iter()
        .for_each(|df: &DataFileMetaData| download_data_file(df));
}

fn download_data_file(df: &DataFileMetaData) {
    println!("Starting to download the data file {}.", df.get_file_name());

    let file_path: String = df.get_file_path();
    let mut response: Response = get(df.get_url())
        .unwrap_or_else(|_| panic!("Failed to download the data file {}.", df.get_file_name()));

    assert!(
        response.status().is_success(),
        "The request to download the data file {} failed.",
        df.get_file_name()
    );

    if response.headers().contains_key("content-md5") {
        let actual_digest = response
            .headers()
            .get("content-md5")
            .unwrap()
            .to_str()
            .unwrap();

        assert_eq!(actual_digest, df.get_md5_digest(),
                   "The request to download the data file {} gave an MD5 checksum digest of {} when the expected value was {}.",
                   df.get_file_name(),
                   actual_digest,
                   df.get_md5_digest()
        );
    }

    let mut out: File = File::create(&file_path).unwrap_or_else(|_| {
        panic!(
            "Couldn't create the output file to which to save the data file {}.",
            &file_path
        )
    });

    response.copy_to(&mut out).unwrap_or_else(|_| {
        panic!(
            "Failed to save the downloaded content of the data file {} to its output file.",
            &file_path
        )
    });

    println!("Finished downloading the data file {}.", df.get_file_name());
}
