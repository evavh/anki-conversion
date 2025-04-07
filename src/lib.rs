use std::fs;
use std::io::Write;

use parse::{extract_header, find_header_entry, parse_separator};

pub use crate::parse::FieldInfo;

mod parse;

pub trait Note {
    #[must_use]
    fn remove_html(self) -> Self;
    fn to_line(self, separator: char) -> String;
    fn from_line(line: &str, separator: char) -> Self;

    #[must_use]
    fn parse_txt(path: &str) -> (Vec<Self>, FieldInfo)
    where
        Self: Sized,
    {
        let data = fs::read_to_string(path).unwrap();
        let header = extract_header(&data);
        let separator = find_header_entry(&data, "separator").unwrap();
        let separator = parse_separator(&separator);
        let mut lines: Vec<_> = data
            .lines()
            .skip_while(|line| line.starts_with('#'))
            .collect();
        lines.sort_unstable();
        let notes = lines
            .into_iter()
            .map(|line| Self::from_line(line, separator))
            .collect();

        (notes, FieldInfo { separator, header })
    }

    fn save_to_txt(notes: Vec<Self>, new_path: &str, field_info: FieldInfo)
    where
        Self: Sized,
    {
        let mut new_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(new_path)
            .unwrap();
        let FieldInfo { header, separator } = field_info;
        write!(new_file, "{header}").unwrap();
        for note in notes {
            writeln!(new_file, "{}", note.to_line(separator)).unwrap();
        }
    }
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn remove_html(word: &str) -> String {
    let pattern = regex::Regex::new("<.*?>").expect("Valid regex");
    pattern
        .replace_all(word, "")
        .replace("&nbsp;", "")
        .replace('\"', "")
}
