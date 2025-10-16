use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};

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

    let default_derives: [syn::Path; 5] = [
        syn::parse_str("Debug").unwrap(),
        syn::parse_str("Clone").unwrap(),
        syn::parse_str("PartialEq").unwrap(),
        syn::parse_str("bincode::Encode").unwrap(),
        syn::parse_str("bincode::Decode").unwrap(),
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
        syn::Data::Struct(data) => generate_struct_encode(name, data),
        syn::Data::Enum(data) => generate_enum_encode(name, data),
        syn::Data::Union(_) => {
            return syn::Error::new_spanned(input, "State cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let decode_impl = match &input.data {
        syn::Data::Struct(data) => generate_struct_decode(name, data),
        syn::Data::Enum(data) => generate_enum_decode(name, data),
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
    };

    TokenStream::from(expanded)
}

fn generate_struct_encode(_name: &syn::Ident, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    match &data.fields {
        syn::Fields::Named(fields) => {
            let encode_fields = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote! {
                    bincode::Encode::encode(&self.#name, encoder)?;
                }
            });
            quote! {
                #(#encode_fields)*
                Ok(())
            }
        }
        syn::Fields::Unnamed(fields) => {
            let encode_fields = fields.unnamed.iter().enumerate().map(|(i, _)| {
                let index = syn::Index::from(i);
                quote! {
                    bincode::Encode::encode(&self.#index, encoder)?;
                }
            });
            quote! {
                #(#encode_fields)*
                Ok(())
            }
        }
        syn::Fields::Unit => {
            quote! {
                Ok(())
            }
        }
    }
}

fn generate_struct_decode(_name: &syn::Ident, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    match &data.fields {
        syn::Fields::Named(fields) => {
            let decode_fields = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote! {
                    #name: bincode::Decode::decode(decoder)?
                }
            });
            quote! {
                Ok(Self {
                    #(#decode_fields),*
                })
            }
        }
        syn::Fields::Unnamed(fields) => {
            let decode_fields = fields.unnamed.iter().map(|_| {
                quote! {
                    bincode::Decode::decode(decoder)?
                }
            });
            quote! {
                Ok(Self(
                    #(#decode_fields),*
                ))
            }
        }
        syn::Fields::Unit => {
            quote! {
                Ok(Self)
            }
        }
    }
}

fn generate_enum_encode(_name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data.variants.iter().enumerate().map(|(i, variant)| {
        let variant_name = &variant.ident;
        let discriminant = i as u32;

        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let encode_fields = field_names.iter().map(|name| {
                    quote! {
                        bincode::Encode::encode(#name, encoder)?;
                    }
                });
                quote! {
                    Self::#variant_name { #(#field_names),* } => {
                        bincode::Encode::encode(&#discriminant, encoder)?;
                        #(#encode_fields)*
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let field_names: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("field{}", i))
                    .collect();
                let encode_fields = field_names.iter().map(|name| {
                    quote! {
                        bincode::Encode::encode(#name, encoder)?;
                    }
                });
                quote! {
                    Self::#variant_name(#(#field_names),*) => {
                        bincode::Encode::encode(&#discriminant, encoder)?;
                        #(#encode_fields)*
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    Self::#variant_name => {
                        bincode::Encode::encode(&#discriminant, encoder)?;
                    }
                }
            }
        }
    });

    quote! {
        match self {
            #(#variants)*
        }
        Ok(())
    }
}

fn generate_enum_decode(_name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variant_count = data.variants.len();
    let max_discriminant = (variant_count as u32).saturating_sub(1);

    let variants = data.variants.iter().enumerate().map(|(i, variant)| {
        let variant_name = &variant.ident;
        let discriminant = i as u32;

        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_decodes = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote! {
                        #name: bincode::Decode::decode(decoder)?
                    }
                });
                quote! {
                    #discriminant => Ok(Self::#variant_name {
                        #(#field_decodes),*
                    })
                }
            }
            syn::Fields::Unnamed(fields) => {
                let field_decodes = fields.unnamed.iter().map(|_| {
                    quote! {
                        bincode::Decode::decode(decoder)?
                    }
                });
                quote! {
                    #discriminant => Ok(Self::#variant_name(
                        #(#field_decodes),*
                    ))
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #discriminant => Ok(Self::#variant_name)
                }
            }
        }
    });

    quote! {
        let discriminant: u32 = bincode::Decode::decode(decoder)?;
        match discriminant {
            #(#variants,)*
            _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                type_name: core::any::type_name::<Self>(),
                allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: #max_discriminant },
                found: discriminant,
            })
        }
    }
}
