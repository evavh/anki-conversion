use std::fs;
use std::io::Write;

use crate::parse::{extract_header, find_header_entry, parse_separator};
use crate::{Error, Note};

pub struct File<N: Note> {
    pub header: Header,
    pub notes: Vec<N>,
}

pub struct Header {
    // TODO: add more anki header keys
    pub separator: char,
    pub header: String,
}

impl<N: Note> File<N> {
    #[must_use]
    pub fn parse_txt(path: &str) -> Result<Self, Error> {
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
            .map(|line| Ok(N::from_line(line, separator)?))
            .collect::<Result<Vec<N>, Error>>()?;

        Ok(Self {
            header: Header { separator, header },
            notes,
        })
    }

    pub fn save_to_txt(self, new_path: &str) -> Result<(), Error> {
        let mut new_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(new_path)?;
        let Header { header, separator } = self.header;

        write!(new_file, "{header}")?;
        for note in self.notes {
            writeln!(new_file, "{}", note.into_line(separator))?;
        }

        Ok(())
    }
}
