import hashlib
import math
import os
import random

import pandas as pandas
import requests

ALPHA = 0.003863  # ratio to get desired file size
OUTPUT_DATA_FILE_FILE_SIZE = 95000000  # 95MB in bytes, to stay below github's max push size
LINES_PER_OUTPUT_DATA_FILE = int(round(ALPHA * OUTPUT_DATA_FILE_FILE_SIZE))

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

DIR_PATH = os.path.dirname(os.path.realpath(__file__))


def does_file_exist(file_path):
    return os.path.isfile(file_path)


def is_downloaded_file_intact(file_path, expected_md5_digest):
    downloaded_file_bytes = open(file_path, "rb").read()
    actual_md5_digest = str(hashlib.md5(downloaded_file_bytes).digest().hex())
    print(
        "in is_downloaded_file_intact() for file_path = " + file_path + ", expected_md5_digest = " + expected_md5_digest
        + ", and actual_md5_digest = " + actual_md5_digest)
    return expected_md5_digest == actual_md5_digest


def download_file(url, file_path):
    r = requests.get(url, allow_redirects=True)
    open(file_path, 'wb').write(r.content)


def delete_file(file_path):
    if does_file_exist(file_path):
        os.remove(file_path)


def file_name_to_file_path(file_name):
    return DIR_PATH.split("/src")[0] + '/data/' + file_name


def get_original_data_files():
    print("get_original_data_files()")
    for [file_name, expected_md5_digest, url] in DATA_FILES_INFO:
        print("file_name = " + file_name)
        file_path = file_name_to_file_path(file_name)

        if not does_file_exist(file_path):
            print("Need to download the file " + file_path + " which wasn't present")
            download_file(url, file_path)

        else:
            print("The file " + file_path + " was already present, not needed to download")

        if not is_downloaded_file_intact(file_path, expected_md5_digest):
            print("Need to download the file " + file_path + " which wasn't intact")
            delete_file(file_path)
            download_file(url, file_path)

        else:
            print("The file " + file_path + " was already present and intact")

        success = is_downloaded_file_intact(file_path, expected_md5_digest)

        print("Final result for the file " + file_path + " was: " + str(success))

        if not success:
            print("Failed to get the data file " + file_name)
            raise


def get_df_from_data_file_info(data_file_info):
    file_name = data_file_info[0]
    file_path = file_name_to_file_path(file_name)
    print("in get_df_from_data_file_info(), file_name = " + file_name + ", and file_path = " + file_path)
    return pandas.read_csv(file_path)[["id_str", "text"]]


def get_tweet_ids_tweet_texts():
    dfs = map(lambda data_file_info: get_df_from_data_file_info(data_file_info),
              DATA_FILES_INFO)

    print("done getting dfs in get_tweet_ids_tweet_texts()")

    tweet_ids_tweet_texts = dict()

    for df in dfs:
        for row in df.itertuples(index=False):
            tweet_ids_tweet_texts[str(row[0])] = row[1]

    return list(tweet_ids_tweet_texts.values())


def write_tweets_to_output_file(tweets, index):
    file_path = './data/out-' + str(index) + ".csv"
    df = pandas.DataFrame(tweets, columns=["text"])
    df.to_csv(file_path, index=False)


def gen_processed_data_files():
    print("gen_processed_data_files()")

    tweet_ids_tweet_texts = get_tweet_ids_tweet_texts()
    random.shuffle(tweet_ids_tweet_texts)

    print("ready to write output files, str(len(tweet_ids_tweet_texts)) = " + str(len(tweet_ids_tweet_texts)))

    index = 0
    num_files = str(math.ceil(len(tweet_ids_tweet_texts) / LINES_PER_OUTPUT_DATA_FILE))

    while len(tweet_ids_tweet_texts) > 0:
        print("writing file " + str((index + 1)) + " of " + num_files)
        tweets = tweet_ids_tweet_texts[:LINES_PER_OUTPUT_DATA_FILE]
        del tweet_ids_tweet_texts[:LINES_PER_OUTPUT_DATA_FILE]
        write_tweets_to_output_file(tweets, index)
        index += 1


def main():
    dir_path = os.path.dirname(os.path.realpath(__file__))

    print("dir_path = " + dir_path)
    get_original_data_files()
    gen_processed_data_files()


main()
