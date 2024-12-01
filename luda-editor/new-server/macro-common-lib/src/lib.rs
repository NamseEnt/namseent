pub use proc_macro2;
use punctuated::Punctuated;
pub use quote;
use quote::*;
pub use syn;
use syn::*;

pub struct RefFielder {
    pub generics: Generics,
    pub generics_without_bounds: Generics,
    pub fields: Vec<Field>,
}

impl RefFielder {
    pub fn new<'a>(fields: impl 'a + IntoIterator<Item = &'a Field>) -> RefFielder {
        let mut output_fields = vec![];
        let mut lifetime_a_used = false;
        let mut generic_params: Punctuated<GenericParam, syn::token::Comma> = Default::default();
        let mut generic_params_without_bounds: Punctuated<GenericParam, syn::token::Comma> =
            Default::default();

        for field in fields {
            let mut field = field.clone();
            let field_name = field.ty.to_token_stream().to_string();

            match field_name.as_str() {
                "String" => {
                    lifetime_a_used = true;
                    field.ty = parse_quote! {&'a str};
                }
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
                | "i128" | "isize" | "SystemTime" | "bool" => {
                    // Copy
                }
                _ => {
                    lifetime_a_used = true;
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
            fields: output_fields,
        }
    }
}
