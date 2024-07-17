use macro_common_lib::*;
use syn::{punctuated::Punctuated, spanned::Spanned, Ident};

pub struct Rpc {
    pub services: Punctuated<Service, syn::Token![,]>,
}

impl syn::parse::Parse for Rpc {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let services = Punctuated::parse_terminated(input)?;
        Ok(Rpc { services })
    }
}

pub struct Service {
    pub name: syn::Ident,
    pub apis: Punctuated<Api, syn::Token![,]>,
}

impl Service {
    pub fn snake_case_name(&self) -> Ident {
        Ident::new(
            &self
                .name
                .to_string()
                .chars()
                .fold(String::new(), |mut acc, c| {
                    if c.is_uppercase() {
                        if !acc.is_empty() {
                            acc.push('_');
                        }
                        acc.push(c.to_ascii_lowercase());
                    } else {
                        acc.push(c);
                    }
                    acc
                }),
            self.name.span(),
        )
    }
}

impl syn::parse::Parse for Service {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon: syn::Token![:] = input.parse()?;
        let content;
        let _brace_token = syn::braced!(content in input);
        let apis = Punctuated::parse_terminated(&content)?;

        Ok(Service { name, apis })
    }
}

pub struct Api {
    pub name: syn::Ident,
    pub items: Vec<syn::Item>,
    pub request: syn::Item,
    pub response: syn::Item,
    pub error: syn::Item,
}
impl syn::parse::Parse for Api {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon: syn::Token![:] = input.parse()?;
        let content;
        let _brace_token = syn::braced!(content in input);
        let mut items = Vec::new();

        let mut request = None;
        let mut response = None;
        let mut error = None;

        while !content.is_empty() {
            let mut item: syn::Item = content.parse()?;

            match &mut item {
                syn::Item::Const(x) => x.vis = syn::Visibility::Public(syn::token::Pub(x.span())),
                syn::Item::Enum(x) => {
                    if x.ident == "Error" {
                        x.variants.push(syn::Variant {
                            attrs: Vec::new(),
                            ident: Ident::new("InternalServerError", x.span()),
                            fields: syn::Fields::Named(syn::FieldsNamed {
                                brace_token: syn::token::Brace(x.span()),
                                named: {
                                    let mut fields = Punctuated::new();
                                    fields.push(syn::parse_quote!(err: String));
                                    fields
                                },
                            }),
                            discriminant: None,
                        });
                    }
                    if x.variants.is_empty() {
                        return Err(syn::Error::new(
                            x.span(),
                            "Enums must have at least one variant",
                        ));
                    }
                    x.vis = syn::Visibility::Public(syn::token::Pub(x.span()))
                }
                syn::Item::ExternCrate(x) => {
                    x.vis = syn::Visibility::Public(syn::token::Pub(x.span()))
                }
                syn::Item::Fn(x) => x.vis = syn::Visibility::Public(syn::token::Pub(x.span())),
                syn::Item::Mod(x) => x.vis = syn::Visibility::Public(syn::token::Pub(x.span())),
                syn::Item::Static(x) => x.vis = syn::Visibility::Public(syn::token::Pub(x.span())),
                syn::Item::Struct(x) => {
                    x.vis = syn::Visibility::Public(syn::token::Pub(x.span()));
                    let span = x.span();
                    x.fields.iter_mut().for_each(|field| {
                        field.vis = syn::Visibility::Public(syn::token::Pub(span));
                    });
                }
                syn::Item::Trait(x) => x.vis = syn::Visibility::Public(syn::token::Pub(x.span())),
                syn::Item::TraitAlias(x) => {
                    x.vis = syn::Visibility::Public(syn::token::Pub(x.span()))
                }
                syn::Item::Type(x) => x.vis = syn::Visibility::Public(syn::token::Pub(x.span())),
                syn::Item::Union(x) => x.vis = syn::Visibility::Public(syn::token::Pub(x.span())),
                syn::Item::Use(x) => x.vis = syn::Visibility::Public(syn::token::Pub(x.span())),
                _ => todo!(),
            }

            let ident = match &item {
                syn::Item::Enum(item_enum) => Some(&item_enum.ident),
                syn::Item::Struct(item_struct) => Some(&item_struct.ident),
                _ => None,
            };
            if let Some(ident) = ident {
                if ident == "Request" {
                    request = Some(item.clone());
                } else if ident == "Response" {
                    response = Some(item.clone());
                } else if ident == "Error" {
                    error = Some(item.clone());
                }
            }

            items.push(item);
        }

        Ok(Api {
            name,
            items,
            request: request.expect("Request not found"),
            response: response.expect("Response not found"),
            error: error.expect("Error not found"),
        })
    }
}
