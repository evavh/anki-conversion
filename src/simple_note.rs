use crate::parse::FromLine;

#[derive(Debug, Default)]
pub struct SimpleNote {
    pub(crate) audio: String,
    pub(crate) word: String,
    pub(crate) tags: String,
}

impl FromLine for SimpleNote {
    fn from_line(line: &str, separator: char) -> Self {
        let mut note = Self::default();
        let mut fields = line.split(separator);

        let Some(field) = fields.next() else {
            return note;
        };
        note.audio = field.to_string();
        let Some(field) = fields.next() else {
            return note;
        };
        note.word = field.to_string();

        let Some(field) = fields.next() else {
            return note;
        };
        note.tags = field.to_string();

        note
    }
}
