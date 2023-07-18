use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char; //as transforms to other primitive types

        //check if the input is a control character (ASCII 0-31 & 127) and don't print it
        //otherwise, print the character
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?}, ({})\r", b, c);
        }
        //stop the program if user inputs q
        if c == 'q' {
            break;
        }
    }
}
