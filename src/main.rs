use error_chain::error_chain;
use std::fs::File;
use std::io;
use tempfile::Builder;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}
fn main() {
    // get current directory
    // TODO: create a temp directory using the  Builder::new().prefix("example").tempdir()?;
    let temp_dir = std::env::current_dir().unwrap();
    let url = "https://images.unsplash.com/photo-1606313564200-e75d5e30476c?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=774&q=80.png";
    let mut response = reqwest::blocking::get(url).unwrap();

    // create file name from the response
    let file_name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("tmp.bin");

    let fname = temp_dir.join(file_name);
    // create a file in temp_dir
    let mut file = File::create(fname).unwrap();
    // copy the response to the file
    io::copy(&mut response, &mut file).unwrap();
}
