#![warn(clippy::all, clippy::pedantic)]
mod editor;

// fn to_ctrl_byte(c: char) -> u8 {
//     //perform a bitwise AND-operation to check if control is pressed
//     let byte = c as u8;
//     byte & 0b0001_1111
// }

use editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
