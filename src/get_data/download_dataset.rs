pub fn download_dataset(
    current_dataset_file_link: String,
    current_dataset_file_md5_digest: String,
    extracted_data_file_path: String,
    compressed_data_file_path: String,
) {
    //TODO: download latest file with progress bar (like wget)
    //TODO: once downloaded, verify downloaded file against md5 digest
    //TODO: once verified, extract downloaded .tsv.gz file to a .tsv file
    //TODO: once extracted, delete the .tsv.gz file
}
