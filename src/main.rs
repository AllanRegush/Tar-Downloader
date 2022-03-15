
use std::io::Cursor;
use std::env;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let folder = &args[1];
    let url = &args[2];
    let file_extension = &url[url.len() - 3 ..url.len()];
    if !file_extension.eq(".gz") {
        return Err("url request must be a .gz file".into())
    }
    let response = reqwest::blocking::get(url)?;
    let content =  Cursor::new(response.bytes().unwrap());
    let tar = GzDecoder::new(content);
    let mut archive = Archive::new(tar);
    archive.unpack(format!("{}", folder))?;
    Ok(())
}
