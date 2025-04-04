use anki_conversion_derive::Note;

use crate::Note;

#[derive(Debug, Note)]
pub struct SpellingNote {
    spelling: String,
    pub word: String,
    pub picture: String,
    pub audio: String,
    pub ipa: String,
}
