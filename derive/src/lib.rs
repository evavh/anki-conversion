use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{Data, Fields, LitInt, LitStr, Type};

#[cfg(test)]
mod test;

#[proc_macro_derive(Note)]
pub fn derive_note_fns(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_note_macro(ast).into()
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

fn impl_note_macro(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let Data::Struct(struct_data) = ast.data else {
        panic!("Cannot derive Note trait on an enum or union")
    };
    let field_idents: Vec<_> = match struct_data.fields {
        Fields::Named(named_fields) => {
            let fields = named_fields.named;
            assert!(
                !fields.is_empty(),
                "Cannot derive Note trait on an empty struct"
            );
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
                        .to_token_stream()
                })
                .collect()
        }
        Fields::Unnamed(unnamed_fields) => {
            let n_fields = unnamed_fields.unnamed.len();
            (0..n_fields)
                .into_iter()
                .map(|n| {
                    LitInt::new(&n.to_string(), Span::call_site())
                        .to_token_stream()
                })
                .collect()
        }
        Fields::Unit => panic!("Cannot derive Note trait on a unit struct"),
    };

    let html_tag_regex = LitStr::new("<.*?>", Span::call_site());
    let nbsp_html = LitStr::new("&nbsp;", Span::call_site());
    let quote = LitStr::new("\"", Span::call_site());

    quote! {
        impl Note for #name {
            fn remove_html(mut self) -> Self {
                fn remove_html(word: &str) -> String {
                    let pattern = regex::Regex::new(#html_tag_regex).unwrap();
                    pattern
                        .replace_all(word, "")
                        .replace(#nbsp_html, "")
                        .replace(#quote, "")
                }

                #(self.#field_idents = remove_html(&self.#field_idents);)*
                self
            }

            fn to_line(self, separator: char) -> String {
                vec![
                    #(self.#field_idents),*
                ]
                .join(&separator.to_string())
            }

            fn from_line(line: &str, separator: char) -> Self {
                let mut note = Self { #(#field_idents: String::new()),*};
                let mut fields = line.split(separator);

                #(let Some(field) = fields.next() else {
                    return note;
                };
                note.#field_idents = field.to_string();)*

                note
            }
        }
    }
}
