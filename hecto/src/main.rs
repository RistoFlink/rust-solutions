use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    //perform a bitwise AND-operation to check if control is pressed
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                //check if the input is a control character (ASCII 0-31 & 127) and don't print it
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    //otherwise, print the character
                    println!("{:?} ({})\r", b, c);
                } //quit the program if the user presses ctrl+q
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    }
}
