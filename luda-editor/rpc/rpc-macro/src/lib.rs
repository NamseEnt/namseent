mod services;

use proc_macro::*;
use services::*;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_rpc(tokens: TokenStream) -> TokenStream {
    let services = parse_macro_input!(tokens as Services);
    // todo!("{}", services.to_tokens().to_string())
    services.to_tokens().into()
}
