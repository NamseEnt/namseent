use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned};

#[proc_macro]
pub fn define_rpc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let services = parse_macro_input!(input as Rpc);

    let rpc_structs = services.to_rpc_structs();
    let server_output = services.to_server_output();
    let client_output = services.to_client_output();
    let expanded = quote! {
        #rpc_structs
        #server_output
        #client_output
    };

    proc_macro::TokenStream::from(expanded)
}

struct Rpc {
    services: Punctuated<Service, syn::Token![,]>,
}
impl Rpc {
    fn to_rpc_structs(&self) -> TokenStream {
        let rpc_structs = self.services.iter().map(|service| service.to_rpc_structs());

        quote! {
            #(#rpc_structs)*
        }
    }

    fn to_server_output(&self) -> TokenStream {
        let method_defines = self
            .services
            .iter()
            .map(|service| service.to_method_defines());

        let service_params = self.services.iter().map(|service| {
            let service_name = &service.name;
            let service_name_in_snake_case = syn::Ident::new(
                &service.name.to_string().to_case(Case::Snake),
                service.name.span(),
            );
            quote! {
                #service_name_in_snake_case: &impl #service_name<TSession>,
            }
        });

        let query_matches = self.services.iter().map(|service| {
            let service_name_in_snake_case = syn::Ident::new(&service.name.to_string().to_case(Case::Snake), service.name.span());
            let method_matches = service.apis.iter().map(|method| {
                let method_name = &method.name;
                let method_name_in_snake_case = syn::Ident::new(&method.name.to_string().to_case(Case::Snake), method.name.span());
                quote! {
                    query if query == stringify!(#method_name) => {
                        let request = serde_json::from_slice::<super::#method_name_in_snake_case::Request>(&body);
                        if let Err(error) = request {
                            return Ok(response_builder
                                .status(hyper::StatusCode::BAD_REQUEST)
                                .body(hyper::Body::from(error.to_string()))
                                .unwrap());
                        }
                        let request = request.unwrap();
                        let response = #service_name_in_snake_case.#method_name(session, request).await;
                        let body = serde_json::to_string(&response).unwrap();
                        Ok(response_builder
                            .status(hyper::StatusCode::OK)
                            .body(hyper::Body::from(body))
                            .unwrap())
                    }
                }
            });
            quote! {
                #(#method_matches)*
            }
        });

        quote! {
            #[cfg(feature = "server")]
            mod server {
                pub use hyper;
                #(#method_defines)*

                pub async fn handle_rpc<'a, TSession>(
                    request: hyper::Request<hyper::Body>,
                    response_builder: hyper::http::response::Builder,
                    #(#service_params)*
                    session: Option<TSession>,
                ) -> Result<hyper::Response<hyper::Body>, Box<dyn std::error::Error + Send + Sync>> {
                    let query = request.uri().query();
                    if query.is_none() {
                        return Ok(response_builder
                            .status(hyper::StatusCode::BAD_REQUEST)
                            .body(hyper::Body::from("No query"))
                            .unwrap());
                    }
                    let query = query.unwrap().to_string();

                    let body = hyper::body::to_bytes(request.into_body()).await;
                    if let Err(error) = body {
                        return Ok(response_builder
                            .status(hyper::StatusCode::BAD_REQUEST)
                            .body(hyper::Body::from(error.to_string()))
                            .unwrap());
                    }
                    let body = body.unwrap();

                    match query {
                        #(#query_matches)*
                        _ => {
                            return Ok(response_builder
                                .status(hyper::StatusCode::BAD_REQUEST)
                                .body(hyper::Body::from("Unknown query"))
                                .unwrap());
                        }
                    }
                }
            }
            #[cfg(feature = "server")]
            pub use server::*;
        }
    }

    fn to_client_output(&self) -> TokenStream {
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
struct Service {
    name: syn::Ident,
    _colon: syn::Token![:],
    _brace_token: syn::token::Brace,
    apis: Punctuated<Api, syn::Token![,]>,
}
impl Service {
    fn to_rpc_structs(&self) -> TokenStream {
        let rpc_structs = self.apis.iter().map(|api| api.to_rpc_structs());

        quote! {
            #(#rpc_structs)*
        }
    }

    fn to_method_defines(&self) -> TokenStream {
        let name = &self.name;
        let methods = self.apis.iter().map(|api| api.to_method_defines());
        quote! {
            pub trait #name<TSession> {
                #(#methods)*
            }
        }
    }

    fn to_client_rpc_method_impls(&self) -> TokenStream {
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

///
/// exchange_google_auth_code_to_access_token: {
///     struct Request {
///         pub code: String,
///     }
///     struct Response {}
///     enum Error {
///         Unknown(String)
///     }
/// }
struct Api {
    name: syn::Ident,
    _colon: syn::Token![:],
    _brace_token: syn::token::Brace,
    items: Vec<syn::Item>,
}
impl Api {
    fn to_rpc_structs(&self) -> TokenStream {
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

    fn to_method_defines(&self) -> TokenStream {
        let name = &self.name;
        quote! {
            fn #name<'a>(
                &'a self,
                session: Option<TSession>,
                req: super::#name::Request,
            ) -> std::pin::Pin<
                Box<dyn 'a + std::future::Future<Output = super::#name::Result> + Send>,
            >;
        }
    }

    fn to_client_rpc_method_impl(&self) -> TokenStream {
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
