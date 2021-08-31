/*
   from terminal commands:

   docker run curlimages/curl:7.78.0 -L <url_src> -o <filepath_dest> --name curl-<filepath_dest>
   docker wait curl-<filepath_dest>
   docker rm curl-<filepath_dest>


   or in code:

   https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
   https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
*/

use std::io::Result;
use std::process::{Command, ExitStatus};

const DATA_DIRECTORY_PATH: &str = "data";
const CURL_CONTAINER_NAME: &str = "Rust-Twitter-Trending-Curl-Container";
const CURL_IMAGE_NAME: &str = "curlimages/curl:7.78.0";

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

    let mut curl_cmd: Command = Command::new("docker");
    curl_cmd.args(["run", "--name", CURL_CONTAINER_NAME, CURL_IMAGE_NAME]);

    for df in dfs {
        curl_cmd.args(["-L", df.uri, "-o", name_to_filepath(df.name).as_str()]);
    }

    let mut await_curl_cmd: Command = Command::new("docker");
    await_curl_cmd.args(["wait", CURL_CONTAINER_NAME]);

    let mut rm_curl_cmd: Command = Command::new("docker");
    rm_curl_cmd.args(["rm", CURL_CONTAINER_NAME]);

    let mut docker_pull_cmd: Command = Command::new("docker");
    docker_pull_cmd.args(["pull", CURL_IMAGE_NAME]);

    rm_curl_cmd.status();

    assert!(
        run_command_and_get_if_success(&mut docker_pull_cmd),
        "Could not pull the curl Docker image."
    );
    assert!(
        run_command_and_get_if_success(&mut curl_cmd),
        "Could not run the curl Docker image."
    );
    assert!(
        run_command_and_get_if_success(&mut await_curl_cmd),
        "Could not wait for the curl Docker image to complete."
    );

    rm_curl_cmd.status();
}

fn run_command_and_get_if_success(cmd: &mut Command) -> bool {
    let status: Result<ExitStatus> = cmd.status();
    status.is_ok() && status.unwrap().success()
}
