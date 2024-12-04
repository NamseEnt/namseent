mod parser;

use macro_common_lib::*;
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
        let apis = service.apis.iter().map(|api| {
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
                        };
                    }
                }
                let ref_item = match item {
                    syn::Item::Struct(item_struct) => {
                        let ident = syn::Ident::new(
                            &format!("Ref{}", item_struct.ident),
                            item_struct.ident.span(),
                        );
                        let RefFielder {
                            generics, fields, ..
                        } = RefFielder::new(&item_struct.fields);
                        quote! {
                            #[derive(Debug, serde::Serialize)]
                            pub struct #ident #generics {
                                #(#fields,)*
                            }
                        }
                    }
                    _ => quote! {},
                };
                quote! {
                    #[derive(Debug, serde::Serialize, serde::Deserialize)]
                    #item
                    #extra

                    #ref_item
                }
            });
            quote! {
                pub mod #api_name {
                    use super::super::*;

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
