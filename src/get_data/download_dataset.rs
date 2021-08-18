use std::fs::{read_to_string, remove_file, write};
use std::io::Read;

use flate2::read::GzDecoder;
use md5::compute;

pub fn download_dataset(
    current_dataset_file_link: String,
    current_dataset_file_md5_digest: String,
    extracted_data_file_path: String,
    compressed_data_file_path: String,
) {
    download_compressed_dataset_file(&current_dataset_file_link, &extracted_data_file_path);
    if !verify_compressed_dataset_file(&current_dataset_file_md5_digest, &extracted_data_file_path)
    {
        panic!("The compressed dataset file failed md5 checksum verification.");
    }
    extract_compressed_dataset_file(&extracted_data_file_path, &compressed_data_file_path);
    remove_file(&extracted_data_file_path);
}

fn extract_compressed_dataset_file(
    extracted_data_file_path: &String,
    compressed_data_file_path: &String,
) {
    let mut decoder = GzDecoder::new(
        file_to_bytes(compressed_data_file_path)
            .expect("Failed to open a decoder to decompress the GZIP-compressed dataset."),
    );
    let mut decompressed_output: String = String::new();
    decoder
        .read_to_string(&mut decompressed_output)
        .expect("Failed to decompress the GZIP-compressed dataset.");
    write(extracted_data_file_path, decompressed_output);
}

fn verify_compressed_dataset_file(
    current_dataset_file_md5_digest: &String,
    extracted_data_file_path: &String,
) -> bool {
    match file_to_bytes(extracted_data_file_path) {
        Some(file_bytes) => {
            format!("{:x}", compute(file_bytes)).eq(current_dataset_file_md5_digest)
        }
        _ => false,
    }
}

fn file_to_bytes(filepath: &String) -> Option<&[u8]> {
    match read_to_string(filepath).ok() {
        Some(contents) => Some(contents.as_bytes()),
        _ => None,
    }
}

fn download_compressed_dataset_file(
    current_dataset_file_link: &String,
    extracted_data_file_path: &String,
) {

    //TODO: download latest file with progress bar (like wget)
}
