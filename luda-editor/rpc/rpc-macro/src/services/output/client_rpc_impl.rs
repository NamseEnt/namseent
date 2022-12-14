use super::*;

pub fn client_rpc_impl(services: &Services) -> proc_macro2::TokenStream {
    let rpc_method_impls = services.services.iter().flat_map(|service| {
        service.methods.iter().map(move |method| {
            let method_name = match &method {
                Method::QueueMethod(method) => &method.name,
                Method::RequestAndResponseMethod(method) => &method.name,
            };
            let service_name_in_snake_case = to_snake_case(&service.name);
            quote! {
                impl Rpc {
                    pub fn #method_name<'a>(
                        &'a self,
                        req: super::#service_name_in_snake_case::#method_name::Request,
                    ) -> crate::RpcFuture<super::#service_name_in_snake_case::#method_name::Result> {
                        pub async fn call<'a>(
                            endpoint: String,
                            session_id: Option<namui::Uuid>,
                            req: super::#service_name_in_snake_case::#method_name::Request,
                        ) -> super::#service_name_in_snake_case::#method_name::Result {
                            let url = format!("{endpoint}/?{method}", 
                                endpoint = endpoint,
                                method = stringify!(#method_name),
                            );
                            let result = namui::network::http::fetch_json::<super::#service_name_in_snake_case::#method_name::Result>(
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
                                Err(error) => Err(super::#service_name_in_snake_case::#method_name::Error::Unknown(error.to_string())),
                            }
                        }
                        crate::RpcFuture {
                            future: Box::new(Box::pin(call(self.endpoint(), self.session_id(), req))),
                        }
                    }
                }
            }
        })
    });

    quote! {
        #(#rpc_method_impls)*
    }
}