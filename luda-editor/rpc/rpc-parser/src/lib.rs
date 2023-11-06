use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned};

pub struct Rpc {
    pub services: Punctuated<Service, syn::Token![,]>,
}
impl Rpc {
    pub fn to_rpc_structs(&self) -> TokenStream {
        let rpc_structs = self.services.iter().map(|service| service.to_rpc_structs());

        quote! {
            #(#rpc_structs)*
        }
    }

    pub fn to_client_output(&self) -> TokenStream {
        let client_rpc_method_impls = self
            .services
            .iter()
            .map(|service| service.to_client_rpc_method_impls());
        quote! {
            #[cfg(feature = "client")]
            mod client {
                use std::sync::Mutex;

                pub struct RpcSetting {
                    endpoint: String,
                    session_id: Option<crate::Uuid>,
                }

                pub struct Rpc {
                    setting: Mutex<RpcSetting>,
                }

                impl Rpc {
                    pub const fn new(endpoint: String) -> Self {
                        Self {
                            setting: Mutex::new(RpcSetting {
                                endpoint,
                                session_id: None,
                            }),
                        }
                    }
                    pub fn set_session_id(&self, session_id: crate::Uuid) {
                        let mut setting = self.setting.lock().unwrap();
                        setting.session_id.replace(session_id);
                    }
                    pub fn session_id(&self) -> Option<crate::Uuid> {
                        let setting = self.setting.lock().unwrap();
                        setting.session_id
                    }
                    pub fn set_endpoint(&self, endpoint: String) {
                        let mut setting = self.setting.lock().unwrap();
                        setting.endpoint = endpoint;
                    }
                    pub fn endpoint(&self) -> String {
                        let setting = self.setting.lock().unwrap();
                        setting.endpoint.clone()
                    }
                }

                #(#client_rpc_method_impls)*
            }
            #[cfg(feature = "client")]
            pub use client::*;

        }
    }
}

impl syn::parse::Parse for Rpc {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let services = Punctuated::parse_terminated(input)?;
        Ok(Rpc { services })
    }
}

/// ServiceName: {
///   ApiName: { ...},
/// }
///
pub struct Service {
    pub name: syn::Ident,
    _colon: syn::Token![:],
    _brace_token: syn::token::Brace,
    pub apis: Punctuated<Api, syn::Token![,]>,
}
impl Service {
    pub fn to_rpc_structs(&self) -> TokenStream {
        let rpc_structs = self.apis.iter().map(|api| api.to_rpc_structs());

        quote! {
            #(#rpc_structs)*
        }
    }

    pub fn to_method_defines(&self) -> TokenStream {
        let name = &self.name;
        let methods = self.apis.iter().map(|api| api.to_method_defines());
        quote! {
            pub trait #name<TSession> {
                #(#methods)*
            }
        }
    }

    pub fn to_client_rpc_method_impls(&self) -> TokenStream {
        let client_rpc_method_impls = self.apis.iter().map(|api| api.to_client_rpc_method_impl());
        quote! {
            #(#client_rpc_method_impls)*
        }
    }
}

impl syn::parse::Parse for Service {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let colon = input.parse()?;
        let content;
        let brace_token = syn::braced!(content in input);
        let apis = Punctuated::parse_terminated(&content)?;

        Ok(Service {
            name,
            _colon: colon,
            _brace_token: brace_token,
            apis,
        })
    }
}

pub struct Api {
    pub name: syn::Ident,
    _colon: syn::Token![:],
    _brace_token: syn::token::Brace,
    pub items: Vec<syn::Item>,
}
impl Api {
    pub fn to_rpc_structs(&self) -> TokenStream {
        let name = &self.name;
        let items = self.items.iter().map(|item| {
            quote! {
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                #item
            }
        });

        quote! {
            pub mod #name {
                #(#items)*

                simple_error_impl!(Error);
                pub type Result = core::result::Result<Response, Error>;
            }
        }
    }

    pub fn to_method_defines(&self) -> TokenStream {
        let name = &self.name;
        quote! {
            pub fn #name<'a>(
                &'a self,
                session: Option<TSession>,
                req: super::#name::Request,
            ) -> std::pin::Pin<
                Box<dyn 'a + std::future::Future<Output = super::#name::Result> + Send>,
            >;
        }
    }

    pub fn to_client_rpc_method_impl(&self) -> TokenStream {
        let name = &self.name;
        quote! {
            impl Rpc {
                pub fn #name<'a>(
                    &'a self,
                    req: super::#name::Request,
                ) -> crate::RpcFuture<super::#name::Result> {
                    pub async fn call<'a>(
                        endpoint: String,
                        session_id: Option<namui::Uuid>,
                        req: super::#name::Request,
                    ) -> super::#name::Result {
                        let url = format!("{endpoint}/?{method}", method = stringify!(#name),);
                        let result = namui::network::http::fetch_json::<super::#name::Result>(
                            url,
                            namui::network::http::Method::POST,
                            |builder| {
                                let builder = builder
                                    .header("Content-Type", "application/json")
                                    .header("Accept", "application/json");
                                (if let Some(session_id) = session_id {
                                    builder.header("session", session_id.to_string())
                                } else {
                                    builder
                                })
                                .body(serde_json::to_string(&req).unwrap())
                            },
                        )
                        .await;

                        match result {
                            Ok(result) => result,
                            Err(error) => Err(super::#name::Error::Unknown(error.to_string())),
                        }
                    }
                    crate::RpcFuture {
                        future: Box::new(Box::pin(call(self.endpoint(), self.session_id(), req))),
                    }
                }
            }
        }
    }
}

impl syn::parse::Parse for Api {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let colon = input.parse()?;
        let content;
        let brace_token = syn::braced!(content in input);
        let mut items = Vec::new();
        while !content.is_empty() {
            let mut item: syn::Item = content.parse()?;
            let span = item.span();

            match &mut item {
                syn::Item::Const(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Enum(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::ExternCrate(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Fn(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Mod(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Static(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Struct(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Trait(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::TraitAlias(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Type(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Union(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                syn::Item::Use(x) => x.vis = syn::Visibility::Public(syn::token::Pub(span)),
                _ => todo!(),
            }
            items.push(item);
        }

        Ok(Api {
            name,
            _colon: colon,
            _brace_token: brace_token,
            items,
        })
    }
}
