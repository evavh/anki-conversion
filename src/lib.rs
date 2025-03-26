mod minimal_pair_note;
mod parse;
mod simple_note;
mod spelling_note;

pub use crate::minimal_pair_note::MinimalPairNote;
pub use crate::parse::{parse_notes, FieldInfo};
pub use crate::simple_note::SimpleNote;
pub use crate::spelling_note::SpellingNote;

fn clean_html(word: &str) -> String {
    let pattern = regex::Regex::new("<.*?>").unwrap();
    pattern
        .replace_all(word, "")
        .replace("&nbsp;", "")
        .replace("\"", "")
}
