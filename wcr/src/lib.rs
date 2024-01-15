use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool, // print the line count or not
    words: bool, // print the word count or not
    bytes: bool, // print the byte count or not
    chars: bool, // print the char count or not
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Risto Flink <risto.flink@pm.me>")
        .about("Rust wc")
        // TODO
        .get_matches();

    Ok(Config {
        files: // TODO
        lines:
        words:
        bytes:
        chars:
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
