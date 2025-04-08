pub struct FieldInfo {
    pub separator: char,
    pub header: String,
}

pub(crate) fn extract_header(data: &str) -> String {
    data.lines()
        .take_while(|line| line.starts_with('#'))
        .map(|line| line.to_string() + "\n")
        .collect()
}

pub(crate) fn find_header_entry(
    data: &str,
    key: &str,
) -> Result<String, crate::Error> {
    let pattern = format!("#{key}:");
    Ok(data
        .lines()
        .find_map(|line| line.strip_prefix(&pattern))
        .ok_or(crate::Error::HeaderEntryNotFound(key.to_string()))?
        .to_string())
}

pub(crate) fn parse_separator(value: &str) -> char {
    if value.len() == 1 {
        return value.chars().next().expect("Length should be 1 (checked)");
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
