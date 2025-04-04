use anki_conversion_derive::Note;

use crate::Note;

#[derive(Debug, Note)]
pub struct SimpleNote {
    pub(crate) audio: String,
    pub(crate) word: String,
    pub(crate) tags: String,
}
