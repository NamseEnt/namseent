use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn document(
    attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attribute_args = parse_macro_input!(attribute_input as syn::AttributeArgs);

    let no_serialize = attribute_args.iter().any(|arg| {
        if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = arg {
            path.is_ident("no_serialize")
        } else {
            false
        }
    });
    let no_deserialize = attribute_args.iter().any(|arg| {
        if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = arg {
            path.is_ident("no_deserialize")
        } else {
            false
        }
    });

    let mut derive_input = parse_macro_input!(input as syn::DeriveInput);

    let mut derives = vec!["Debug", "Clone"];
    if !no_serialize {
        derives.push("serde::Serialize");
    }
    if !no_deserialize {
        derives.push("serde::Deserialize");
    }
    derives.into_iter().for_each(|trait_name| {
        let trait_ident: TokenStream = trait_name.parse().unwrap();
        let is_already_derived = derive_input.attrs.iter().any(|attr| {
            attr.path.is_ident("derive")
                && attr.tokens.to_string().contains(&trait_ident.to_string())
        });
        if !is_already_derived {
            derive_input
                .attrs
                .push(syn::parse_quote!(#[derive(#trait_ident)]));
        }
    });

    // #[derive()]

    let syn::Data::Struct(struct_input) = &mut derive_input.data else {
        panic!("Document can only be derived on structs");
    };

    let mut pk_fields = vec![];
    let mut sk_fields = vec![];

    for field in &mut struct_input.fields {
        enum FieldKind {
            PK,
            SK,
        }
        let kind = field.attrs.iter().find_map(|attr| {
            if attr.path.is_ident("pk") {
                Some(FieldKind::PK)
            } else if attr.path.is_ident("sk") {
                Some(FieldKind::SK)
            } else {
                None
            }
        });
        field
            .attrs
            .retain(|attr| !attr.path.is_ident("pk") && !attr.path.is_ident("sk"));

        match kind {
            Some(FieldKind::PK) => pk_fields.push(field),
            Some(FieldKind::SK) => sk_fields.push(field),
            None => {}
        }
    }

    let struct_ident = &derive_input.ident;

    let prefixed_pk_fields = pk_fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let field_ident = Ident::new(&format!("pk_{}", ident), ident.span());
            let field_type = &field.ty;
            quote! {
                pub #field_ident: #field_type
            }
        })
        .collect::<Vec<_>>();

    let prefixed_sk_fields = sk_fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let field_ident = Ident::new(&format!("sk_{}", ident), ident.span());
            let field_type = &field.ty;
            quote! {
                pub #field_ident: #field_type
            }
        })
        .collect::<Vec<_>>();

    let prefixed_pk = {
        let pk_double_quote_content: TokenStream = ("\"".to_string()
            + &pk_fields
                .iter()
                .map(|field| format!("#{}:{{}}", field.ident.as_ref().unwrap()))
                .collect::<String>()
            + "\"")
            .parse()
            .unwrap();

        let parameters: TokenStream = pk_fields
            .iter()
            .map(|field| format!(", self.pk_{}", field.ident.as_ref().unwrap()))
            .collect::<String>()
            .parse()
            .unwrap();
        quote! {format!(#pk_double_quote_content #parameters)}
    };

    let prefixed_sk = {
        if sk_fields.is_empty() {
            quote! { Option::<String>::None }
        } else {
            let sk_double_quote_content: TokenStream = ("\"".to_string()
                + &sk_fields
                    .iter()
                    .map(|field| format!("#{}:{{}}", field.ident.as_ref().unwrap()))
                    .collect::<String>()
                + "\"")
                .parse()
                .unwrap();

            let parameters: TokenStream = sk_fields
                .iter()
                .map(|field| format!(", self.sk_{}", field.ident.as_ref().unwrap()))
                .collect::<String>()
                .parse()
                .unwrap();
            quote! { Some(format!(#sk_double_quote_content #parameters)) }
        }
    };

    let get_struct_output = {
        let get_struct_ident = Ident::new(&format!("{}Get", struct_ident), struct_ident.span());
        let get_struct_fields = prefixed_pk_fields.iter().chain(prefixed_sk_fields.iter());
        quote! {
            pub struct #get_struct_ident {
                #(#get_struct_fields),*
            }
            impl #get_struct_ident {
                pub async fn run(self) -> Result<#struct_ident, crate::storage::dynamo_db::GetItemError> {
                    let pk = #prefixed_pk;
                    let sk = #prefixed_sk;
                    crate::dynamo_db().get_item::<#struct_ident>(pk, sk).await
                }
            }
        }
    };

    let query_struct_output = {
        let query_struct_ident = Ident::new(&format!("{}Query", struct_ident), struct_ident.span());
        let query_struct_fields = prefixed_pk_fields.iter();
        quote! {
            pub struct #query_struct_ident {
                #(#query_struct_fields),*
            }
            impl #query_struct_ident {
                pub async fn run(self) -> Result<Vec<#struct_ident>, crate::storage::dynamo_db::QueryError> {
                    let pk = #prefixed_pk;
                    crate::dynamo_db().query::<#struct_ident>(pk).await
                }
            }
        }
    };

    let impl_document = {
        let pk = {
            let pk_double_quote_content: TokenStream = ("\"".to_string()
                + &pk_fields
                    .iter()
                    .map(|field| format!("#{}:{{}}", field.ident.as_ref().unwrap()))
                    .collect::<String>()
                + "\"")
                .parse()
                .unwrap();

            let parameters: TokenStream = pk_fields
                .iter()
                .map(|field| format!(", self.{}", field.ident.as_ref().unwrap()))
                .collect::<String>()
                .parse()
                .unwrap();
            quote! {format!(#pk_double_quote_content #parameters)}
        };

        let sk = {
            if sk_fields.is_empty() {
                quote! { Option::<String>::None }
            } else {
                let sk_double_quote_content: TokenStream = ("\"".to_string()
                    + &sk_fields
                        .iter()
                        .map(|field| format!("#{}:{{}}", field.ident.as_ref().unwrap()))
                        .collect::<String>()
                    + "\"")
                    .parse()
                    .unwrap();

                let parameters: TokenStream = sk_fields
                    .iter()
                    .map(|field| format!(", self.{}", field.ident.as_ref().unwrap()))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                quote! { Some(format!(#sk_double_quote_content #parameters)) }
            }
        };

        quote! {
            impl crate::storage::dynamo_db::Document for #struct_ident {
                fn partition_key_prefix() -> &'static str {
                    stringify!(#struct_ident)
                }

                fn partition_key_without_prefix(&self) -> String {
                    #pk
                }

                fn sort_key(&self) -> Option<String> {
                    #sk
                }
            }
        }
    };

    let output = quote! {
        #derive_input
        #impl_document
        #get_struct_output
        #query_struct_output
    };

    output.into()
}
