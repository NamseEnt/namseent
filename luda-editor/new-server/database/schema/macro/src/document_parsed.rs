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
}

impl<'a> DocumentParsed<'a> {
    pub fn new(input: &'a DeriveInput) -> Self {
        let name = &input.ident;

        let fields = {
            let struct_input = match &input.data {
                Data::Struct(data) => data,
                _ => unreachable!(),
            };

            let mut fields = vec![];
            struct_input
                .fields
                .clone()
                .into_iter()
                .for_each(|mut field| {
                    field.vis = Visibility::Public(token::Pub(field.vis.span()));

                    fields.push(field);
                });
            fields
        };

        let input_redefine = input_redefine(input);
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

        Self {
            ref_fielder: RefFielder::new(&fields),
            name,
            input_redefine,
            fields,
            ref_struct_name,
            ref_struct_value,
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

fn input_redefine(input: &DeriveInput) -> TokenStream {
    let mut input = input.clone();
    input.vis = Visibility::Public(token::Pub(input.vis.span()));

    let struct_input = match &mut input.data {
        Data::Struct(data) => data,
        _ => unreachable!(),
    };

    struct_input.fields.iter_mut().for_each(|field| {
        field.vis = Visibility::Public(token::Pub(field.vis.span()));
    });

    quote! {
        #[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #[archive_attr(derive(Debug))]
        #[archive(check_bytes)]
        #input
    }
}
