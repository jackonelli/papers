use std::process;
use clap::clap_app;
use clap::crate_version;
use clap::crate_authors;
use clap::crate_description;

use papers::download::{ftp_download, http_download};
use papers::utils;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<::std::error::Error>> {
    let args = clap_app!(papers =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg URL: +required +takes_value "url to download")
        ).get_matches();

    let url = utils::parse_url(args.value_of("URL").unwrap())?;
    let quiet_mode = args.is_present("quiet");
    let file_name = args.value_of("FILE");

    match url.scheme() {
        "ftp" => ftp_download(url, quiet_mode, file_name),
        "http" | "https" => http_download(url, &args, crate_version!()),
        _ => utils::gen_error(format!("unsupported url scheme '{}'", url.scheme())),
    }
}
