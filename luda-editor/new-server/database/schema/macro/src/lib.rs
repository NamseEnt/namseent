mod parsed;

use parsed::*;
use quote::{quote, ToTokens};
use spanned::Spanned;
use syn::*;

#[proc_macro_attribute]
pub fn schema(
    _attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    let parsed = Parsed::new(&input);
    let struct_name = &input.ident;

    let ref_struct = parsed.ref_struct();

    let struct_get_define = struct_get_define(&parsed);
    let struct_put_define = struct_put_define(&parsed);
    let struct_create_define = struct_create_define(&parsed);
    let struct_delete_define = struct_delete_define(&parsed);
    let struct_query_define = struct_query_define(&parsed);

    let attrs_removed_input = &parsed.attrs_removed_input;

    let output = quote! {
        #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #attrs_removed_input

        impl document::Document for #struct_name {
            fn name() -> &'static str {
                stringify!(#struct_name)
            }

            fn heap_archived(value_buffer: document::ValueBuffer) -> document::HeapArchived<Self> {
                document::HeapArchived::new(value_buffer)
            }

            fn from_bytes(bytes: Vec<u8>) -> document::Result<Self> {
                unsafe { Ok(rkyv::from_bytes_unchecked(&bytes)?) }
            }

            fn to_bytes(&self) -> document::Result<Vec<u8>> {
                Ok(rkyv::to_bytes::<_, 1024>(self)?.to_vec())
            }
        }

        #ref_struct

        #struct_get_define
        #struct_put_define
        #struct_create_define
        #struct_delete_define
        #struct_query_define
    };

    output.into()
}

fn struct_get_define(parsed: &Parsed) -> impl quote::ToTokens {
    let Parsed {
        name,
        pk_cow,
        sk_cow,
        pk_sk_ref_fields,
        ..
    } = parsed;
    let get_struct_name = Ident::new(&format!("{}Get", name), name.span());

    quote! {
        pub struct #get_struct_name<'a> {
            #(#pk_sk_ref_fields),*
        }
        impl document::DocumentGet for #get_struct_name<'_> {
            type Output = #name;

            fn pk<'a>(&'a self) -> std::borrow::Cow<'a, [u8]> {
                #pk_cow
            }
            fn sk<'a>(&'a self) -> Option<std::borrow::Cow<'a, [u8]>> {
                #sk_cow
            }
        }
    }
}

fn struct_put_define(parsed: &Parsed) -> impl quote::ToTokens {
    let Parsed {
        name,
        fields_without_pksk_attr,
        ref_struct_value,
        pk_cow,
        sk_cow,
        ..
    } = parsed;
    let put_struct_name = Ident::new(&format!("{}Put", name), name.span());
    let ref_fields = as_ref_fields(fields_without_pksk_attr);

    quote! {
        pub struct #put_struct_name<'a> {
            #(#ref_fields,)*
            pub ttl: Option<std::time::Duration>,
        }

        impl<'a> TryInto<document::TransactItem<'a>> for #put_struct_name<'a> {
            type Error = document::SerErr;
            fn try_into(self) -> Result<document::TransactItem<'a>, document::SerErr> {
                Ok(document::TransactItem::Put {
                    name: stringify!(#name),
                    pk: #pk_cow,
                    sk: #sk_cow,
                    value: #ref_struct_value,
                    ttl: self.ttl
                })
            }
        }
    }
}

fn struct_create_define(
    Parsed {
        name,
        fields_without_pksk_attr,
        pk_cow,
        sk_cow,
        ref_struct_value,
        ..
    }: &Parsed,
) -> impl quote::ToTokens {
    let create_struct_name = Ident::new(&format!("{}Create", name), name.span());
    let fields_as_refs = as_ref_fields(fields_without_pksk_attr);

    quote! {
        pub struct #create_struct_name<'a> {
            #(#fields_as_refs,)*
            pub ttl: Option<std::time::Duration>,
        }
        impl<'a> TryInto<document::TransactItem<'a>> for #create_struct_name<'a> {
            type Error = document::SerErr;
            fn try_into(self) -> Result<document::TransactItem<'a>, document::SerErr> {


                Ok(document::TransactItem::Create {
                    name: stringify!(#name),
                    pk: #pk_cow,
                    sk: #sk_cow,
                    value: #ref_struct_value,
                    ttl: self.ttl
                })
            }
        }
    }
}

fn struct_delete_define(
    Parsed {
        name,
        pk_cow,
        sk_cow,
        pk_sk_ref_fields,
        ..
    }: &Parsed,
) -> impl quote::ToTokens {
    let delete_struct_name = Ident::new(&format!("{}Delete", name), name.span());

    quote! {
        pub struct #delete_struct_name<'a> {
            #(#pk_sk_ref_fields),*
        }
        impl<'a> TryInto<document::TransactItem<'a>> for #delete_struct_name<'a> {
            type Error = document::SerErr;
            fn try_into(self) -> Result<document::TransactItem<'a>, document::SerErr> {
                Ok(document::TransactItem::Delete {
                    name: stringify!(#name),
                    pk: #pk_cow,
                    sk: #sk_cow,
                })
            }
        }
    }
}

fn struct_query_define(parsed: &Parsed) -> impl quote::ToTokens {
    let Parsed {
        name,
        pk_cow,
        pk_ref_fields,
        ..
    } = parsed;
    let query_struct_name = Ident::new(&format!("{}Query", name), name.span());

    quote! {
        pub struct #query_struct_name<'a> {
            #(#pk_ref_fields),*
        }
        impl document::DocumentQuery for #query_struct_name<'_> {
            type Output = #name;

            fn pk<'a>(&'a self) -> std::borrow::Cow<'a, [u8]> {
                #pk_cow
            }
        }
    }
}

fn as_ref_fields_with_rkyv_with_attr<'a>(
    fields: impl 'a + IntoIterator<Item = &'a Field>,
) -> Vec<Field> {
    fields
        .into_iter()
        .map(|field| {
            let mut field = field.clone();
            if field.ty.to_token_stream().to_string() == "String" {
                field.ty = parse_quote! {&'a str};
                field
                    .attrs
                    .push(parse_quote! {#[with(rkyv::with::RefAsBox)]});
            } else {
                let ty = field.ty;
                field.ty = parse_quote! {&'a #ty};
                field.attrs.push(parse_quote! {#[with(rkyv::with::Inline)]});
            }

            field
        })
        .collect::<Vec<_>>()
}

fn as_ref_fields<'a>(fields: impl 'a + IntoIterator<Item = &'a Field>) -> Vec<Field> {
    as_ref_fields_with_rkyv_with_attr(fields)
        .into_iter()
        .map(|field| {
            let mut field = field.clone();
            field
                .attrs
                .retain(|attr| !attr.path.segments[0].ident.to_string().starts_with("with"));
            field
        })
        .collect::<Vec<_>>()
}
