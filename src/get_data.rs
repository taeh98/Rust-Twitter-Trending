/**

1. go to https://doi.org/10.5281/zenodo.3723939 for latest version of https://github.com/thepanacealab/covid19_twitter
2. find full_dataset_clean.tsv.gz element and current date or version
3. if latest version already downloaded then exit
4. get download link and its md5 checksum digest
5. download the dataset
6. verify integrity against the md5 digest
7. extract the .tsv.gz file to a .tsv file
8. delete the .tsv.gz file

 */
extern crate reqwest;

use std::fs::File;
use std::io;

const CURRENT_VERSION_URL: &str = "https://doi.org/10.5281/zenodo.3723939";

pub async fn check_or_get_tweets_data() {
    let current_dataset_page: String = reqwest::get(CURRENT_VERSION_URL)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("current_dataset_page = {}", current_dataset_page);

    let current_dataset_file_link: String =
        current_dataset_page_to_dataset_file_link(current_dataset_page);

    let current_dataset_file_contents = reqwest::get(current_dataset_file_link)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    save_downloaded_file(
        "/data/tweets.tar.gz",
        current_dataset_file_contents.as_ref(),
    )
}

fn current_dataset_page_to_dataset_file_link(current_dataset_page: String) -> String {}

fn save_downloaded_file(file_path: &str, current_dataset_file_contents: &[u8]) {
    let mut out = File::create(file_path)
        .expect(format!("Failed to create the file \"{}\".", file_path).as_str());
    io::copy(current_dataset_file_contents, &mut out).expect("failed to copy content");
}
