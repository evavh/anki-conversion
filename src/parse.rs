use std::fmt::Debug;
use std::str::FromStr;

pub struct FieldInfo {
    pub separator: char,
    pub header: String,
}

pub(crate) fn extract_header(data: &String) -> String {
    data.lines()
        .take_while(|line| line.starts_with("#"))
        .map(|line| line.to_string() + "\n")
        .collect()
}

pub(crate) fn find_header_entry<T: FromStr>(data: &str, key: &str) -> Option<T>
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

pub(crate) fn parse_separator(value: String) -> char {
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
