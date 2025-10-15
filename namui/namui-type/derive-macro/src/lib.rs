mod bincode_impl;
mod our_serde_impl;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};

///
/// #[type_derives]
///
/// #[type_derives(A, B, -C)]
///
/// #[type_derives(Copy)] // includes Copy
///
/// #[type_derives(-PartialEq)] // excludes PartialEq
///
#[proc_macro_attribute]
pub fn type_derives(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as TypeDerives);
    let item = syn::parse_macro_input!(item as syn::Item);

    let (includes, excludes) = attr
        .type_derives
        .iter()
        .partition::<Vec<_>, _>(|derive| !derive.is_excluded());

    let default_derives: [syn::Path; 4] = [
        syn::parse_str("Debug").unwrap(),
        syn::parse_str("Clone").unwrap(),
        syn::parse_str("PartialEq").unwrap(),
        syn::parse_str("State").unwrap(),
    ];

    let mut type_derives = Vec::new();

    for default_derive in default_derives {
        if !excludes.iter().any(|derive| {
            derive.path.to_token_stream().to_string()
                == default_derive.to_token_stream().to_string()
        }) {
            type_derives.push(default_derive);
        }
    }

    for include in includes {
        type_derives.push(include.path.clone());
    }

    let expanded = quote! {
        #[derive(#( #type_derives ),*)]
        #item
    };

    proc_macro::TokenStream::from(expanded)
}

struct Derive {
    minus: Option<syn::Token![-]>,
    path: syn::Path,
}
impl syn::parse::Parse for Derive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let minus = input.parse()?;
        let path = input.parse()?;

        Ok(Self { minus, path })
    }
}
impl Derive {
    fn is_excluded(&self) -> bool {
        self.minus.is_some()
    }
}

struct TypeDerives {
    type_derives: syn::punctuated::Punctuated<Derive, syn::Token![,]>,
}

impl syn::parse::Parse for TypeDerives {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_derives = input.parse_terminated(Derive::parse, syn::Token![,])?;

        Ok(Self { type_derives })
    }
}

#[proc_macro_derive(State)]
pub fn derive_state(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let encode_impl = match &input.data {
        syn::Data::Struct(data) => bincode_impl::generate_struct_encode(data),
        syn::Data::Enum(data) => bincode_impl::generate_enum_encode(data),
        syn::Data::Union(_) => {
            return syn::Error::new_spanned(input, "State cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let decode_impl = match &input.data {
        syn::Data::Struct(data) => bincode_impl::generate_struct_decode(data),
        syn::Data::Enum(data) => bincode_impl::generate_enum_decode(data),
        syn::Data::Union(_) => {
            return syn::Error::new_spanned(input, "State cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let serialize_impl = match &input.data {
        syn::Data::Struct(data) => our_serde_impl::generate_struct_serialize(data),
        syn::Data::Enum(data) => our_serde_impl::generate_enum_serialize(data),
        syn::Data::Union(_) => {
            return syn::Error::new_spanned(input, "State cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let deserialize_impl = match &input.data {
        syn::Data::Struct(data) => our_serde_impl::generate_struct_deserialize(data),
        syn::Data::Enum(data) => our_serde_impl::generate_enum_deserialize(data),
        syn::Data::Union(_) => {
            return syn::Error::new_spanned(input, "State cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl #impl_generics bincode::Encode for #name #ty_generics #where_clause {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                #encode_impl
            }
        }

        impl #impl_generics bincode::Decode<()> for #name #ty_generics #where_clause {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                #decode_impl
            }
        }

        impl #impl_generics Serialize for #name #ty_generics #where_clause {
            fn serialize(&self) -> Vec<u8> {
                #serialize_impl
            }
        }

        impl #impl_generics Deserialize for #name #ty_generics #where_clause {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                #deserialize_impl
            }
        }
    };

    TokenStream::from(expanded)
}
