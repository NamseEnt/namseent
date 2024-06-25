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
}
impl syn::parse::Parse for Api {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _colon: syn::Token![:] = input.parse()?;
        let content;
        let _brace_token = syn::braced!(content in input);
        let mut items = Vec::new();
        while !content.is_empty() {
            let mut item: syn::Item = content.parse()?;
            let span = item.span();

            match &mut item {
                syn::Item::Const(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Enum(x) => {
                    if x.ident.to_string() == "Error" {
                        let mut fields = Punctuated::new();
                        fields.push(syn::parse_quote!(error: String));
                        x.variants.push(syn::Variant {
                            attrs: Vec::new(),
                            ident: Ident::new("InternalServerError", span),
                            fields: syn::Fields::Named(syn::FieldsNamed {
                                brace_token: syn::token::Brace(span),
                                named: fields,
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
                    x.vis = syn::Visibility::Public(syn::token::Pub(span))
                }
                syn::Item::ExternCrate(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Fn(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Mod(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Static(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Struct(x) => {
                    x.vis = syn::Visibility::Public(syn::token::Pub(span));
                    x.fields.iter_mut().for_each(|field| {
                        field.vis = syn::Visibility::Public(syn::token::Pub(span));
                    });
                }
                syn::Item::Trait(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::TraitAlias(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Type(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Union(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Use(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                _ => todo!(),
            }
            items.push(item);
        }

        Ok(Api { name, items })
    }
}
