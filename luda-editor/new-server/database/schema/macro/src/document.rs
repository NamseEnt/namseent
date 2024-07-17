use crate::document_parsed::*;
use macro_common_lib::*;
use quote::quote;
use syn::*;

pub fn document(
    _attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    let parsed = DocumentParsed::new(&input);
    let struct_name = &input.ident;
    let input_redefine = &parsed.input_redefine;

    let ref_struct = parsed.ref_struct();

    let struct_get_define = struct_get_define(&parsed);
    let struct_put_define = struct_put_define(&parsed);
    let struct_create_define = struct_create_define(&parsed);
    let struct_update_define = struct_update_define(&parsed);
    let struct_delete_define = struct_delete_define(&parsed);
    let struct_query_define = struct_query_define(&parsed);
    let debug_define = debug_define(&parsed);

    let output = quote! {
        #input_redefine

        impl document::Document for #struct_name {
            fn name() -> &'static str {
                stringify!(#struct_name)
            }

            fn heap_archived(value_buffer: document::ValueBuffer) -> document::HeapArchived<Self> {
                document::HeapArchived::new(value_buffer)
            }

            fn from_bytes(bytes: Vec<u8>) -> document::Result<Self> {
                document::deserialize(&bytes)
            }

            fn to_bytes(&self) -> document::Result<Vec<u8>> {
                document::serialize(self)
            }
        }

        #ref_struct

        #struct_get_define
        #struct_put_define
        #struct_create_define
        #struct_update_define
        #struct_delete_define
        #struct_query_define

        #debug_define
    };

    output.into()
}

fn struct_get_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed {
        name,
        pk_cow,
        sk_cow,
        pk_sk_ref_fields,
        ..
    } = parsed;
    let get_struct_name = Ident::new(&format!("{}Get", name), name.span());

    quote! {
        pub struct #get_struct_name<'a> {
            #(#pk_sk_ref_fields,)*
        }
        impl document::DocumentGet for #get_struct_name<'_> {
            type Output = #name;

            fn pk<'a>(&'a self) -> document::Result<std::borrow::Cow<'a, [u8]>> {
                Ok(#pk_cow)
            }
            fn sk<'a>(&'a self) -> document::Result<Option<std::borrow::Cow<'a, [u8]>>> {
                Ok(#sk_cow)
            }
        }
    }
}

fn struct_put_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed {
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
            fn try_into(self) -> document::Result<document::TransactItem<'a>> {
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
    DocumentParsed {
        name,
        fields_without_pksk_attr,
        pk_cow,
        sk_cow,
        ref_struct_value,
        ..
    }: &DocumentParsed,
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
            fn try_into(self) -> document::Result<document::TransactItem<'a>> {
                Ok(document::TransactItem::Create {
                    name: stringify!(#name),
                    pk: #pk_cow,
                    sk: #sk_cow,
                    value_fn: Some(Box::new(move || Ok(#ref_struct_value))),
                    ttl: self.ttl,
                })
            }
        }
    }
}

fn struct_update_define(
    DocumentParsed {
        name,
        pk_sk_ref_fields,
        pk_cow,
        sk_cow,
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let update_struct_name = Ident::new(&format!("{}Update", name), name.span());

    quote! {
        pub struct #update_struct_name<'a, WantUpdateFn, UpdateFn>
        where
            WantUpdateFn: 'a + Send + FnOnce(&rkyv::Archived<#name>) -> WantUpdate,
            UpdateFn: 'a + Send + FnOnce(&mut #name),
        {
            #(#pk_sk_ref_fields,)*
            pub want_update: WantUpdateFn,
            pub update: UpdateFn,
        }

        impl<'a, WantUpdateFn, UpdateFn> TryInto<document::TransactItem<'a>>
            for #update_struct_name<'a, WantUpdateFn, UpdateFn>
        where
            WantUpdateFn: 'a + Send + FnOnce(&rkyv::Archived<#name>) -> WantUpdate,
            UpdateFn: 'a + Send + FnOnce(&mut #name),
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a>> {
                Ok(document::TransactItem::Update {
                    name: stringify!(#name),
                    pk: #pk_cow,
                    sk: #sk_cow,
                    update_fn: Some(Box::new(|vec| {
                        let want_update =
                            (self.want_update)(unsafe { rkyv::archived_root::<#name>(vec) });

                        if let WantUpdate::Yes = want_update {
                            let mut doc = deserialize::<#name>(vec)?;
                            (self.update)(&mut doc);
                            *vec = serialize(&doc)?;
                        }

                        Ok(want_update)
                    })),
                })
            }
        }
    }
}

fn struct_delete_define(
    DocumentParsed {
        name,
        pk_cow,
        sk_cow,
        pk_sk_ref_fields,
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let delete_struct_name = Ident::new(&format!("{}Delete", name), name.span());

    quote! {
        pub struct #delete_struct_name<'a> {
            #(#pk_sk_ref_fields,)*
        }
        impl<'a> TryInto<document::TransactItem<'a>> for #delete_struct_name<'a> {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a>> {
                Ok(document::TransactItem::Delete {
                    name: stringify!(#name),
                    pk: #pk_cow,
                    sk: #sk_cow,
                })
            }
        }
    }
}

fn struct_query_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed {
        name,
        pk_cow,
        pk_ref_fields,
        ..
    } = parsed;
    let query_struct_name = Ident::new(&format!("{}Query", name), name.span());

    quote! {
        pub struct #query_struct_name<'a> {
            #(#pk_ref_fields,)*
        }
        impl document::DocumentQuery for #query_struct_name<'_> {
            type Output = #name;

            fn pk<'a>(&'a self) -> document::Result<std::borrow::Cow<'a, [u8]>> {
                Ok(#pk_cow)
            }
        }
    }
}

fn debug_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed { name, .. } = parsed;

    quote! {
        document::inventory::submit! {
            document::DocumentLogPlugin::new(stringify!(#name), |value| {
                let Ok(deserialized) = rkyv::validation::validators::check_archived_root::<
                    #name
                >(value) else {
                    println!("Validation failed");
                    return;
                };
                println!("{:#?}", deserialized);
            })
        }
    }
}
