use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn schema(
    _attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = input.ident.clone();

    let output = quote! {
        #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #input

        impl document::Document for #struct_name {
            fn name() -> &'static str {
                stringify!(#struct_name)
            }

            fn from_bytes(bytes: Vec<u8>) -> document::Result<Self> {
                unsafe { Ok(rkyv::from_bytes_unchecked(&bytes)?) }
            }

            fn to_bytes(&self) -> document::Result<Vec<u8>> {
                Ok(rkyv::to_bytes::<_, 1024>(self)?.to_vec())
            }
        }

    };

    output.into()
}
