pub use file::{File, Header};
#[cfg(feature = "genanki-rs")]
pub use genanki_rs;
pub use note::Note;

mod file;
mod note;
mod parse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Header entry not found: {0}")]
    HeaderEntryNotFound(String),
    #[error("Number of struct fields ({0}) < fields per line in txt ({1})")]
    NotEnoughStructFields(usize, usize),
    #[error("Number of struct fields ({0}) > fields per line in txt ({1})")]
    TooManyStructFields(usize, usize),
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
