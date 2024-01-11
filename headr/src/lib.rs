use clap::{App, Arg};
use std::{error::Error, io::{BufRead, BufReader, self}};
use std::fs::File;
use std::io::Read;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
     // use the Vec::len method to get the number of files
    let num_files = config.files.len();

    // use the Iterator::enumerate method to track the file number and filenames
    for (file_num, filename) in config.files.iter().enumerate() { 
        match open(&filename) {
            Err(err) => eprintln!("headr: {}: {}", filename, err),
            Ok(mut file) => { // accept the filehandle as a mutable value
                if num_files > 1 { // only print headers if there are multiple files
                    println!(
                        "{}==> {} <==",
                        // print a newline when file_num is greater than 0, which indicates the first file
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                // use pattern matching to check if config.bytes is Some number of bytes
                if let Some(num_bytes) = config.bytes { 
                    let mut handle = file.take(num_bytes as u64); // use take to read the requested number of bytes

                    let mut buffer = vec![0; num_bytes]; // create a mutable buffer of length num_bytes to hold the bytes read

                    let bytes_read = handle.read(&mut buffer)?; // read the desired number of bytes from the filehandle into the buffer
                    print!(
                        "{}",
                        String::from_utf8_lossy(&buffer[..bytes_read]) // convert the selected bytes into a string
                      );
                 } else {
                        let mut line = String::new(); // create a new empty mutable string buffer to hold each line

                        for _ in 0..config.lines { // Iterate through a std::ops::Range from 0 to requested number of lines
                            let bytes = file.read_line(&mut line)?; // use BufRead::read_line to read the next line
                            if bytes == 0 { // break when filehandle returns zero bytes
                                break;
                            }
                            print!("{}", line); // print the line, including the original ending
                            line.clear(); // use String::clear to empty the line buffer
                        }
                    }
                }
            }
        };
    Ok(())
   }

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Risto Flink <risto.flink@pm.me>")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .help("How many lines to be displayed.")
                .value_name("LINES")
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("How many bytes to be displayed.")
                .conflicts_with("lines")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s).")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(), 
        bytes
})
}
// really not an elegant solution to this problem..
// fn parse_positive_int(val: &str) -> MyResult<usize> {
//     match val.parse::<usize>() {
//         //parsed_val: usize = val.parse()?;
//         Ok(parsed_val) => {
//             if parsed_val > 0 {
//                 return Ok(parsed_val);
//             }
//         }
//         Err(_) => {} //unimplemented!();
//     }
//     Err(val.into())
// }

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}


#[test]
fn test_parse_positive_int() {
    // integers like 3 works
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // strings don't work
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // zero doesn't work
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
