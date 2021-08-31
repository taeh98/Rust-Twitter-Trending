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
use std::io::copy;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use reqwest::blocking::{get, Response};

const DATA_DIRECTORY_PATH: &str = "data";

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

pub fn file_name_to_filepath(name: &str) -> String {
    format!("{}/{}", DATA_DIRECTORY_PATH, name)
}

pub fn download_data_files(dfs: &Vec<DataFileMetaData>) {
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
    let response: Response = get(df.get_url())
        .expect(format!("Failed to download the data file {}.", df.get_file_name()).as_str());

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

    let bytes: Vec<u8> = response
        .bytes()
        .expect(
            format!(
                "Failed to get the bytes content of the downloaded data file {}.",
                df.get_file_name()
            )
            .as_str(),
        )
        .to_vec();
    let mut current_dataset_file_contents: &[u8] = bytes.as_slice();

    let mut out = File::create(&file_path)
        .expect(format!("Failed to create the data file \"{}\".", &file_path).as_str());

    copy(&mut current_dataset_file_contents, &mut out).expect(
        format!(
            "Failed to copy content to the data file \"{}\".",
            &file_path
        )
        .as_str(),
    );

    println!("Finished downloading the data file {}.", df.get_file_name());
}
