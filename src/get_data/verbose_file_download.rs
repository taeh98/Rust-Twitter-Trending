/*
   from terminal commands:

   docker run curlimages/curl:7.78.0 -L <url_src> -o <filepath_dest> --name curl-<filepath_dest>
   docker wait curl-<filepath_dest>
   docker rm curl-<filepath_dest>


   or in code:

   https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
   https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
*/

use core::result::Result::Err;
use std::fs::File;
use std::str::FromStr;

use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;

struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self, &'static str> {
        match buffer_size == 0 {
            true => Err("invalid buffer_size, give a value greater than zero."),
            _ => Ok(PartialRangeIter {
                start,
                end,
                buffer_size,
            }),
        }
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            Some(
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                    .expect("string provided by format!"),
            )
        }
    }
}

pub fn download_file_with_progress(url_src: &str, filepath_dest: &str) {
    const CHUNK_SIZE: u32 = 10240;

    let client: Client = reqwest::blocking::Client::new();
    let response: Response = client
        .head(url_src)
        .send()
        .expect("Failed to get client head.");
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .expect("response doesn't include the content length");
    let length: u64 = u64::from_str(
        length
            .to_str()
            .expect("Failed to convert the content length into a String."),
    )
    .map_err(|_| "invalid Content-Length header")
    .expect("Failed to get the content length.");

    let mut output_file: File =
        File::create(filepath_dest).expect("Failed to create the output file.");

    println!("starting download...");
    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)
        .expect("Failed to make a new PartialRangeIter for the data chunks.")
    {
        println!("range {:?}", range);
        let mut response: Response = client
            .get(url_src)
            .header(RANGE, range)
            .send()
            .expect("Failed to retrieve a data chunk from a remote server.");

        let status: StatusCode = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            panic!("Unexpected server response: {}", status);
        }
        std::io::copy(&mut response, &mut output_file)
            .expect("Failed to copy a data chunk to the output file.");
    }

    let content: String = response
        .text()
        .expect("Failed to read the downloaded data into a string.");
    std::io::copy(&mut content.as_bytes(), &mut output_file)
        .expect("Failed to copy the downloaded data to the output file.");

    println!("Finished with success!");
}
