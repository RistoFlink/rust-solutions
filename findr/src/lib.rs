use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;
#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>, // paths will be a vector of strings and may name files or directories
    names: Vec<Regex>, // names will be a vector of compiled regular expressions represented by the type regex::Regex
    entry_type: Vec<EntryType>, // entry_types will be a vector of EntryType variants
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Risto FLink <risto.flink@pm.me>")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Path to search for")
                .default_value(".")
                .multiple(true),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name to search for")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Type to search for")
                .possible_values(&["f", "d", "l"])
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

let names = matches
    .value_of_lossy("names")
    .map(|vals| {
    vals.into_iter()
        .map(|name| {
            Regex::new(&name)
                .map_err(|_| format!("Invalid --name: \"{}\"", name))
        })
        .collect::<Result<Vec<_>, _>>()
    })
    .transpose()?
    .unwrap_or_default();
})


pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}