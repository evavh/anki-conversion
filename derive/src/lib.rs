use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields, Type};

#[proc_macro_derive(Note)]
pub fn derive_note_fns(input: TokenStream) -> TokenStream {
    // TODO: derive remove_html, from_line and to_line
    let ast = syn::parse(input).unwrap();

    impl_note_macro(ast)
}

fn type_is_string(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };
    let Some(path_term) = type_path.path.segments.last() else {
        return false;
    };
    path_term.ident.to_string() == "String"
}

fn impl_note_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let Data::Struct(struct_data) = ast.data else {
        panic!("Cannot derive Note trait on an enum or union")
    };
    let field_idents: Vec<_> = match struct_data.fields {
        Fields::Named(named_fields) => {
            let fields = named_fields.named;
            fields
                .into_iter()
                .inspect(|f| {
                    assert!(
                        type_is_string(&f.ty),
                        "All Note fields must be of type `String`"
                    )
                })
                .map(|f| {
                    f.ident
                        .expect("Field of a named struct should have a name")
                        .to_string()
                })
                .collect()
        }
        Fields::Unnamed(unnamed_fields) => {
            let n_fields = unnamed_fields.unnamed.len();
            (0..n_fields).into_iter().map(|n| n.to_string()).collect()
        }
        Fields::Unit => panic!("Cannot derive Note trait on a unit struct"),
    };

    dbg!(field_idents);

    let gen = quote! {
        impl Note for #name {
            fn remove_html(mut self) -> Self {
                todo!()
            }

            fn to_line(self, separator: char) -> String {
                todo!()
            }

            fn from_line(line: &str, separator: char) -> Self {
                todo!()
            }
        }
    };
    gen.into()
}
