import hashlib
import os
import random

import pandas as pandas
import requests

BYTES_PER_LINE = 172
LINES_PER_BYTE = 1 / BYTES_PER_LINE
OUTPUT_DATA_FILE_FILE_SIZE = 9.75e+7  # 95MB in bytes, to stay below github's max push size
OUTPUT_DATA_FILES_TOTAL_SIZE = 2e+9  # 2GB in bytes
NUM_OUTPUT_DATA_FILES_TO_MAKE = round(OUTPUT_DATA_FILES_TOTAL_SIZE / OUTPUT_DATA_FILE_FILE_SIZE)
LINES_PER_OUTPUT_DATA_FILE = LINES_PER_BYTE * OUTPUT_DATA_FILE_FILE_SIZE

DATA_FILES_INFO = [
    [
        "full_who_dataset1.csv",
        "259389f2f6c1b232fe248c91107eeccd",
        "https://zenodo.org/record/3928240/files/full_who_dataset1.csv?download=1",
    ],
    [
        "full_who_dataset2.csv",
        "ea266ada5b1b817638ab89388138d95e",
        "https://zenodo.org/record/3928240/files/full_who_dataset2.csv?download=1",
    ],
    [
        "full_who_dataset3.csv",
        "fc4b898f8d7c81293a776bf116668bab",
        "https://zenodo.org/record/3928240/files/full_who_dataset3.csv?download=1",
    ],
]


def does_file_exist(file_path):
    return os.path.isfile(file_path)


def is_downloaded_file_intact(file_path, expected_md5_digest):
    downloaded_file_bytes = open(file_path, "rb").read()
    actual_md5_digest = hashlib.md5(downloaded_file_bytes)
    return expected_md5_digest == actual_md5_digest


def download_file(url, file_path):
    r = requests.get(url, allow_redirects=True)
    open(file_path, 'wb').write(r.content)


def delete_file(file_path):
    if does_file_exist(file_path):
        os.remove(file_path)


def file_name_to_file_path(file_name):
    return './data/' + file_name


def get_original_data_files():
    for [file_name, expected_md5_digest, url] in DATA_FILES_INFO:
        file_path = file_name_to_file_path(file_name)

        if not does_file_exist(file_path):
            download_file(url, file_path)

        if not is_downloaded_file_intact(file_path, expected_md5_digest):
            delete_file(file_path)
            download_file(url, file_path)

        success = is_downloaded_file_intact(file_path, expected_md5_digest)

        if not success:
            print("Failed to get the data file " + file_name)
            raise


def get_tweet_ids_tweet_texts():
    dfs = map(lambda data_file_info: pandas.read_csv(file_name_to_file_path(data_file_info[0]))["id_str", "text"],
              DATA_FILES_INFO)

    tweet_ids_tweet_texts = dict()

    for df in dfs:
        for row in df.itertuples():
            tweet_ids_tweet_texts[row["id_str"]] = row["text"]

    return tweet_ids_tweet_texts.values()


def write_tweets_to_output_file(tweets, index):
    file_path = './data/out-' + index + ".csv"
    df = pandas.DataFrame(tweets, columns=["text"])
    df.to_csv(file_path, index=False)


def gen_processed_data_files():
    tweet_ids_tweet_texts = get_tweet_ids_tweet_texts()
    random.shuffle(tweet_ids_tweet_texts)

    for index in range(NUM_OUTPUT_DATA_FILES_TO_MAKE):
        tweets = tweet_ids_tweet_texts[:LINES_PER_OUTPUT_DATA_FILE]
        del tweet_ids_tweet_texts[:LINES_PER_OUTPUT_DATA_FILE]
        write_tweets_to_output_file(tweets, index)


def main():
    get_original_data_files()
    gen_processed_data_files()


main()
