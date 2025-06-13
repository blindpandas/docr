use clap::{Arg, Command};
use docrapi::*;
use std::process;

fn main() {
    let matches = Command::new("Docr")
        .version("0.1")
        .author("Musharraf Omer <ibnomer2011@hotmail.com>")
        .about("Converts images to text using optical character recognition (OCR).")
        .arg(
            Arg::new("FILENAME")
                .required(true)
                .index(1)
                .help("File to recognize. Supported files include images [*.jpg, *.png]"),
        )
        .arg(
            Arg::new("language")
                .help("OCR language")
                .long("lang")
                .short('l')
                .default_value("en"),
        )
        .get_matches();

    let filename = matches.value_of("FILENAME").unwrap();
    let language = matches.value_of("language").unwrap();

    let exit_code = match recognize_image(language, filename) {
        Ok(text) => {
            println!("{}", text);
            0
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    };
    process::exit(exit_code);
}
