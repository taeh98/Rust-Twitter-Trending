# Rust-Twitter-Trending
Recreating Twitter's trending feature with their API and MapReduce processing in Rust.

This program runs parallel and serial MapReduce algorithms to recreate Twitter's trending page in Rust. It measures the performance of both to allow for comparisons to be made.

Build and run it as with any cargo project. Output will apeear in the /out directory.

The source file src/make_data_files.py was used to generate the data files.

Adjust the parameters NUM_REPEATS (in src/main.rs) and NUM_DATA_FILES_TO_USE (in src/get_tweets.rs) to optimise the tradeoff between time taken and the effect of chance/less representative samples.
