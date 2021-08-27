pub fn download_file_with_progress(url_src: &str, filepath_dest: &str) {
    //TODO: download latest file with progress bar (like wget)
}

// docker run curlimages/curl:7.78.0 -L <url_src> -o <filepath_dest> --name curl-<filepath_dest>
// docker wait curl-<filepath_dest>
// docker rm curl-<filepath_dest>
