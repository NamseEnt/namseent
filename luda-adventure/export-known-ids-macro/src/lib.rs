use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use regex::bytes::Regex;
use std::collections::BTreeMap;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, token::Colon, Ident, LitStr, Token,
};

#[proc_macro]
pub fn export_known_ids(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);

    let exports = input.uuid_name_map.into_iter().map(|(uuid, name)| {
        quote! {
            pub const #name: namui::Uuid = namui::uuid!(#uuid);
        }
    });

    TokenStream::from(quote! {
        use namui::prelude::uuid;

        #(#exports)*
    })
}

struct MacroInput {
    uuid_name_map: BTreeMap<String, Ident>,
}
impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let uuid_name_pair_list = Punctuated::<NameUuidPair, Token!(,)>::parse_terminated(input)?;
        let mut uuid_name_map = BTreeMap::<String, Ident>::new();
        for NameUuidPair { name, uuid } in uuid_name_pair_list {
            let is_duplicated_uuid = uuid_name_map.insert(uuid.value(), name).is_some();
            if is_duplicated_uuid {
                return Err(syn::Error::new(
                    uuid.span(),
                    format!("Duplicated UUID: {}", uuid.value()),
                ));
            }
        }
        Ok(Self { uuid_name_map })
    }
}

struct NameUuidPair {
    name: Ident,
    uuid: LitStr,
}
impl Parse for NameUuidPair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _colon: Colon = input.parse()?;
        let uuid: LitStr = input.parse()?;
        if !validate_uuid(uuid.value().as_str()) {
            return Err(syn::Error::new(uuid.span(), "Invalid uuid"));
        }
        Ok(Self { name, uuid })
    }
}

fn validate_uuid(uuid: &str) -> bool {
    lazy_static! {
        static ref UUID_REGEX: Regex = Regex::new(
            "^[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}$"
        )
        .unwrap();
    }
    UUID_REGEX.is_match(uuid.as_ref())
}
