use std::fs;
use std::io::Write;

use crate::minimal_pair_note::MinimalPairNote;
use crate::parse::{parse_notes, FieldInfo};
use crate::simple_note::SimpleNote;
use crate::spelling_note::SpellingNote;

mod minimal_pair_note;
mod parse;
mod simple_note;
mod spelling_note;

fn main() {
    convert_ipa_in_word();
    convert_frontback();
    deduplicate();
    convert_spellings();
}

fn convert_spellings() {
    let path =
        "/home/focus/downloads/Norsk__Pronunciation__Spellings and Sounds.txt";
    parse_notes::<SpellingNote>(path);
}

fn deduplicate() {
    let path = "/home/focus/downloads/Norsk__Pronunciation__Minimal Pairs.txt";
    let (notes, field_info) = parse_notes::<MinimalPairNote>(path);

    let n_total = notes.len();
    let n_duplicates = notes
        .windows(2)
        .map(|pair| pair.into_iter().collect::<Vec<_>>())
        .filter(|pair| pair[0].is_duplicate(pair[1]))
        .count();

    let mut note_to_be_checked: Option<MinimalPairNote> = None;
    let mut deduplicated = Vec::new();
    for note in notes {
        assert!(note.word1 < note.word2, "{note:#?}");
        if let Some(prev_note) = note_to_be_checked {
            if note.is_duplicate(&prev_note) {
                let new_note =
                    MinimalPairNote::merge_duplicates(prev_note.clone(), note);
                deduplicated.push(new_note);
                // Both notes have now been processed, so we clear this
                note_to_be_checked = None;
            } else {
                // Prev note is not a duplicate, so we push it as is
                deduplicated.push(prev_note.clone());
                // Current note needs to be checked against the next one
                note_to_be_checked = Some(note);
            }
        } else {
            // Prev note was None, so next time we check the current note
            note_to_be_checked = Some(note);
        }
    }
    // Don't forget the last note!
    if let Some(last_note) = note_to_be_checked {
        deduplicated.push(last_note);
    }

    assert_eq!(deduplicated.len(), n_total - n_duplicates);

    let mut new_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("deduplicated.txt")
        .unwrap();

    let FieldInfo { header, separator } = field_info;
    write!(new_file, "{}", header).unwrap();
    for note in deduplicated {
        writeln!(new_file, "{}", note.to_line(separator)).unwrap();
    }
}

fn convert_frontback() {
    let path = "/home/focus/downloads/Norsk__Pronunciation__Minimal Pairs - frontback.txt";
    let (simple_notes, field_info) = parse_notes::<SimpleNote>(path);

    let pairs = simple_notes.chunks_exact(2);
    assert_eq!(pairs.remainder().len(), 0, "{:?}", pairs.remainder());
    let mut notes: Vec<_> = pairs
        .map(|pair| MinimalPairNote::from_simple_notes(&pair[0], &pair[1]))
        .collect();

    for note in &mut notes {
        if note.word1 == "støt" {
            note.word3 = "stutt".to_owned();
            note.audio3 = "[sound:stutt.mp3]".to_owned();
            note.compare_word3 = "y".to_owned();
        }
    }

    let mut new_file = fs::OpenOptions::new()
        .truncate(true)
        .append(true)
        .create(true)
        .open("frontback_output.txt")
        .unwrap();

    let FieldInfo { header, separator } = field_info;
    write!(
        new_file,
        "{}",
        header.replace("#tags column:3", "#tags column:11")
    )
    .unwrap();
    for note in notes {
        writeln!(new_file, "{}", note.to_line(separator)).unwrap();
    }
}

fn convert_ipa_in_word() {
    let path = "/home/focus/downloads/Norsk__Pronunciation__Minimal pairs - IPA in word.txt";
    let (notes, field_info) = parse_notes::<MinimalPairNote>(path);
    let notes: Vec<_> = notes
        .into_iter()
        .map(MinimalPairNote::move_ipas_from_words)
        .map(MinimalPairNote::clean_all)
        .collect();

    let mut new_file = fs::OpenOptions::new()
        .truncate(true)
        .append(true)
        .create(true)
        .open("ipa_in_word_output.txt")
        .unwrap();

    let FieldInfo { header, separator } = field_info;
    write!(new_file, "{}", header).unwrap();
    for note in notes {
        writeln!(new_file, "{}", note.to_line(separator)).unwrap();
    }
}

fn clean_html(word: &str) -> String {
    let pattern = regex::Regex::new("<.*?>").unwrap();
    pattern
        .replace_all(word, "")
        .replace("&nbsp;", "")
        .replace("\"", "")
}
