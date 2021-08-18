use std::fs::read_to_string;

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
    //TODO: once verified, extract downloaded .tsv.gz file to a .tsv file
    //TODO: once extracted, delete the .tsv.gz file
}

fn verify_compressed_dataset_file(
    current_dataset_file_md5_digest: &String,
    extracted_data_file_path: &String,
) -> bool {
    match file_to_bytes(extracted_data_file_path) {
        Some(file_bytes) => compute(file_bytes).eq(current_dataset_file_md5_digest.as_bytes()),
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
