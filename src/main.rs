use clap::{arg, command};
use flate2::read::GzDecoder;
use std::env;
use std::io::Cursor;
use tar::Archive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .arg(arg!([folder] "Folder Name of where the Archive will be extracted").required(true))
        .arg(arg!([url] "URL of the file to be downloaded").required(true))
        .get_matches();

    let folder = matches.value_of("folder").unwrap();
    let url = matches.value_of("url").unwrap();
    let file_extension_gz = &url[url.len() - 3..url.len()];
    let file_extension_tgz = &url[url.len() - 4..url.len()];
    if !file_extension_gz.eq(".gz") && !file_extension_tgz.eq(".tgz") {
        return Err("url request must be a .gz or .tgz file".into());
    }
    let response = reqwest::blocking::get(url)?;
    let content = Cursor::new(response.bytes().unwrap());
    let tar = GzDecoder::new(content);
    let mut archive = Archive::new(tar);
    archive.unpack(format!("{}", folder))?;

    Ok(())
}

