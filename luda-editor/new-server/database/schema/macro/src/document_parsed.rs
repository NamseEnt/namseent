use macro_common_lib::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, *};

pub struct DocumentParsed<'a> {
    pub name: &'a Ident,
    pub input_redefine: TokenStream,
    pub fields: Vec<Field>,
    pub ref_struct_name: Ident,
    pub ref_struct_value: TokenStream,
    pub ref_fielder: RefFielder,
    pub id_fields: Vec<Field>,
    pub id_field_idents: Vec<Ident>,
    pub id_ref_fielder: RefFielder,
}

impl<'a> DocumentParsed<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        let name = &input.ident;

        let (fields, id_fields) = {
            let struct_input = match &input.data {
                Data::Struct(data) => data,
                _ => unreachable!(),
            };

            let mut fields = match &struct_input.fields {
                Fields::Named(fields_named) => {
                    fields_named.named.clone().into_iter().collect::<Vec<_>>()
                }
                Fields::Unnamed(..) | Fields::Unit => unimplemented!(),
            };

            fields.iter_mut().for_each(|field| {
                field.vis = Visibility::Public(token::Pub(field.vis.span()));
            });

            let mut id_attr_fields = fields
                .iter_mut()
                .filter(|field| field.attrs.iter().any(|attr| attr.path().is_ident("id")))
                .collect::<Vec<_>>();

            let id_fields = if id_attr_fields.is_empty() {
                fields.insert(
                    0,
                    parse_quote! {
                        pub id: u128
                    },
                );
                vec![fields.first().unwrap().clone()]
            } else {
                id_attr_fields.iter_mut().for_each(|field| {
                    field.attrs.retain(|attr| !attr.path().is_ident("id"));
                });

                id_attr_fields.into_iter().map(|x| x.clone()).collect()
            };

            (fields, id_fields)
        };

        let input_redefine = input_redefine(input, &fields);
        let ref_struct_name = Ident::new(&format!("{}Ref", name), name.span());
        let field_names = fields
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
        let id_field_idents = id_fields.iter().map(|x| x.ident.clone().unwrap()).collect();

        Self {
            ref_fielder: RefFielder::new(&fields),
            id_ref_fielder: RefFielder::new(&id_fields),
            name,
            input_redefine,
            fields,
            ref_struct_name,
            ref_struct_value,
            id_fields,
            id_field_idents,
        }
    }

    pub(crate) fn ref_struct(&self) -> impl ToTokens {
        let Self {
            fields,
            ref_struct_name,
            ..
        } = self;
        let RefFielder {
            fields, generics, ..
        } = RefFielder::new(fields);
        quote! {
            #[derive(rkyv::Archive, rkyv::Serialize)]
            struct #ref_struct_name #generics {
                #(#fields,)*
            }
        }
    }
}

fn input_redefine(input: &DeriveInput, fields: &[Field]) -> TokenStream {
    let ident = &input.ident;

    quote! {
        #[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive_attr(derive(Debug))]
        #[archive(check_bytes)]
        pub struct #ident {
            #(#fields,)*
        }
    }
}
