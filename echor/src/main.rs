use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Risto Flink <risto.flink@pm.me>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        // .arg(
        //     Arg::with_name("hello_dear")
        //         .short("d")
        //         .help("Hello dear, love you a lot <3")
        //         .takes_value(false),
        // )
        .get_matches();

    //println!("{:#?}", matches);
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    //let ending = if omit_newline { "" } else { "\n" };
    // let mut ending = "\n";
    // if omit_newline {
    //     ending = ""; // this won't work
    // }
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
