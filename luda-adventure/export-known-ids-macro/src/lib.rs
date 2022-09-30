use lazy_static::lazy_static;
use proc_macro::{Ident, TokenStream, TokenTree};
use proc_macro_error::{abort, proc_macro_error};
use regex::bytes::Regex;
use std::collections::BTreeMap;

#[proc_macro_error]
#[proc_macro]
pub fn export_known_ids(item: TokenStream) -> TokenStream {
    let mut uuid_name_map = BTreeMap::<String, Ident>::new();
    let mut item_iter = item.into_iter();
    while let Some(name_ident) = item_iter.next() {
        match name_ident {
            TokenTree::Ident(name_ident) => {
                let colon_punct = item_iter.next().unwrap();
                if !check_token_tree_is_colon(&colon_punct) {
                    abort!(colon_punct.span(), "Expected ':'. Found {:?}", colon_punct);
                }

                let uuid_literal = item_iter.next().unwrap();
                if !check_token_tree_is_literal(&uuid_literal) {
                    abort!(
                        uuid_literal.span(),
                        "Expected literal. Found {:?}",
                        uuid_literal
                    );
                }
                if !validate_uuid(uuid_literal.to_string().as_str()) {
                    abort!(uuid_literal.span(), "Invalid uuid");
                }

                let is_duplicated_uuid = uuid_name_map
                    .insert(uuid_literal.to_string(), name_ident)
                    .is_some();
                if is_duplicated_uuid {
                    abort!(uuid_literal.span(), "Duplicated UUID: {}", uuid_literal);
                }
            }
            TokenTree::Punct(_) => continue,
            _ => abort!(name_ident.span(), "Expected ':'. Found {:?}", name_ident),
        }
    }

    create_known_id_exports(uuid_name_map)
}

fn check_token_tree_is_colon(token_tree: &TokenTree) -> bool {
    match token_tree {
        TokenTree::Punct(punct) => punct.as_char() == ':',
        _ => false,
    }
}

fn check_token_tree_is_literal(token_tree: &TokenTree) -> bool {
    match token_tree {
        TokenTree::Literal(_) => true,
        _ => false,
    }
}

fn validate_uuid(uuid: &str) -> bool {
    lazy_static! {
        static ref UUID_REGEX: Regex = Regex::new(
            "^\"[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}\"$"
        )
        .unwrap();
    }
    UUID_REGEX.is_match(uuid.as_ref())
}

fn create_known_id_exports(uuid_name_map: BTreeMap<String, Ident>) -> TokenStream {
    let mut exports_string = "use namui::prelude::uuid;\n".to_string();
    for (uuid, name) in uuid_name_map {
        exports_string
            .push_str(format!("pub const {name}: namui::Uuid = namui::uuid!({uuid});\n").as_str());
    }
    exports_string.parse().unwrap()
}
