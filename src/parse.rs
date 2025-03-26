use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub trait FromLine {
    fn from_line(line: &str, separator: char) -> Self;
}

pub trait ToLine {
    fn to_line(self, separator: char) -> String;
}

pub struct FieldInfo {
    pub separator: char,
    pub header: String,
}

pub fn parse_notes<T: FromLine>(path: &str) -> (Vec<T>, FieldInfo) {
    let data = fs::read_to_string(path).unwrap();
    let header = extract_header(&data);
    let separator = find_header_entry(&data, "separator").unwrap();
    let separator = parse_separator(separator);
    let mut lines: Vec<_> = data
        .lines()
        .skip_while(|line| line.starts_with("#"))
        .collect();
    lines.sort();
    let notes = lines
        .into_iter()
        .map(|line| T::from_line(line, separator))
        .collect();

    (notes, FieldInfo { separator, header })
}

fn extract_header(data: &String) -> String {
    data.lines()
        .take_while(|line| line.starts_with("#"))
        .map(|line| line.to_string() + "\n")
        .collect()
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
