pub use proc_macro2;
pub use quote;
use quote::*;
pub use syn;
use syn::*;

pub fn as_ref_fields_with_rkyv_with_attr<'a>(
    fields: impl 'a + IntoIterator<Item = &'a Field>,
) -> Vec<Field> {
    fields
        .into_iter()
        .map(|field| {
            let mut field = field.clone();
            let field_name = field.ty.to_token_stream().to_string();

            match field_name.as_str() {
                "String" => {
                    field.ty = parse_quote! {&'a str};
                    field
                        .attrs
                        .push(parse_quote! {#[with(serializer::rkyv_with::StrAsString)]});
                }
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64"
                | "i128" | "isize" | "SystemTime" | "bool" => {
                    // Copy
                }
                _ => {
                    let ty = field.ty;
                    field.ty = parse_quote! {&'a #ty};
                    field.attrs.push(parse_quote! {#[with(rkyv::with::Inline)]});
                }
            }

            field
        })
        .collect::<Vec<_>>()
}

pub fn as_ref_fields<'a>(fields: impl 'a + IntoIterator<Item = &'a Field>) -> Vec<Field> {
    as_ref_fields_with_rkyv_with_attr(fields)
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
