use std::fmt::{self, Debug, Display};
use std::{fs, str::FromStr};

fn main() {
    let path = "/home/focus/downloads/Norsk__Pronunciation__Minimal pairs - IPA in word.txt";
    let data = fs::read_to_string(path).unwrap();

    let separator = find_header_entry(&data, "separator").unwrap();
    let separator = parse_separator(separator);
    let html: bool = find_header_entry(&data, "html").unwrap();
    let tags_column: usize = find_header_entry(&data, "tags column").unwrap();
    // TODO: other header keys

    let lines = data.lines().skip_while(|line| line.starts_with("#"));
    let notes: Vec<_> = lines
        .map(|line| Note::from_line(line, separator))
        .map(Note::move_ipas_from_words)
        .map(Note::clean_all)
        .collect();

    for note in notes {
        println!("{note:#?}");
    }
}

#[derive(Default)]
struct Note {
    word1: String,
    audio1: String,
    ipa1: String,
    word2: String,
    audio2: String,
    ipa2: String,
    word3: String,
    audio3: String,
    ipa3: String,
    compare_word3: String,
    tags: String,
}

impl Debug for Note {
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

impl Note {
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

    fn move_ipas_from_words(mut self) -> Self {
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

    fn clean_all(mut self) -> Self {
        self.word1 = clean_html(&self.word1);
        self.ipa1 = clean_html(&self.ipa1);
        self.word2 = clean_html(&self.word2);
        self.ipa2 = clean_html(&self.ipa2);
        self.word3 = clean_html(&self.word3);
        self.ipa3 = clean_html(&self.ipa3);

        self
    }
}

fn split_ipa_from_word(word: &str) -> Option<(String, String)> {
    let (word, ipa) = word.split_once('/')?;
    let ipa = ipa.replace('/', "");

    Some((word.to_string(), ipa))
}

fn clean_html(word: &str) -> String {
    let pattern = regex::Regex::new("<.*?>").unwrap();
    pattern
        .replace_all(word, "")
        .replace("&nbsp;", "")
        .replace("\"", "")
}

fn find_header_entry<T: FromStr>(data: &str, key: &str) -> Option<T>
where
    <T as FromStr>::Err: Debug,
{
    let pattern = format!("#{key}:");
    Some(
        data.lines()
            .find_map(|line| line.strip_prefix(&pattern))?
            .parse()
            .unwrap(),
    )
}

fn parse_separator(value: String) -> char {
    if value.len() == 1 {
        return value.chars().next().unwrap();
    }

    match value.to_lowercase().as_str() {
        "tab" => '\t',
        "space" => ' ',
        "comma" => ',',
        "semicolon" => ';',
        "pipe" => '|',
        "colon" => ':',
        _ => panic!("Unrecognised separator: {value}"),
    }
}
