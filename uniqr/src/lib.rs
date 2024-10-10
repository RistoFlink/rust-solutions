use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String, // input filename to read (might be in STDIN if the filename is a dash
    out_file: Option<String>, // output written either to an optional file or STDOUT
    count: bool, // whether or not to print the counts of each line
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Risto Flink <risto.flink@pm.me")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .help("Show counts")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of_lossy("in_file").unwrap().to_string(), // convert the in_file argument to a String
        out_file: matches.value_of("out_file").map(|v| v.to_string()), // convert the out_file argument ton an Option<String>
        count: matches.is_present("count"), // either present or not -> convert to bool
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?; // either read STDIN if input file is a dash or open the given filename
    let mut out_file: Box<dyn Write> = match config.out_file { // mutable out_file will be a boxed value that implements the std::io ::Write trait
        Some(out_name) => Box::new(File::create(out_name)?), // if config.out_file is Some filename, use File::create to try to create the file..
        _ => Box::new(io::stdout()), // .. otherwise use std::io::stdout()
    };

    let mut line = String::new(); // new mutable String buffer to hold each line
    let mut previous = String::new(); // mutable variable to hold the previous line
    let mut count: u64 = 0; // mutable variable to hold the count

    let print = |count: u64, text: &str| { // print closure accepts count and text values
        if count > 0 { // print only if count is greater than 0
            if config.count { // check if config.count value is true
                print!("{:>4} {}", count, text); // print! macro to print the count and text to STDOUT..
            } else {
                print!("{}", text); // otherwise print the text to STDOUT
            }
        };
    };

    loop { // infinite loop
        let bytes = file.read_line(&mut line)?; // read a line and preserve line endings
        if bytes == 0 {
            break; // break out of the loop if no bytes were read
        }

        if line.trim_end() != previous.trim_end() { // compare the current line to the previous line (and remove trailing whitespace)
            print(count, &previous);
            previous = line.clone(); // set previous as a copy of the current line
            count = 0; // reset the count
        }
        count += 1; // increment the count
        line.clear(); // clear the line buffer
    }
    print(count, &previous);
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}