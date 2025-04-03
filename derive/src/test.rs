const TESTY: &str = r"struct Testy {
    field1: String,
    woop: String,
}";

#[test]
fn testy_works() {
    let ast = syn::parse_str(TESTY).unwrap();
    assert_eq!(
        super::impl_note_macro(ast).to_string(),
        quote::quote! { lol }.to_string()
    );
}
