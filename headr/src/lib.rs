use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", config);
    for filename in config.files {
        match  {
            Err(_) => eprintln!(),
            Ok(_)
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Risto Flink <risto.flink@pm.me>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files.")
                .required(true)
                .min_values(1)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .help("How many lines to be displayed.")
                .default_value("10")
                .required(false)
                .conflicts_with("bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("How many bytes to be displayed.")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.value_of("lines").unwrap().parse::<usize>().unwrap(),
        bytes: matches
            .value_of("bytes")
            .map(|val| val.parse::<usize>().unwrap()),
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
