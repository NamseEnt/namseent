use macro_common_lib::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, *};

pub struct DocumentPartParsed {
    pub input_redefine: TokenStream,
}

impl DocumentPartParsed {
    pub fn new(input: &DeriveInput) -> Self {
        let input_redefine = input_redefine(input);
        Self { input_redefine }
    }
}

fn input_redefine(input: &DeriveInput) -> TokenStream {
    let mut input = input.clone();
    input.vis = Visibility::Public(token::Pub(input.vis.span()));

    if let Data::Struct(data) = &mut input.data {
        data.fields.iter_mut().for_each(|field| {
            field.vis = Visibility::Public(token::Pub(field.vis.span()));
        });
    }

    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #input
    }
}
