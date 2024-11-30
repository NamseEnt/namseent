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

    fn replace_recursive(field: &mut Field) {
        if field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("recursive"))
        {
            field
                .attrs
                .retain(|attr| !attr.path().is_ident("recursive"));
            field.attrs.push(parse_quote! {
                #[omit_bounds]
            });
            field.attrs.push(parse_quote! {
                #[rkyv(omit_bounds)]
            });
        }
    }

    match &mut input.data {
        Data::Struct(struct_input) => {
            struct_input.fields.iter_mut().for_each(|field| {
                field.vis = Visibility::Public(token::Pub(field.vis.span()));

                replace_recursive(field);
            });
        }
        Data::Enum(enum_input) => {
            enum_input.variants.iter_mut().for_each(|variant| {
                variant.fields.iter_mut().for_each(|field| {
                    replace_recursive(field);
                });
            });
        }
        _ => unreachable!(),
    };

    quote! {
        #[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[rkyv(derive(Debug))]
        #input
    }
}
