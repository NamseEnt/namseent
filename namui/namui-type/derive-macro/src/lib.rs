use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;

///
/// #[type_derives]
///
/// #[type_derives(A, B, -C)]
///
/// #[type_derives(Copy)] // includes Copy
///
/// #[type_derives(-PartialEq)] // excludes PartialEq
///
#[proc_macro_attribute]
pub fn type_derives(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as TypeDerives);
    let item = syn::parse_macro_input!(item as syn::Item);

    let (includes, excludes) = attr
        .type_derives
        .iter()
        .partition::<Vec<_>, _>(|derive| !derive.is_excluded());

    let default_derives: [syn::Path; 8] = [
        syn::parse_str("Debug").unwrap(),
        syn::parse_str("Clone").unwrap(),
        syn::parse_str("PartialEq").unwrap(),
        syn::parse_str("serde::Serialize").unwrap(),
        syn::parse_str("serde::Deserialize").unwrap(),
        syn::parse_str("rkyv::Archive").unwrap(),
        syn::parse_str("rkyv::Serialize").unwrap(),
        syn::parse_str("rkyv::Deserialize").unwrap(),
    ];

    let mut type_derives = Vec::new();

    for default_derive in default_derives {
        if !excludes.iter().any(|derive| {
            derive.path.to_token_stream().to_string()
                == default_derive.to_token_stream().to_string()
        }) {
            type_derives.push(default_derive);
        }
    }

    for include in includes {
        type_derives.push(include.path.clone());
    }

    let mut extra_attrs: Vec<syn::Attribute> = Vec::new();

    if type_derives.iter().any(|x| {
        x.to_token_stream().to_string()
            == syn::parse_str::<syn::Path>("rkyv::Archive")
                .unwrap()
                .to_token_stream()
                .to_string()
    }) {
        extra_attrs.push(parse_quote! {#[archive_attr(derive(Debug))]});
        extra_attrs.push(parse_quote! {#[archive(check_bytes)]});
        extra_attrs.push(parse_quote! {#[archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))]});
        extra_attrs.push(parse_quote! {#[archive_attr(check_bytes(
            bound = "__C: rkyv::validation::ArchiveContext, <__C as rkyv::Fallible>::Error: std::error::Error"
        ))]});
    }

    let expanded = quote! {
        #[derive(#( #type_derives ),*)]
        #( #extra_attrs )*
        #item
    };

    proc_macro::TokenStream::from(expanded)
}

struct Derive {
    minus: Option<syn::Token![-]>,
    path: syn::Path,
}
impl syn::parse::Parse for Derive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let minus = input.parse()?;
        let path = input.parse()?;

        Ok(Self { minus, path })
    }
}
impl Derive {
    fn is_excluded(&self) -> bool {
        self.minus.is_some()
    }
}

struct TypeDerives {
    type_derives: syn::punctuated::Punctuated<Derive, syn::Token![,]>,
}

impl syn::parse::Parse for TypeDerives {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_derives = input.parse_terminated(Derive::parse, syn::Token![,])?;

        Ok(Self { type_derives })
    }
}
