use quote::quote;
use rpc_parser::*;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_rpc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let services = parse_macro_input!(input as Rpc);

    let rpc_structs = services.to_rpc_structs();
    let client_output = services.to_client_output();
    let expanded = quote! {
        #rpc_structs
        #client_output
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn handler_query_matching(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let services = parse_macro_input!(input as Rpc);

    // let rpc_structs = services.to_rpc_structs();
    // let server_output = services.to_server_output();
    // let client_output = services.to_client_output();
    let expanded = quote! {
        match query.as_str() {
            "exchange_google_auth_code_to_access_token" => {
                crate::apis::auth::exchange_google_auth_code_to_access_token(
                    body,
                    response_builder,
                    session,
                )
            },
            _ => {
                return Ok(response_builder
                    .status(hyper::StatusCode::BAD_REQUEST)
                    .body(hyper::Body::from("Unknown query"))
                    .unwrap());
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
