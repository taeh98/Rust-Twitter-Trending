pub fn download_file_with_progress(url_src: &str, filepath_dest: &str) {
    //TODO: download latest file with progress bar (like wget)
}

/*
    from terminal commands:

    docker run curlimages/curl:7.78.0 -L <url_src> -o <filepath_dest> --name curl-<filepath_dest>
    docker wait curl-<filepath_dest>
    docker rm curl-<filepath_dest>


    or in code:

    https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html#make-a-partial-download-with-http-range-headers
    https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
 */
