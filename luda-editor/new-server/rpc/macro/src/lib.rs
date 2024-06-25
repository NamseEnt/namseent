use quote::quote;

#[proc_macro]
pub fn define_rpc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let rpc = syn::parse_macro_input!(input as rpc_parser::Rpc);

    let define_rpc_meta = define_rpc_meta(&rpc);
    let define_rpc_structs_and_mods = define_rpc_structs_and_mods(&rpc);
    proc_macro::TokenStream::from(quote! {
        #define_rpc_meta
        #define_rpc_structs_and_mods
    })
}

fn define_rpc_meta(rpc: &rpc_parser::Rpc) -> proc_macro2::TokenStream {
    let services = rpc.services.iter().map(|service| {
        let name: &syn::Ident = &service.name;
        let snake_case_name = &service.snake_case_name();
        let apis = service.apis.iter().map(|api| {
            let name = &api.name;
            let items = &api.items;
            quote! {
                Api {
                    name: syn::Ident::new(stringify!(#name), proc_macro2::Span::call_site()),
                    items: vec![
                        #(syn::parse_quote!(#items),)*
                    ],
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

fn define_rpc_structs_and_mods(rpc: &rpc_parser::Rpc) -> proc_macro2::TokenStream {
    let services = rpc.services.iter().map(|service| {
        let service_name = service.snake_case_name();
        let apis = service.apis.iter().map(|api| {
            let api_name = &api.name;
            let items = api.items.iter().map(|item| {
                quote! {
                    #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
                    #[archive(check_bytes)]
                    #item
                }
            });
            quote! {
                pub mod #api_name {
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
