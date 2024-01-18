use clap::{App, Arg};
use std::{error::Error, io::{BufRead, BufReader, self}};
use std::fs::File;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool, // print the line count or not
    words: bool, // print the word count or not
    bytes: bool, // print the byte count or not
    chars: bool, // print the char count or not
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
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
    for filename in &config.files {
        match open(filename) {
            // when a file fails to open, print the filename and error STDERR
            Err(err) => eprintln!("{}: {}", filename, err),
            // when a file is opened, print a message to STDOUT
            Ok(_) => println!("Opened {}", filename),
        }
    } 
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// accepts a mutable file value, and might return a FileInfo struct
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    // initialize mutable variables to count the lines etc.
   let mut num_lines = 0;
   let mut num_words = 0;
   let mut num_bytes = 0;
   let mut num_chars = 0;

   Ok(FileInfo {
       // return a FileInfo with nothing but zeros (for now)
       num_lines,
       num_words,
       num_bytes,
       num_chars,
   })
}

// TESTING
// this module will only be compiled when testing
#[cfg(test)]
// define a new module tests to contain the test code
mod tests {
    // import the count function and FileInfo struct
    use super::{count, FileInfo};
    // Cursor is used to fake a filehandle for testing
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half..\r\n";
        // run count with the Cursor
        let info = count(Cursor::new(text));
        // ensure the result is Ok
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
       // compare the result to the expected value 
        assert_eq!(info.unwrap(), expected);
    }
}
