use crate::doc_part_parsed::*;
use macro_common_lib::*;
use quote::quote;
use syn::*;

pub fn doc_part(
    _attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    let parsed = DocumentPartParsed::new(&input);
    let input_redefine = &parsed.input_redefine;

    let output = quote! {
        #input_redefine
    };

    output.into()
}
