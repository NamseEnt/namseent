use quote::{format_ident, quote};

pub fn generate_struct_encode(data: &syn::DataStruct) -> proc_macro2::TokenStream {
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

pub fn generate_struct_decode(data: &syn::DataStruct) -> proc_macro2::TokenStream {
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

pub fn generate_enum_encode(data: &syn::DataEnum) -> proc_macro2::TokenStream {
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

pub fn generate_enum_decode(data: &syn::DataEnum) -> proc_macro2::TokenStream {
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
