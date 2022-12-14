use super::*;

pub fn server_handle_rpc(services: &Services) -> proc_macro2::TokenStream {
    let service_parameters = services.services.iter().map(|service| {
        let service_name_in_snake_case = to_snake_case(&service.name);
        let service_name_in_pascal_case = &service.name;
        quote! {
            #service_name_in_snake_case: &impl #service_name_in_pascal_case<TSession>,
        }
    });

    let query_matches = services.services.iter().flat_map(|service| {
        let service_name_in_snake_case = to_snake_case(&service.name);
        service.methods.iter().map(move |method| {
            let method_name = match &method {
                Method::QueueMethod(method) => &method.name,
                Method::RequestAndResponseMethod(method) => &method.name,
            };

            quote! {
                query if query == stringify!(#method_name) => {
                    let request = serde_json::from_slice::<crate::#service_name_in_snake_case::#method_name::Request>(&body);
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
        })
    });

    quote! {
        pub async fn handle_rpc<'a, TSession>(
            request: hyper::Request<hyper::Body>,
            response_builder: hyper::http::response::Builder,
            #(#service_parameters),*
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
                #(#query_matches),*
                _ => {
                    return Ok(response_builder
                        .status(hyper::StatusCode::BAD_REQUEST)
                        .body(hyper::Body::from("Unknown query"))
                        .unwrap());
                }
            }
        }
    }
}
