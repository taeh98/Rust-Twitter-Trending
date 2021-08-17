use reqwest::{get, Response};
use scraper::html::Select;
use scraper::node::{Attrs, Element};
use scraper::{ElementRef, Html, Node, Selector};
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
use std::fs::File;
use std::io;

const CURRENT_VERSION_URL: &str = "https://doi.org/10.5281/zenodo.3723939";

pub async fn check_or_get_tweets_data() {
    let current_dataset_page: String = get(CURRENT_VERSION_URL)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(current_dataset_page.as_str());

    match current_dataset_page_to_dataset_file_link_and_md5_digest(document) {
        Some(link_digest_pair) => {
            match link_digest_pair {
                (current_dataset_file_link, current_dataset_file_md5_digest) => {
                    println!("current_dataset_file_link = \"{}\"", current_dataset_file_link);
                    println!("current_dataset_file_md5_digest = \"{}\"", current_dataset_file_md5_digest);

                    let current_dataset_file_response: Response = get(current_dataset_file_link).await.unwrap();

                    save_downloaded_file(
                        "/data/tweets.tar.gz",
                        current_dataset_file_response
                            .text()
                            .await
                            .unwrap()
                            .as_bytes(),
                    )
                }
            }
        }
        _ => {
            eprintln!("Failed to get the database file link and MD5 digest.");
        }
    }
}

fn get_href_from_a_element(a_el: ElementRef) -> Option<String> {
    let attrs: Attrs = a_el.value().attrs();
    for attribute in attrs {
        if attribute.0 == "href" {
            return Some(String::from(attribute.1));
        }
    }

    None
}

fn find_dataset_link_element(document: &Html) -> Option<ElementRef> {
    let file_link_element_selector: Selector = Selector::parse(r#"a.filename"#).unwrap();
    let file_link_elements: Select = document.select(&file_link_element_selector);

    for file_link_element in file_link_elements {
        let href = get_href_from_a_element(file_link_element);
        match href {
            Some(link) => {
                if link.contains("full_dataset_clean.tsv.gz") {
                    return Some(file_link_element);
                }
            }
            _ => {}
        }
    }

    None
}

fn find_small_md5_element_from_dataset_link_element(dataset_link_element: ElementRef,
) -> Option<ElementRef> {
    match dataset_link_element.parent() {
        Some(parent_table_cell_node_ref) => {
            let small_element_selector: Selector = Selector::parse(r#"small"#).unwrap();
            let parent_element: ElementRef = ElementRef::wrap(parent_table_cell_node_ref).unwrap();

            let mut small_elements = parent_element.select(&small_element_selector);

            small_elements.next()
        }
        _ => None,
    }
}

fn process_digest_string(start_string: String) -> Option<String> {
    assert!(start_string.starts_with("md5:"));
    let split_string_md5 = start_string.split("md5:");

    for md5_split_string in split_string_md5 {
        for whitespace_split_string in md5_split_string.split_whitespace() {
            return Some(String::from(whitespace_split_string));
        }
    }

    None
}

fn find_md5_digest_from_small_element(small_md5_element: ElementRef) -> Option<String> {
    for child in small_md5_element.children() {
        if let Some(text) = child.value().as_text() {
            let string = text.to_string();
            return process_digest_string(string);
        }
    }

    None
}

fn find_dataset_md5_digest_from_dataset_link_element(
    dataset_link_element: ElementRef,
) -> Option<String> {
    match find_small_md5_element_from_dataset_link_element(dataset_link_element) {
        Some (small_md5_element) => {
            find_md5_digest_from_small_element(small_md5_element)
        }
        _ => None
    }
}

fn current_dataset_page_to_dataset_file_link_and_md5_digest(
    document: Html,
) -> Option<(String, String)> {
    match find_dataset_link_element(&document) {
        Some(dataset_link_element) => {
            let link: String = format!("https://zenodo.org{}", get_href_from_a_element(dataset_link_element).unwrap());
            let digest =
                find_dataset_md5_digest_from_dataset_link_element(dataset_link_element).unwrap();

            return Some((link, digest));
        }
        _ => None,
    }
}

fn save_downloaded_file(file_path: &str, mut current_dataset_file_contents: &[u8]) {
    let mut out = File::create(file_path)
        .expect(format!("Failed to create the data file \"{}\".", file_path).as_str());
    io::copy(&mut current_dataset_file_contents, &mut out)
        .expect(format!("Failed to copy content to the data file \"{}\".", file_path).as_str());
}
