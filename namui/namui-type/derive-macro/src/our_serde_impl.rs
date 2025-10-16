use quote::{format_ident, quote};

pub fn generate_struct_serialize(_data: &syn::DataStruct) -> proc_macro2::TokenStream {
    quote! {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
}

pub fn generate_struct_serialize_without_name(data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let serialize_fields = match &data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let field_name = &f.ident;
                let field_name_str = field_name.as_ref().unwrap().to_string();
                quote! {
                    buf.write_string(#field_name_str);
                    self.#field_name.serialize_without_name(buf);
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
                    self.#index.serialize_without_name(buf);
                }
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    quote! {
        #(#serialize_fields)*
    }
}

pub fn generate_struct_deserialize(_data: &syn::DataStruct) -> proc_macro2::TokenStream {
    quote! {
        buf.read_name(std::any::type_name::<Self>())?;
        Self::deserialize_without_name(buf)
    }
}

pub fn generate_struct_deserialize_without_name(data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let output = match &data.fields {
        syn::Fields::Named(fields) => {
            let deserialize_fields = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                let field_name_str = field_name.as_ref().unwrap().to_string();
                quote! {
                    let field_name = buf.read_name(#field_name_str)?;
                    let #field_name = Deserialize::deserialize_without_name(buf)?;
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
                        let #field_name = Deserialize::deserialize_without_name(buf)?;
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
        #output
    }
}

pub fn generate_enum_serialize(_data: &syn::DataEnum) -> proc_macro2::TokenStream {
    quote! {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
}

pub fn generate_enum_serialize_without_name(data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();

        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_names = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap().clone());
                let serialize_fields = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_name_str = field_name.as_ref().unwrap().to_string();
                    quote! {
                        buf.write_string(#field_name_str);
                        #field_name.serialize_without_name(buf);
                    }
                });

                quote! {
                    Self::#variant_name { #(#field_names),* } => {
                        buf.write_string(#variant_name_str);
                        #(#serialize_fields)*
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let field_names: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("field{}", i))
                    .collect();
                let serialize_fields = field_names
                    .iter()
                    .map(|field_name| {
                        quote! {
                            #field_name.serialize_without_name(buf);
                        }
                    })
                    .collect::<Vec<_>>();
                quote! {
                    Self::#variant_name ( #(#field_names),* ) => {
                        buf.write_string(#variant_name_str);
                        #(#serialize_fields)*
                    }
                }
            }
            syn::Fields::Unit => quote! {
                Self::#variant_name => {
                    buf.write_string(#variant_name_str);
                }
            },
        }
    });

    quote! {
        match self {
            #(#variants)*
        }
    }
}

pub fn generate_enum_deserialize(_data: &syn::DataEnum) -> proc_macro2::TokenStream {
    quote! {
        buf.read_name(std::any::type_name::<Self>())?;
        Self::deserialize_without_name(buf)
    }
}

pub fn generate_enum_deserialize_without_name(data: &syn::DataEnum) -> proc_macro2::TokenStream {
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
                        let #field_name = Deserialize::deserialize_without_name(buf)?;
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
                            Deserialize::deserialize_without_name(buf)?
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
        let variant_name = buf.read_string();
        match variant_name.as_ref() {
            #(#variants,)*
            _ => Err(DeserializeError::InvalidEnumVariant {
                expected: std::any::type_name::<Self>().to_string(),
                actual: variant_name,
            })
        }
    }
}
