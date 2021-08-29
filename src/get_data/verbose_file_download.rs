/*
   from terminal commands:

   docker run curlimages/curl:7.78.0 -L <url_src> -o <filepath_dest> --name curl-<filepath_dest>
   docker wait curl-<filepath_dest>
   docker rm curl-<filepath_dest>


   or in code:

   https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
   https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
*/

use std::process::Command;

const DATA_DIRECTORY_PATH: &str = "data";
const CURL_CONTAINER_NAME: &str = "Rust-Twitter-Trending-Curl-Container";

#[derive(Clone, Debug)]
pub struct DataFileMetaData<'a> {
    pub name: &'a str,
    pub md5_digest: &'a str,
    pub uri: &'a str,
}

pub fn name_to_filepath(name: &str) -> String {
    format!("{}/{}", DATA_DIRECTORY_PATH, name)
}

pub fn download_data_files(dfs: Vec<DataFileMetaData>) {
    if dfs.is_empty() {
        return;
    }

    let mut curl_cmd: &mut Command = Command::new("docker").args(["run", "curlimages/curl:7.78.0"]);

    for df in dfs {
        curl_cmd = curl_cmd.args(["-L", df.uri, "-o", name_to_filepath(df.name)]);
    }

    curl_cmd = curl_cmd.args(["--name", CURL_CONTAINER_NAME]);

    let await_curl_cmd: &Command = Command::new("docker").args(["wait", CURL_CONTAINER_NAME]);
    let rm_curl_cmd: &Command = Command::new("docker").args(["rm", CURL_CONTAINER_NAME]);



}