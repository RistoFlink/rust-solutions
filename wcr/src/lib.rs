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
                .default_value("-")
                .multiple(true),
            )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Show line count")
                .short("l")
                .long("--lines")
                .takes_value(false),
            )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("Show word count")
                .short("w")
                .long("--words")
                .takes_value(false),
            )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Show byte count")
                .short("c")
                .long("--bytes")
                .conflicts_with("chars")
                .takes_value(false),
            )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Show character count")
                .short("m")
                .long("--chars")
                .takes_value(false),
            )
        .get_matches();
    
    // unpack all the flags
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");

    // if all the flags are false, set lines, words and bytes to true
    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    // use the struct field initialization shorthand to set the values
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
