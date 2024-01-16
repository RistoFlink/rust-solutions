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
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .required(true)
                .min_values(1)
                .default_value("-"),
            )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Show line count")
                .short("l")
                .long("--lines")
                .required(false)
                .default_value("true"),
            )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("Show word count")
                .short("w")
                .long("--words")
                .required(false)
                .default_value("true"),
            )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Show byte count")
                .short("c")
                .long("--bytes")
                .required(false)
                .default_value("true"),
            )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Show character count")
                .short("m")
                .long("--chars")
                .required(false)
                .default_value("false"),
            )
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
