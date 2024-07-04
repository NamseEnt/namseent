mod parser;

use quote::quote;

#[proc_macro]
pub fn define_rpc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let rpc = syn::parse_macro_input!(input as parser::Rpc);

    let define_rpc_meta = define_rpc_meta(&rpc);
    let define_rpc_structs_and_mods = define_rpc_structs_and_mods(&rpc);
    proc_macro::TokenStream::from(quote! {
        #define_rpc_meta
        #define_rpc_structs_and_mods
    })
}

fn define_rpc_meta(rpc: &parser::Rpc) -> proc_macro2::TokenStream {
    let services = rpc.services.iter().map(|service| {
        let name: &syn::Ident = &service.name;
        let snake_case_name = &service.snake_case_name();
        let apis = service.apis.iter().map(|api| {
            let name = &api.name;
            let items = &api.items;
            let request = &api.request;
            let response = &api.response;
            let error = &api.error;
            quote! {
                Api {
                    name: syn::Ident::new(stringify!(#name), proc_macro2::Span::call_site()),
                    items: vec![
                        #(syn::parse_quote!(#items),)*
                    ],
                    request: syn::parse_quote!(#request),
                    response: syn::parse_quote!(#response),
                    error: syn::parse_quote!(#error),
                }
            }
        });
        quote! {
            Service {
                name: syn::Ident::new(stringify!(#name), proc_macro2::Span::call_site()),
                snake_case_name: syn::Ident::new(stringify!(#snake_case_name), proc_macro2::Span::call_site()),
                apis: vec![
                    #(#apis,)*
                ],
            }
        }
    });

    quote! {
        pub struct Rpc {
            pub services: Vec<Service>,
        }
        pub struct Service {
            pub name: syn::Ident,
            pub snake_case_name: syn::Ident,
            pub apis: Vec<Api>,
        }
        pub struct Api {
            pub name: syn::Ident,
            pub items: Vec<syn::Item>,
            pub request: syn::Item,
            pub response: syn::Item,
            pub error: syn::Item,
        }
        pub fn get_rpc() -> Rpc {
            Rpc {
                services: vec![
                    #(#services,)*
                ]
            }
        }
    }
}

fn define_rpc_structs_and_mods(rpc: &parser::Rpc) -> proc_macro2::TokenStream {
    let services = rpc.services.iter().map(|service| {
        let service_name = service.snake_case_name();
        let apis =
            service.apis.iter().map(|api| {
                let api_name = &api.name;
                let items = api.items.iter().map(|item| {
                let mut extra = quote! {};
                if let syn::Item::Enum(enum_item) = item {
                    if enum_item.ident == "Error" {
                        extra = quote! {
                            impl std::fmt::Display for Error {
                                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                    write!(f, "{:?}", self)
                                }
                            }
                            impl std::error::Error for Error {}

                            #[cfg(feature = "server")]
                            impl From<database::Error> for Error {
                                fn from(e: database::Error) -> Self {
                                    Error::InternalServerError {
                                        err: format!("{e}"),
                                    }
                                }
                            }
                        };
                    }
                }
                let ref_item = match item {
                    syn::Item::Struct(item_struct) => {
                        let mut item_struct = item_struct.clone();
                        item_struct.ident = syn::Ident::new(
                            &format!("Ref{}", item_struct.ident),
                            item_struct.ident.span(),
                        );
                        if !item_struct.fields.is_empty() {
                            item_struct.generics.params.push(syn::parse_quote!('a));
                            item_struct.fields.iter_mut().for_each(|field| {
                                let ty = &field.ty;
                                field.ty = syn::parse_quote!(&'a #ty);
                                field.attrs.push(syn::parse_quote!(#[with(rkyv::with::Inline)]));
                            });
                        }
                        quote! {
                            #[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
                            #[archive(check_bytes)]
                            #item_struct
                        }
                    }
                    syn::Item::Const(_)
                    | syn::Item::Enum(_)
                    | syn::Item::ExternCrate(_)
                    | syn::Item::Fn(_)
                    | syn::Item::ForeignMod(_)
                    | syn::Item::Impl(_)
                    | syn::Item::Macro(_)
                    | syn::Item::Mod(_)
                    | syn::Item::Static(_)
                    | syn::Item::Trait(_)
                    | syn::Item::TraitAlias(_)
                    | syn::Item::Type(_)
                    | syn::Item::Union(_)
                    | syn::Item::Use(_)
                    | syn::Item::Verbatim(_)
                    | _ => quote! {},
                };
                quote! {
                    #[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
                    #[archive(check_bytes)]
                    #item
                    #extra

                    #ref_item
                }
            });
                quote! {
                    pub mod #api_name {
                        use super::super::types::*;

                        #(#items)*
                    }
                }
            });

        quote! {
            pub mod #service_name {
                #(#apis)*
            }
        }
    });

    quote! {
        #(#services)*
    }
}
