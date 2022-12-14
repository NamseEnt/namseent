use super::*;
use quote::ToTokens;
use syn::{parse::Parse, spanned::Spanned, Ident, Token};

pub struct RequestAndResponseMethod {
    pub name: Ident,
    pub request: syn::ItemStruct,
    pub response: syn::ItemStruct,
    pub error: ErrorDef,
    pub other_structs: Vec<syn::ItemStruct>,
    pub other_enums: Vec<syn::ItemEnum>,
}
impl Parse for RequestAndResponseMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let content;
        syn::braced!(content in input);

        let mut request = None;
        let mut response = None;
        let mut error = None;
        let mut other_structs: Vec<syn::ItemStruct> = Vec::new();
        let mut other_enums: Vec<syn::ItemEnum> = Vec::new();

        while !content.is_empty() {
            if content.peek(Ident) {
                error = Some(content.parse::<ErrorDef>()?);
            } else {
                let item = content.parse::<syn::Item>()?;

                match item {
                    syn::Item::Enum(item_enum) => {
                        match item_enum.vis {
                            syn::Visibility::Public(_) => {}
                            _ => {
                                let message = format!(
                                    "expected `pub enum` but found `{}`",
                                    item_enum.vis.to_token_stream().to_string()
                                );
                                return Err(syn::Error::new_spanned(item_enum, message));
                            }
                        };
                        other_enums.push(item_enum);
                    }
                    syn::Item::Struct(item_struct) => {
                        match &item_struct.vis {
                            syn::Visibility::Public(_) => {}
                            _ => {
                                let message = format!(
                                    "expected `pub struct` but found `{}`",
                                    item_struct.vis.to_token_stream().to_string()
                                );
                                return Err(syn::Error::new_spanned(item_struct, message));
                            }
                        };
                        match item_struct.ident.to_string().as_str() {
                            "Request" => {
                                if request.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        item_struct,
                                        "expected only one `Request`",
                                    ));
                                }
                                request = Some(item_struct);
                            }
                            "Response" => {
                                if response.is_some() {
                                    return Err(syn::Error::new_spanned(
                                        item_struct,
                                        "expected only one `Response`",
                                    ));
                                }
                                response = Some(item_struct);
                            }
                            _ => {
                                if other_structs.iter().any(|s| s.ident == item_struct.ident) {
                                    return Err(syn::Error::new_spanned(
                                        item_struct,
                                        "expected only one pub struct with this name",
                                    ));
                                }
                                other_structs.push(item_struct);
                            }
                        }
                    }
                    _ => {
                        let message = format!(
                            "expected `pub enum` or `pub struct` but found `{}`",
                            item.to_token_stream().to_string()
                        );
                        return Err(syn::Error::new_spanned(item, message));
                    }
                }
            }
        }

        let request = request.ok_or(syn::Error::new(
            name.span(),
            "expected `Request` pub struct in method",
        ))?;
        let response = response.ok_or(syn::Error::new(
            request.span(),
            "expected `Response` pub struct in method",
        ))?;
        let error = error.ok_or(syn::Error::new(
            response.span(),
            "expected `Error` pub struct in method",
        ))?;

        Ok(Self {
            name,
            request,
            response,
            error,
            other_structs,
            other_enums,
        })
    }
}
