use syn::ItemImpl;

const NAMED_FIELD_STRUCT: &str = r"struct Named {
    field1: String,
    woop: String,
}";

const TUPLE_STRUCT: &str = r"struct Tuple (
    String,
    String,
);";

const EMPTY_STRUCT: &str = r"struct Empty {}";

#[test]
fn named_field_struct_compiles() {
    let ast = syn::parse_str(NAMED_FIELD_STRUCT).unwrap();
    let output = super::impl_note_macro(ast);
    println!("{output}");
    assert!(syn::parse2::<ItemImpl>(output).is_ok());
}

#[test]
fn tuple_struct_compiles() {
    let ast = syn::parse_str(TUPLE_STRUCT).unwrap();
    let output = super::impl_note_macro(ast);
    println!("{output}");
    assert!(syn::parse2::<ItemImpl>(output).is_ok());
}

#[test]
#[should_panic]
fn empty_struct_doesnt_compile() {
    let ast = syn::parse_str(EMPTY_STRUCT).unwrap();
    let _output = super::impl_note_macro(ast);
}
