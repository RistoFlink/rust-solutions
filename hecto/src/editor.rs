use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        //check if the input is a control character (ASCII 0-31 & 127) and just print the ASCII value
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            //otherwise, print the character and ASCII value
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{key:?}\r"), //the default option: if not a character or ctrl+q, just print it
                },
                Err(err) => die(&err),
            }
        }
    }
    pub fn default() -> Self {
        Self {}
    }
}

fn die(e: &std::io::Error) {
    panic!("{}", e);
}
