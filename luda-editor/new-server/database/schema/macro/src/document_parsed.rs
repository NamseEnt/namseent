use macro_common_lib::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, *};

pub struct DocumentParsed<'a> {
    pub name: &'a Ident,
    pub input_redefine: TokenStream,
    pub fields_without_pksk_attr: Vec<Field>,
    pub ref_struct_name: Ident,
    pub ref_struct_value: TokenStream,
    pub pk_cow: TokenStream,
    pub sk_cow: TokenStream,
    pub ref_fielder: RefFielder,
    pub pk_ref_fielder: RefFielder,
    pub pk_sk_ref_fielder: RefFielder,
}

impl<'a> DocumentParsed<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        let name = &input.ident;

        let (pk_fields_without_pk_attr, sk_fields_without_sk_attr, fields_without_pksk_attr) = {
            let struct_input = match &input.data {
                Data::Struct(data) => data,
                _ => unreachable!(),
            };

            let mut pk_fields_without_pk_attr = vec![];
            let mut sk_fields_without_sk_attr = vec![];
            let mut fields_without_pksk_attr = vec![];
            struct_input
                .fields
                .clone()
                .into_iter()
                .for_each(|mut field| {
                    field.vis = Visibility::Public(token::Pub(field.vis.span()));

                    if field.attrs.iter().any(|attr| attr.path().is_ident("pk")) {
                        field.attrs.retain(|attr| !attr.path().is_ident("pk"));
                        pk_fields_without_pk_attr.push(field.clone());
                    }
                    if field.attrs.iter().any(|attr| attr.path().is_ident("sk")) {
                        field.attrs.retain(|attr| !attr.path().is_ident("sk"));
                        sk_fields_without_sk_attr.push(field.clone());
                    }
                    fields_without_pksk_attr.push(field);
                });
            (
                pk_fields_without_pk_attr,
                sk_fields_without_sk_attr,
                fields_without_pksk_attr,
            )
        };

        let input_redefine = input_redefine(input);
        let ref_struct_name = Ident::new(&format!("{}Ref", name), name.span());
        let field_names = fields_without_pksk_attr
            .iter()
            .map(|field| field.ident.as_ref().unwrap().clone())
            .collect::<Vec<_>>();

        let ref_struct_value = quote! {
            {
                document::serialize(&#ref_struct_name{
                    #(
                        #field_names: self.#field_names,
                    )*
                })?
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
            let pk_ref_fields = RefFielder::new(&pk_fields_without_pk_attr).fields;

            quote! {
                {
                    #[derive(rkyv::Archive, rkyv::Serialize)]
                    struct Pk<'a> {
                        #(
                            #pk_ref_fields,
                        )*
                    }
                    document::serialize(&Pk{
                        #(
                            #pk_field_names: self.#pk_field_names,
                        )*
                    })?.into()
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
            let sk_ref_fields = RefFielder::new(&sk_fields_without_sk_attr).fields;

            quote! {
                {
                    #[derive(rkyv::Archive, rkyv::Serialize)]
                    struct Sk<'a> {
                        #(
                            #sk_ref_fields,
                        )*
                    }
                    Some(document::serialize(&Sk{
                        #(
                            #sk_field_names: self.#sk_field_names,
                        )*
                    })?.into())
                }
            }
        };

        Self {
            ref_fielder: RefFielder::new(&fields_without_pksk_attr),
            pk_ref_fielder: RefFielder::new(&pk_fields_without_pk_attr),
            pk_sk_ref_fielder: RefFielder::new(
                pk_fields_without_pk_attr
                    .iter()
                    .chain(&sk_fields_without_sk_attr),
            ),
            name,
            input_redefine,
            fields_without_pksk_attr,
            ref_struct_name,
            ref_struct_value,
            pk_cow,
            sk_cow,
        }
    }

    pub(crate) fn ref_struct(&self) -> impl ToTokens {
        let Self {
            fields_without_pksk_attr,
            ref_struct_name,
            ..
        } = self;
        let RefFielder {
            fields, generics, ..
        } = RefFielder::new(fields_without_pksk_attr);
        quote! {
            #[derive(rkyv::Archive, rkyv::Serialize)]
            struct #ref_struct_name #generics {
                #(#fields,)*
            }
        }
    }
}

fn input_redefine(input: &DeriveInput) -> TokenStream {
    let mut input = input.clone();
    input.vis = Visibility::Public(token::Pub(input.vis.span()));

    let struct_input = match &mut input.data {
        Data::Struct(data) => data,
        _ => unreachable!(),
    };

    struct_input.fields.iter_mut().for_each(|field| {
        field.vis = Visibility::Public(token::Pub(field.vis.span()));
        field
            .attrs
            .retain(|attr| !attr.path().is_ident("pk") && !attr.path().is_ident("sk"));
    });

    quote! {
        #[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive_attr(derive(Debug))]
        #[archive(check_bytes)]
        #input
    }
}
