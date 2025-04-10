use std::fs;
use std::io::Write;

use crate::parse::{
    extract_header, find_header_entry, parse_separator, FieldInfo,
};
use crate::Error;

pub trait Note: Sized {
    #[must_use]
    fn remove_html(self) -> Self;
    fn into_line(self, separator: char) -> String;
    fn from_line(line: &str, separator: char) -> Result<Self, Error>;
    // Must be like this because of the orphan rule
    // Otherwise we would impl Into<genanki_rs::Note> for types that
    // impl Note
    #[cfg(feature = "genanki-rs")]
    fn into_genanki(
        self,
        model_id: i64,
    ) -> Result<genanki_rs::Note, genanki_rs::Error>;

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
            writeln!(new_file, "{}", note.into_line(separator))?;
        }

        Ok(())
    }
}
