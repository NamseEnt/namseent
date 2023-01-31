use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn document(
    _attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut derive_input = parse_macro_input!(input as syn::DeriveInput);

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

    let get_struct_ident = Ident::new(&format!("{}Get", struct_ident), struct_ident.span());
    let get_struct_fields = pk_fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let field_ident = Ident::new(&format!("pk_{}", ident), ident.span());
            let field_type = &field.ty;
            quote! {
                pub #field_ident: #field_type
            }
        })
        .chain(sk_fields.iter().map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let field_ident = Ident::new(&format!("sk_{}", ident), ident.span());
            let field_type = &field.ty;
            quote! {
                pub #field_ident: #field_type
            }
        }));

    let get_struct_output = quote! {
        pub struct #get_struct_ident {
            #(#get_struct_fields),*
        }
    };

    let get_struct_impl = {
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
                .map(|field| format!(", self.pk_{}", field.ident.as_ref().unwrap()))
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
                    .map(|field| format!(", self.sk_{}", field.ident.as_ref().unwrap()))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                quote! { Some(format!(#sk_double_quote_content #parameters)) }
            }
        };

        quote! {
            impl #get_struct_ident {
                pub async fn run(self) -> Result<#struct_ident, crate::storage::dynamo_db::GetItemError> {
                    let pk = #pk;
                    let sk = #sk;
                    crate::dynamo_db().get_item::<#struct_ident>(pk, sk).await
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
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #derive_input
        #impl_document
        #get_struct_output
        #get_struct_impl
    };

    output.into()
}
