use std::fs;
use std::io::Write;

use parse::{extract_header, find_header_entry, parse_separator};

pub use crate::parse::FieldInfo;

mod parse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Header entry not found: {0}")]
    HeaderEntryNotFound(String),
    #[error("Number of struct fields < fields per line in txt")]
    NotEnoughStructFields,
    #[error("Number of struct fields > fields per line in txt")]
    TooManyStructFields,
}

pub trait Note: Sized {
    #[must_use]
    fn remove_html(self) -> Self;
    fn to_line(self, separator: char) -> String;
    fn from_line(line: &str, separator: char) -> Result<Self, Error>;

    #[must_use]
    fn parse_txt(path: &str) -> Result<(Vec<Self>, FieldInfo), Error> {
        let data = fs::read_to_string(path)?;
        let header = extract_header(&data);
        let separator = find_header_entry(&data, "separator")?;
        let separator = parse_separator(&separator);
        let mut lines: Vec<_> = data
            .lines()
            .skip_while(|line| line.starts_with('#'))
            .collect();
        lines.sort_unstable();
        let notes = lines
            .into_iter()
            .map(|line| Ok(Self::from_line(line, separator)?))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok((notes, FieldInfo { separator, header }))
    }

    fn save_to_txt(
        notes: Vec<Self>,
        new_path: &str,
        field_info: FieldInfo,
    ) -> Result<(), Error>
    where
        Self: Sized,
    {
        let mut new_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(new_path)?;
        let FieldInfo { header, separator } = field_info;

        write!(new_file, "{header}")?;
        for note in notes {
            writeln!(new_file, "{}", note.to_line(separator))?;
        }

        Ok(())
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
