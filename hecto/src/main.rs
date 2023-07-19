#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#[allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
mod document;
mod editor;
mod row;
mod terminal;
// fn to_ctrl_byte(c: char) -> u8 {
//     //perform a bitwise AND-operation to check if control is pressed
//     let byte = c as u8;
//     byte & 0b0001_1111
// }
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
