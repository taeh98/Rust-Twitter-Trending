use std::fs::{create_dir, read_to_string, remove_dir_all};
use std::path::Path;

use md5::compute;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::blocking::get;
use scraper::html::Select;
use scraper::node::Attrs;
use scraper::{ElementRef, Html, Selector};

mod download_dataset;

#[derive(Clone, Debug)]
struct DataFileMetaData<'a> {
    pub name: &'a str,
    pub md5_digest: &'a str,
    pub uri: &'a str,
}

const DATA_DIRECTORY_PATH: &str = "data";

pub fn check_or_get_tweets_data() {
    let data_files: [DataFileMetaData; 3] = [
        DataFileMetaData {
            name: "full_who_dataset1.csv",
            md5_digest: "259389f2f6c1b232fe248c91107eeccd",
            uri: "https://zenodo.org/record/3928240/files/full_who_dataset1.csv?download=1",
        },
        DataFileMetaData {
            name: "full_who_dataset2.csv",
            md5_digest: "ea266ada5b1b817638ab89388138d95e",
            uri: "https://zenodo.org/record/3928240/files/full_who_dataset2.csv?download=1",
        },
        DataFileMetaData {
            name: "full_who_dataset3.csv",
            md5_digest: "fc4b898f8d7c81293a776bf116668bab",
            uri: "https://zenodo.org/record/3928240/files/full_who_dataset3.csv?download=1",
        },
    ];

    data_files
        .to_vec()
        .into_par_iter()
        .for_each(|df: DataFileMetaData| check_or_get_data_file(df));
}

fn name_to_filepath(path: &str) -> String {
    format!("{}/{}", DATA_DIRECTORY_PATH, path)
}

fn check_or_get_data_file(df: DataFileMetaData) {
    let filepath: String = name_to_filepath(df.name);
    let path: &Path = Path::new(filepath.as_str());

    if path.exists() {
        if check_file_integrity(filepath, df.md5_digest) {
            println!("The data file {} was already present and intact.", df.name);
            return;
        }
    } else {
        println!("path doesnt already exist");
    }
}

fn check_file_integrity(filepath: String, expected_md5_digest: &str) -> bool {
    let actual_md5_digest: String = gen_file_md5_digest(&filepath).unwrap();
    actual_md5_digest.as_str().eq(expected_md5_digest)
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

fn gen_file_md5_digest(filepath: &String) -> Option<String> {
    match file_to_u8_vec(filepath) {
        Some(bytes) => Some(format!("{:x}", compute(bytes))),
        _ => None,
    }
}

// fn get_href_from_a_element(a_el: ElementRef) -> Option<String> {
//     let attrs: Attrs = a_el.value().attrs();
//     for attribute in attrs {
//         if attribute.0 == "href" {
//             return Some(String::from(attribute.1));
//         }
//     }
//
//     None
// }
//
// fn find_dataset_link_element(document: &Html) -> Option<ElementRef> {
//     let file_link_element_selector: Selector = Selector::parse(r#"a.filename"#).unwrap();
//     let file_link_elements: Select = document.select(&file_link_element_selector);
//
//     for file_link_element in file_link_elements {
//         let href = get_href_from_a_element(file_link_element);
//         match href {
//             Some(link) => {
//                 if link.contains("full_dataset_clean.tsv.gz") {
//                     return Some(file_link_element);
//                 }
//             }
//             _ => {}
//         }
//     }
//
//     None
// }
//
// fn find_small_md5_element_from_dataset_link_element(
//     dataset_link_element: ElementRef,
// ) -> Option<ElementRef> {
//     match dataset_link_element.parent() {
//         Some(parent_table_cell_node_ref) => {
//             let small_element_selector: Selector = Selector::parse(r#"small"#).unwrap();
//             let parent_element: ElementRef = ElementRef::wrap(parent_table_cell_node_ref).unwrap();
//
//             let mut small_elements = parent_element.select(&small_element_selector);
//
//             small_elements.next()
//         }
//         _ => None,
//     }
// }
//
// fn process_digest_string(start_string: String) -> Option<String> {
//     assert!(start_string.starts_with("md5:"));
//     let split_string_md5 = start_string.split("md5:");
//
//     for md5_split_string in split_string_md5 {
//         for whitespace_split_string in md5_split_string.split_whitespace() {
//             return Some(String::from(whitespace_split_string));
//         }
//     }
//
//     None
// }
//
// fn find_md5_digest_from_small_element(small_md5_element: ElementRef) -> Option<String> {
//     for child in small_md5_element.children() {
//         if let Some(text) = child.value().as_text() {
//             let string = text.to_string();
//             return process_digest_string(string);
//         }
//     }
//
//     None
// }
//
// fn find_dataset_md5_digest_from_dataset_link_element(
//     dataset_link_element: ElementRef,
// ) -> Option<String> {
//     match find_small_md5_element_from_dataset_link_element(dataset_link_element) {
//         Some(small_md5_element) => find_md5_digest_from_small_element(small_md5_element),
//         _ => None,
//     }
// }
//
// fn current_dataset_page_to_dataset_file_link_and_md5_digest(
//     document: Html,
// ) -> Option<(String, String)> {
//     match find_dataset_link_element(&document) {
//         Some(dataset_link_element) => {
//             let link: String = format!(
//                 "https://zenodo.org{}",
//                 get_href_from_a_element(dataset_link_element).unwrap()
//             );
//             let digest =
//                 find_dataset_md5_digest_from_dataset_link_element(dataset_link_element).unwrap();
//
//             return Some((link, digest));
//         }
//         _ => None,
//     }
// }
//
// fn get_record_id_from_file_link(file_link: &String) -> String {
//     file_link
//         .split("https://zenodo.org/record/")
//         .find(|string: &&str| !string.is_empty())
//         .unwrap()
//         .split("/")
//         .next()
//         .unwrap()
//         .to_string()
// }
//
// fn check_or_download_dataset_file(
//     current_dataset_file_link: String,
//     current_dataset_file_md5_digest: String,
// ) {
//     let record_id: String = get_record_id_from_file_link(&current_dataset_file_link);
//
//     let extracted_data_file_path: String =
//         format!("{}/data_{}.tsv", DATA_DIRECTORY_PATH, record_id);
//     let compressed_data_file_path: String = format!("{}.gz", extracted_data_file_path);
//
//     if Path::new(extracted_data_file_path.as_str()).exists() {
//         println!("The latest dataset has already been downloaded.");
//         return;
//     }
//
//     println!("The latest dataset is not already present, downloading it now.");
//
//     remove_dir_all(DATA_DIRECTORY_PATH).expect("Failed to clear the /data directory.");
//     create_dir(DATA_DIRECTORY_PATH).expect("Failed to recreate the /data directory.");
//
//     download_dataset::download_dataset(
//         current_dataset_file_link,
//         current_dataset_file_md5_digest,
//         extracted_data_file_path,
//         compressed_data_file_path,
//     );
// }
