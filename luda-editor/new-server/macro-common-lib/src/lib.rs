pub use proc_macro2;
use punctuated::Punctuated;
pub use quote;
use quote::*;
pub use syn;
use syn::{spanned::Spanned, *};

pub struct RefFielder {
    pub generics: Generics,
    pub generics_without_bounds: Generics,
    pub fields: Vec<Field>,
    pub fields_without_attr: Vec<Field>,
}

impl RefFielder {
    pub fn new<'a>(fields: impl 'a + IntoIterator<Item = &'a Field>) -> RefFielder {
        let mut output_fields = vec![];
        let mut lifetime_a_used = false;
        let mut next_str_index = 0;
        let mut generic_params: Punctuated<GenericParam, syn::token::Comma> = Default::default();
        let mut generic_params_without_bounds: Punctuated<GenericParam, syn::token::Comma> =
            Default::default();

        for field in fields {
            let mut field = field.clone();
            let field_name = field.ty.to_token_stream().to_string();

            match field_name.as_str() {
                "String" => {
                    lifetime_a_used = true;

                    field
                        .attrs
                        .push(parse_quote! {#[with(serializer::rkyv_with::StrAsString)]});
                    field.ty = parse_quote! {&'a str};
                }
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
                | "i128" | "isize" | "SystemTime" | "bool" => {
                    // Copy
                }
                "Vec < String >" => {
                    lifetime_a_used = true;

                    let str_ident = Ident::new(&format!("Str{}", next_str_index), field.span());
                    next_str_index += 1;

                    field
                        .attrs
                        .push(parse_quote! {#[with(serializer::rkyv_with::StrVec)]});
                    field.ty = parse_quote! {&'a [#str_ident]};

                    generic_params
                        .push(parse_quote! {#str_ident: std::ops::Deref<Target = str> + std::marker::Sync});
                    generic_params_without_bounds.push(parse_quote! {#str_ident});
                }
                _ => {
                    lifetime_a_used = true;

                    field.attrs.push(parse_quote! {#[with(rkyv::with::Inline)]});
                    let ty = field.ty;
                    field.ty = parse_quote! {&'a #ty};
                }
            }

            output_fields.push(field);
        }

        if lifetime_a_used {
            generic_params.insert(0, parse_quote! {'a});
            generic_params_without_bounds.insert(0, parse_quote! {'a});
        }

        RefFielder {
            generics: Generics {
                lt_token: Some(Default::default()),
                params: generic_params,
                gt_token: Some(Default::default()),
                where_clause: None,
            },
            generics_without_bounds: Generics {
                lt_token: Some(Default::default()),
                params: generic_params_without_bounds,
                gt_token: Some(Default::default()),
                where_clause: None,
            },
            fields_without_attr: strip_rkyv_with_attr(output_fields.iter()),
            fields: output_fields,
        }
    }
}

fn strip_rkyv_with_attr<'a>(fields: impl 'a + IntoIterator<Item = &'a Field>) -> Vec<Field> {
    fields
        .into_iter()
        .map(|field| {
            let mut field = field.clone();
            field.attrs.retain(|attr| {
                !attr.path().segments[0]
                    .ident
                    .to_string()
                    .starts_with("with")
            });
            field
        })
        .collect::<Vec<_>>()
}
