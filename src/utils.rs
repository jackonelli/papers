use std::io;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};
use reqwest::{Url, UrlError};
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;


pub fn parse_url(url: &str) -> Result<Url, UrlError> {
    match Url::parse(url) {
        Ok(url) => Ok(url),
        Err(error) if error == UrlError::RelativeUrlWithoutBase => {
            let url_with_base = format!("{}{}", "http://", url);
            Url::parse(url_with_base.as_str())
        }
        Err(error) => Err(error),
    }

}

pub fn gen_error(msg: String) -> Result<(), Box<::std::error::Error>> {
    Err(Box::new(Error::new(ErrorKind::Other, msg)))
}

pub fn get_file_handle(fname: &str, resume_download: bool) -> io::Result<Box<Write>> {
    if fname == "-" {
        return Ok(Box::new(io::stdout()));
    }
    if resume_download && Path::new(fname).exists() {
        match OpenOptions::new().append(true).open(fname) {
            Ok(file) => Ok(Box::new(file)),
            Err(error) => Err(error),
        }
    } else {
        match OpenOptions::new().write(true).create(true).open(fname) {
            Ok(file) => Ok(Box::new(file)),
            Err(error) => Err(error),
        }
    }
}

pub fn read_file(filepath: &str) -> String {
    let file = fs::File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };
    contents
}
