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
