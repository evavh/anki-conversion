const TESTY: &str = r"struct Testy {
    field1: String,
    woop: String,
}"

#[test]
fn testy_works() {
    let ast = syn::parse_str(TESTY);
    anki_conversion_derive::impl_note_macro(ast);
}
