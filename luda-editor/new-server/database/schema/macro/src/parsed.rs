use crate::{as_ref_fields, as_ref_fields_with_rkyv_with_attr};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::*;

pub struct Parsed<'a> {
    pub name: &'a Ident,
    pub attrs_removed_input: DeriveInput,
    pub fields_without_pksk_attr: Vec<Field>,
    pub ref_struct_name: Ident,
    pub ref_struct_value: TokenStream,
    pub pk_cow: TokenStream,
    pub sk_cow: TokenStream,
    pub pk_sk_ref_fields: Vec<Field>,
    pub pk_ref_fields: Vec<Field>,
}

impl<'a> Parsed<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        let name = &input.ident;
        let mut attrs_removed_input = input.clone();
        let mut pk_fields_without_pk_attr = Vec::new();
        let mut sk_fields_without_sk_attr = Vec::new();
        let mut fields_without_pksk_attr = Vec::new();
        {
            let struct_input = match &mut attrs_removed_input.data {
                Data::Struct(data) => data,
                _ => unreachable!(),
            };
            struct_input.fields.iter_mut().for_each(|field| {
                if field.attrs.iter().any(|attr| attr.path.is_ident("pk")) {
                    field.attrs.retain(|attr| !attr.path.is_ident("pk"));
                    pk_fields_without_pk_attr.push(field.clone());
                }
                if field.attrs.iter().any(|attr| attr.path.is_ident("sk")) {
                    field.attrs.retain(|attr| !attr.path.is_ident("sk"));
                    sk_fields_without_sk_attr.push(field.clone());
                }
                fields_without_pksk_attr.push(field.clone());
            });
        }
        let ref_struct_name = Ident::new(&format!("{}Ref", name), name.span());
        let field_names = fields_without_pksk_attr
            .iter()
            .map(|field| field.ident.as_ref().unwrap().clone())
            .collect::<Vec<_>>();

        let ref_struct_value = quote! {
            {
                use rkyv::ser::{serializers::AllocSerializer, Serializer};
                let mut serializer = AllocSerializer::<1024>::default();
                serializer.serialize_value(&#ref_struct_name{
                    #(
                        #field_names: self.#field_names,
                    )*
                })?;
                serializer.into_serializer().into_inner().to_vec()
            }
        };

        let pk_cow = 'pk_cow: {
            if pk_fields_without_pk_attr.len() == 1 {
                let first_field_name = &pk_fields_without_pk_attr[0].ident;
                let type_name = pk_fields_without_pk_attr[0]
                    .ty
                    .to_token_stream()
                    .to_string();

                if type_name == "String" {
                    break 'pk_cow quote! {
                        self.#first_field_name.as_bytes().into()
                    };
                }

                if [
                    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128",
                    "isize",
                ]
                .contains(&type_name.as_str())
                {
                    break 'pk_cow quote! {
                        self.#first_field_name.to_le_bytes().into()
                    };
                }
            }

            let pk_field_names = pk_fields_without_pk_attr
                .iter()
                .map(|field| field.ident.as_ref().unwrap());
            let pk_ref_fields = as_ref_fields_with_rkyv_with_attr(pk_fields_without_pk_attr.iter());

            quote! {
                {
                    #[derive(rkyv::Archive, rkyv::Serialize)]
                    struct Pk<'a> {
                        #(
                            #pk_ref_fields,
                        )*
                    }
                    let mut serializer = AllocSerializer::<1024>::default();
                    serializer.serialize_value(&Pk{
                        #(
                            #pk_field_names: self.#pk_field_names,
                        )*
                    })?;
                    serializer.into_serializer().into_inner().to_vec().into()
                }
            }
        };

        let sk_cow = 'sk_cow: {
            if sk_fields_without_sk_attr.is_empty() {
                break 'sk_cow quote! {
                    None
                };
            }

            if sk_fields_without_sk_attr.len() == 1 {
                let first_field_name = &sk_fields_without_sk_attr[0].ident;
                let type_name = sk_fields_without_sk_attr[0]
                    .ty
                    .to_token_stream()
                    .to_string();

                if type_name == "String" {
                    break 'sk_cow quote! {
                        Some(self.#first_field_name.as_bytes().into())
                    };
                }

                if [
                    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128",
                    "isize",
                ]
                .contains(&type_name.as_str())
                {
                    break 'sk_cow quote! {
                        Some(self.#first_field_name.to_le_bytes().into())
                    };
                }
            }

            let sk_field_names = sk_fields_without_sk_attr
                .iter()
                .map(|field| field.ident.as_ref().unwrap());
            let sk_ref_fields = as_ref_fields_with_rkyv_with_attr(sk_fields_without_sk_attr.iter());

            quote! {
                {
                    #[derive(rkyv::Archive, rkyv::Serialize)]
                    struct Sk<'a> {
                        #(
                            #sk_ref_fields,
                        )*
                    }
                    let mut serializer = AllocSerializer::<1024>::default();
                    serializer.serialize_value(&Sk{
                        #(
                            #sk_field_names: self.#sk_field_names,
                        )*
                    })?;
                    Some(serializer.into_serializer().into_inner().to_vec().into())
                }
            }
        };
        let pk_sk_ref_fields = {
            let mut pk_sk_ref_fields = vec![];
            pk_sk_ref_fields.extend(as_ref_fields(&pk_fields_without_pk_attr));
            pk_sk_ref_fields.extend(as_ref_fields(&sk_fields_without_sk_attr));
            pk_sk_ref_fields
        };

        let pk_ref_fields = as_ref_fields(&pk_fields_without_pk_attr);

        Self {
            name,
            attrs_removed_input,
            fields_without_pksk_attr,
            ref_struct_name,
            ref_struct_value,
            pk_cow,
            sk_cow,
            pk_sk_ref_fields,
            pk_ref_fields,
        }
    }

    pub(crate) fn ref_struct(&self) -> impl ToTokens {
        let Self {
            fields_without_pksk_attr,
            ref_struct_name,
            ..
        } = self;
        let ref_fields_with_rkyv_with_attr =
            as_ref_fields_with_rkyv_with_attr(fields_without_pksk_attr);
        quote! {
            #[derive(rkyv::Archive, rkyv::Serialize)]
            struct #ref_struct_name<'a> {
                #(
                    #ref_fields_with_rkyv_with_attr,
                )*
            }
        }
    }
}
