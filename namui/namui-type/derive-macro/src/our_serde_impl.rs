use quote::{format_ident, quote};

pub fn generate_struct_serialize(data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let serialize_fields = match &data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let field_name = &f.ident;
                let field_name_str = field_name.as_ref().unwrap().to_string();
                quote! {
                    buffer.write_name(#field_name_str);
                    let field_bytes = Serialize::serialize(&self.#field_name);
                    buffer.put_slice(&field_bytes);
                }
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let index = syn::Index::from(i);
                quote! {
                    let field_bytes = Serialize::serialize(&self.#index);
                    buffer.put_slice(&field_bytes);
                }
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    quote! {
        use bytes::BufMut;
        use BufMutExt;
        let mut buffer = vec![];
        buffer.write_name(std::any::type_name::<Self>());
        #(#serialize_fields)*
        buffer
    }
}

pub fn generate_struct_deserialize(data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let output = match &data.fields {
        syn::Fields::Named(fields) => {
            let deserialize_fields = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                let field_name_str = field_name.as_ref().unwrap().to_string();
                quote! {
                    let field_name = buf.read_name(#field_name_str)?;
                    let #field_name = Deserialize::deserialize(buf)?;
                }
            });
            let field_names = fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().clone());
            quote! {
                #(#deserialize_fields)*
                Ok(Self {
                    #(#field_names),*
                })
            }
        }
        syn::Fields::Unnamed(fields) => {
            let deserialize_fields = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let field_name = format_ident!("field{}", i);
                    quote! {
                        let #field_name = Deserialize::deserialize(buf)?;
                    }
                })
                .collect::<Vec<_>>();
            let field_names: Vec<_> = (0..fields.unnamed.len())
                .map(|i| format_ident!("field{}", i))
                .collect();
            quote! {
                #(#deserialize_fields)*
                Ok(Self (
                    #(#field_names),*
                ))
            }
        }
        syn::Fields::Unit => quote! {
            Ok(Self)
        },
    };

    quote! {
        use BufExt;
        buf.read_name(std::any::type_name::<Self>())?;
        #output
    }
}

pub fn generate_enum_serialize(data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();

        let (field_names, serialize_fields) = match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_names: Vec<_> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap().clone())
                    .collect();
                let serialize_fields = fields
                    .named
                    .iter()
                    .map(|f| {
                        let field_name = &f.ident;
                        let field_name_str = field_name.as_ref().unwrap().to_string();
                        quote! {
                            buffer.write_name(#field_name_str);
                            let field_bytes = Serialize::serialize(#field_name);
                            buffer.put_slice(&field_bytes);
                        }
                    })
                    .collect::<Vec<_>>();
                (field_names, serialize_fields)
            }
            syn::Fields::Unnamed(fields) => {
                let field_names: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("field{}", i))
                    .collect();
                let serialize_fields = field_names
                    .iter()
                    .map(|field_name| {
                        quote! {
                            let field_bytes = Serialize::serialize(#field_name);
                            buffer.put_slice(&field_bytes);
                        }
                    })
                    .collect::<Vec<_>>();
                (field_names, serialize_fields)
            }
            syn::Fields::Unit => (vec![], vec![]),
        };

        quote! {
            Self::#variant_name { #(#field_names),* } => {
                buffer.write_name(#variant_name_str);
                #(#serialize_fields)*
            }
        }
    });

    quote! {
        use bytes::BufMut;
        use BufMutExt;
        let mut buffer = vec![];
        buffer.write_name(std::any::type_name::<Self>());
        match self {
            #(#variants)*
        }
        buffer
    }
}

pub fn generate_enum_deserialize(data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();

        match &variant.fields {
            syn::Fields::Named(fields) => {
                let deserialize_fields = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_name_str = field_name.as_ref().unwrap().to_string();
                    quote! {
                        let field_name = buf.read_name(#field_name_str)?;
                        let #field_name = Deserialize::deserialize(buf)?;
                    }
                });
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                quote! {
                    #variant_name_str => {
                        #(#deserialize_fields)*
                        Ok(Self::#variant_name {
                            #(#field_names),*
                        })
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let deserialize_fields = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let field_name = format_ident!("field{}", i);
                    quote! {
                        let #field_name = {
                            use bytes::Buf;
                            let field_len = buf.get_u64() as usize;
                            let field_bytes = &buf[..field_len];
                            buf = &buf[field_len..];
                            Deserialize::deserialize(field_bytes)?
                        };
                    }
                });
                let field_names: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("field{}", i))
                    .collect();
                quote! {
                    #variant_name_str => {
                        #(#deserialize_fields)*
                        Ok(Self::#variant_name(
                            #(#field_names),*
                        ))
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #variant_name_str => Ok(Self::#variant_name)
                }
            }
        }
    });

    quote! {
        use BufExt;
        use bytes::Buf;
        buf.read_name(std::any::type_name::<Self>())?;
        let variant_name = buf.read_name_unknown();
        match variant_name.as_ref() {
            #(#variants,)*
            _ => Err(DeserializeError::InvalidEnumVariant {
                expected: std::any::type_name::<Self>().to_string(),
                actual: variant_name,
            })
        }
    }
}
