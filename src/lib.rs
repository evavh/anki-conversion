use std::fs;
use std::io::Write;

pub use crate::minimal_pair_note::MinimalPairNote;
pub use crate::parse::{parse_notes, FieldInfo, ToLine};
pub use crate::simple_note::SimpleNote;
pub use crate::spelling_note::SpellingNote;

mod minimal_pair_note;
mod parse;
mod simple_note;
mod spelling_note;

pub trait Note {
    fn remove_html(self) -> Self;
    fn to_line(self, separator: char) -> String;
    fn from_line(line: &str, separator: char) -> Self;
}

pub fn remove_html(word: &str) -> String {
    let pattern = regex::Regex::new("<.*?>").unwrap();
    pattern
        .replace_all(word, "")
        .replace("&nbsp;", "")
        .replace("\"", "")
}

pub fn save<T: ToLine>(notes: Vec<T>, new_path: &str, field_info: FieldInfo) {
    let mut new_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(new_path)
        .unwrap();
    let FieldInfo { header, separator } = field_info;
    write!(new_file, "{}", header).unwrap();
    for note in notes {
        writeln!(new_file, "{}", note.to_line(separator)).unwrap();
    }
}
