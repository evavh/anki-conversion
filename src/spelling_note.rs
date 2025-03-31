use crate::{parse::FromLine, ToLine};

#[derive(Debug, Default)]
pub struct SpellingNote {
    spelling: String,
    pub word: String,
    pub picture: String,
    pub audio: String,
    pub ipa: String,
}

impl FromLine for SpellingNote {
    fn from_line(line: &str, separator: char) -> Self {
        let mut note = Self::default();
        let mut fields = line.split(separator);

        let Some(field) = fields.next() else {
            return note;
        };
        note.spelling = field.to_string();

        let Some(field) = fields.next() else {
            return note;
        };
        note.word = field.to_string();

        let Some(field) = fields.next() else {
            return note;
        };
        note.picture = field.to_string();

        let Some(field) = fields.next() else {
            return note;
        };
        note.audio = field.to_string();

        let Some(field) = fields.next() else {
            return note;
        };
        note.ipa = field.to_string();

        note
    }
}

impl ToLine for SpellingNote {
    fn to_line(self, separator: char) -> String {
        vec![self.spelling, self.word, self.picture, self.audio, self.ipa]
            .join(&separator.to_string())
    }
}
