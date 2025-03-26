use super::clean_html;

use crate::parse::FromLine;
use crate::simple_note::SimpleNote;

use std::fmt;
use std::fmt::Debug;

#[derive(Clone, Default)]
pub(crate) struct MinimalPairNote {
    pub(crate) word1: String,
    pub(crate) audio1: String,
    pub(crate) ipa1: String,
    pub(crate) word2: String,
    pub(crate) audio2: String,
    pub(crate) ipa2: String,
    pub(crate) word3: String,
    pub(crate) audio3: String,
    pub(crate) ipa3: String,
    pub(crate) compare_word3: String,
    pub(crate) tags: String,
}

impl Debug for MinimalPairNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Note");

        let always_fields = debug
            .field("word1", &self.word1)
            .field("audio1", &self.audio1)
            .field("ipa1", &self.ipa1)
            .field("word2", &self.word2)
            .field("audio2", &self.audio2)
            .field("ipa2", &self.ipa2);

        let with_word3_if_nonempty = if !self.compare_word3.is_empty() {
            always_fields
                .field("word3", &self.word3)
                .field("audio3", &self.audio3)
                .field("ipa3", &self.ipa3)
        } else {
            always_fields
        };

        if self.tags.is_empty() {
            with_word3_if_nonempty.finish()
        } else {
            with_word3_if_nonempty.field("tags", &self.tags).finish()
        }
    }
}

impl FromLine for MinimalPairNote {
    fn from_line(line: &str, separator: char) -> Self {
        let mut note = Self::default();
        let mut fields = line.split(separator);

        let Some(field) = fields.next() else {
            return note;
        };
        note.word1 = field.to_string();
        let Some(field) = fields.next() else {
            return note;
        };
        note.audio1 = field.to_string();
        let Some(field) = fields.next() else {
            return note;
        };
        note.ipa1 = field.to_string();

        let Some(field) = fields.next() else {
            return note;
        };
        note.word2 = field.to_string();
        let Some(field) = fields.next() else {
            return note;
        };
        note.audio2 = field.to_string();
        let Some(field) = fields.next() else {
            return note;
        };
        note.ipa2 = field.to_string();

        let Some(field) = fields.next() else {
            return note;
        };
        note.word3 = field.to_string();
        let Some(field) = fields.next() else {
            return note;
        };
        note.audio3 = field.to_string();
        let Some(field) = fields.next() else {
            return note;
        };
        note.ipa3 = field.to_string();

        let Some(field) = fields.next() else {
            return note;
        };
        note.compare_word3 = field.to_string();
        let Some(field) = fields.next() else {
            return note;
        };
        note.tags = field.to_string();

        note
    }
}

impl MinimalPairNote {
    pub(crate) fn from_simple_notes(
        simple_note1: &SimpleNote,
        simple_note2: &SimpleNote,
    ) -> Self {
        let mut note = Self::default();

        note.word1 = simple_note1.word.clone();
        note.audio1 = simple_note1.audio.clone();
        note.word2 = simple_note2.word.clone();
        note.audio2 = simple_note2.audio.clone();

        note
    }

    pub(crate) fn to_line(self, separator: char) -> String {
        vec![
            self.word1,
            self.audio1,
            self.ipa1,
            self.word2,
            self.audio2,
            self.ipa2,
            self.word3,
            self.audio3,
            self.ipa3,
            self.compare_word3,
            self.tags,
        ]
        .join(&separator.to_string())
    }

    pub(crate) fn move_ipas_from_words(mut self) -> Self {
        if let Some((word, ipa)) = split_ipa_from_word(&self.word1) {
            self.word1 = word;
            self.ipa1 = ipa;
        }
        if let Some((word, ipa)) = split_ipa_from_word(&self.word2) {
            self.word2 = word;
            self.ipa2 = ipa;
        }
        if let Some((word, ipa)) = split_ipa_from_word(&self.word3) {
            self.word3 = word;
            self.ipa3 = ipa;
        }

        self
    }

    pub(crate) fn clean_all(mut self) -> Self {
        self.word1 = clean_html(&self.word1);
        self.ipa1 = clean_html(&self.ipa1);
        self.word2 = clean_html(&self.word2);
        self.ipa2 = clean_html(&self.ipa2);
        self.word3 = clean_html(&self.word3);
        self.ipa3 = clean_html(&self.ipa3);

        self
    }

    pub(crate) fn merge_duplicates(note1: Self, note2: Self) -> Self {
        assert_eq!(note1.word1, note2.word1);
        assert_eq!(note1.word2, note2.word2);
        assert!(note1.word3.is_empty());
        assert!(note2.word3.is_empty());

        let mut note = Self::default();

        note.word1 = note1.word1;
        note.audio1 = note2.audio1 + &note1.audio1;
        if !note1.ipa1.is_empty() {
            note.ipa1 = note1.ipa1;
        } else if !note2.ipa1.is_empty() {
            note.ipa1 = note2.ipa1;
        }

        note.word2 = note1.word2;
        note.audio2 = note2.audio2 + &note1.audio2;
        if !note1.ipa2.is_empty() {
            note.ipa2 = note1.ipa2;
        } else if !note2.ipa2.is_empty() {
            note.ipa2 = note2.ipa2;
        }

        note
    }

    pub(crate) fn is_duplicate(
        &self,
        other: &MinimalPairNote,
    ) -> bool {
        self.word1 == other.word1
            && self.word2 == other.word2
            && self.word3.is_empty()
            && other.word3.is_empty()
    }
}

fn split_ipa_from_word(word: &str) -> Option<(String, String)> {
    let (word, ipa) = word.split_once('/')?;
    let ipa = ipa.replace('/', "");

    Some((word.to_string(), ipa))
}
