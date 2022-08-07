use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(target_family = "wasm")]
    fn inner(item: TokenStream) -> TokenStream {
        wasm_bindgen::prelude::wasm_bindgen(item);
    }

    #[cfg(not(target_family = "wasm"))]
    fn inner(item: TokenStream) -> TokenStream {
        // TODO: tokio::main
        item.into()
    }

    inner(item)
}
