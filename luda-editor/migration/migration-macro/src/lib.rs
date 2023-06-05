use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn version(
    attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attribute_args = parse_macro_input!(attribute_input as syn::AttributeArgs);

    let version = attribute_args
        .iter()
        .find_map(|arg| match arg {
            syn::NestedMeta::Lit(syn::Lit::Int(lit)) => Some(lit.base10_parse::<u64>().unwrap()),
            _ => None,
        })
        .expect("version attribute must have a integer version");

    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = input.ident.clone();

    let deserialize = if version == 0 {
        quote! {
            fn deserialize(bytes: &[u8], version: u64) -> Result<Self, bincode::Error> {
                assert_eq!(version, 0);
                bincode::deserialize(bytes)
            }
        }
    } else {
        let previous_v: TokenStream = format!("v{}", version - 1).parse().unwrap();
        quote! {
            fn deserialize(bytes: &[u8], version: u64) -> Result<Self, bincode::Error> {
                if version == #version {
                    bincode::deserialize(bytes)
                } else {
                    let prev = #previous_v::#struct_name::deserialize(bytes, version)?;
                    Ok(Self::migrate(prev))
                }
            }
        }
    };

    let migration_trait_impl = quote! {
        impl migration::Migration for #struct_name {
            fn migration_version() -> u64
            {
                #version
            }
            #deserialize
        }
    };

    let output = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #input
        #migration_trait_impl
    };

    output.into()
}
