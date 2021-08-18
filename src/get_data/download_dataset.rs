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
    remove_file(&extracted_data_file_path)
        .expect("Failed to remove the GZIP-compressed datafile after extracting it.");
}

fn extract_compressed_dataset_file(
    extracted_data_file_path: &String,
    compressed_data_file_path: &String,
) {
    let compressed_file_u8_vec: Vec<u8> = file_to_u8_vec(compressed_data_file_path)
        .expect("Failed to open the GZIP-compressed dataset as a u8 vector.");
    let mut decoder = GzDecoder::new(compressed_file_u8_vec.as_slice());
    let mut decompressed_output: String = String::new();
    decoder
        .read_to_string(&mut decompressed_output)
        .expect("Failed to decompress the GZIP-compressed dataset.");
    write(extracted_data_file_path, decompressed_output)
        .expect("Failed to write the decompressed datafile.");
}

fn verify_compressed_dataset_file(
    current_dataset_file_md5_digest: &String,
    extracted_data_file_path: &String,
) -> bool {
    match file_to_u8_vec(extracted_data_file_path) {
        Some(file_bytes) => {
            format!("{:x}", compute(file_bytes)).eq(current_dataset_file_md5_digest)
        }
        _ => false,
    }
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

fn download_compressed_dataset_file(
    current_dataset_file_link: &String,
    extracted_data_file_path: &String,
) {

    //TODO: download latest file with progress bar (like wget)
}
