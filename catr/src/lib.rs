use std::error::Error;
use clap::{App, Arg}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Risto Flink <risto.flink@pm.me>")
        .about("Rust cat")
        .arg(
            Arg::with_name("number_lines")
            .short("n")
            .help("Number the output lines, starting at 1.")
            .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
            .short("b")
            .help("Number the non-blank output lines, starting at 1.")
            .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: ...,
        number_lines: ...,
        number_nonblank_lines: ...,
    })
}
