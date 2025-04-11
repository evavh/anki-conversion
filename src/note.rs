use crate::Error;

pub trait Note: Sized {
    #[must_use]
    fn remove_html(self) -> Self;
    fn into_line(self, separator: char) -> String;
    fn from_line(line: &str, separator: char) -> Result<Self, Error>;
    // Must be like this because of the orphan rule
    // Otherwise we would impl Into<genanki_rs::Note> for types that
    // impl Note
    #[cfg(feature = "genanki-rs")]
    fn into_genanki(
        self,
        model_id: i64,
    ) -> Result<genanki_rs::Note, genanki_rs::Error>;
}
