use clap::{App, Arg};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    //dbg!(config);
    for filename in config.files {
        let mut contents = fs::read_to_string(&filename);
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                let mut index: i8 = 1;
                for line_result in reader.lines() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{} {}", index, line);
                        index += 1;
                    } else if config.number_nonblank_lines {
                        if !line.trim().is_empty() {
                            println!("{} {}", index, line);
                            index += 1;
                        } else {
                            println!("{}", line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Risto Flink <risto.flink@pm.me>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files.")
                .required(true)
                .min_values(1)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("--number")
                .help("Number the output lines, starting at 1.")
                .conflicts_with("number_nonblank_lines")
                .default_value("false")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("--number-nonblank")
                .help("Number the non-blank output lines, starting at 1.")
                .default_value("false")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
