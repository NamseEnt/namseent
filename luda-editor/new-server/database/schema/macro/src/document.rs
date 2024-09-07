use crate::document_parsed::*;
use macro_common_lib::*;
use quote::quote;
use spanned::Spanned;
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
        pk_sk_ref_fielder,
        ..
    } = parsed;
    let get_struct_name = Ident::new(&format!("{}Get", name), name.span());

    let RefFielder {
        generics,
        generics_without_bounds,
        fields_without_attr,
        ..
    } = pk_sk_ref_fielder;

    quote! {
        pub struct #get_struct_name #generics {
            #(#fields_without_attr,)*
        }
        impl #generics document::DocumentGet for #get_struct_name #generics_without_bounds {
            type Output = #name;

            fn pk<'b>(&'b self) -> document::Result<std::borrow::Cow<'b, [u8]>> {
                Ok(#pk_cow)
            }
            fn sk<'b>(&'b self) -> document::Result<Option<std::borrow::Cow<'b, [u8]>>> {
                Ok(#sk_cow)
            }
        }
    }
}

fn struct_put_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed {
        name,
        ref_struct_value,
        pk_cow,
        sk_cow,
        ref_fielder,
        ..
    } = parsed;
    let put_struct_name = Ident::new(&format!("{}Put", name), name.span());

    let RefFielder {
        generics,
        generics_without_bounds,
        fields_without_attr,
        ..
    } = ref_fielder;

    let try_into_generics = {
        let mut generics = generics.clone();
        generics.params.push(parse_quote! { AbortReason });
        generics
    };

    quote! {
        pub struct #put_struct_name #generics {
            #(#fields_without_attr,)*
            pub ttl: Option<std::time::Duration>,
        }

        impl #try_into_generics TryInto<document::TransactItem<'a, AbortReason>>
            for #put_struct_name #generics_without_bounds
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
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
        pk_cow,
        sk_cow,
        ref_struct_value,
        ref_fielder,
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let create_struct_name = Ident::new(&format!("{}Create", name), name.span());

    let RefFielder {
        generics,
        generics_without_bounds,
        fields_without_attr,
        ..
    } = ref_fielder;

    let try_into_generics = {
        let mut generics = generics.clone();
        generics.params.push(parse_quote! { AbortReason });
        generics
    };

    quote! {
        pub struct #create_struct_name #generics {
            #(#fields_without_attr,)*
            pub ttl: Option<std::time::Duration>,
        }
        impl #try_into_generics TryInto<document::TransactItem<'a, AbortReason>>
            for #create_struct_name #generics_without_bounds
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
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
        pk_cow,
        sk_cow,
        pk_sk_ref_fielder:
            RefFielder {
                generics,
                fields_without_attr,
                ..
            },
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let update_struct_name = Ident::new(&format!("{}Update", name), name.span());

    let mut generics = generics.clone();
    generics.params.push(parse_quote!(AbortReason));
    generics
        .params
        .push(parse_quote!(WantUpdateFn: 'a + Send + FnOnce(&rkyv::Archived<#name>) -> WantUpdate<AbortReason>));
    generics
        .params
        .push(parse_quote!(UpdateFn: 'a + Send + FnOnce(&mut #name)));

    let generics_without_bounds = {
        let mut generics = generics.clone();
        generics.params.iter_mut().for_each(|param| match param {
            GenericParam::Type(param) => {
                param.bounds = Default::default();
            }
            GenericParam::Lifetime(param) => {
                param.bounds = Default::default();
            }
            GenericParam::Const(_param) => {}
        });
        generics
    };

    quote! {
        pub struct #update_struct_name #generics
        {
            #(#fields_without_attr,)*
            pub want_update: WantUpdateFn,
            pub update: UpdateFn,
        }

        impl #generics TryInto<document::TransactItem<'a, AbortReason>>
            for #update_struct_name #generics_without_bounds
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
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
        pk_sk_ref_fielder:
            RefFielder {
                generics,
                generics_without_bounds,
                fields_without_attr,
                ..
            },
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let delete_struct_name = Ident::new(&format!("{}Delete", name), name.span());

    let try_into_generics = {
        let mut generics = generics.clone();
        generics.params.push(parse_quote! { AbortReason });
        generics
    };

    quote! {
        pub struct #delete_struct_name #generics {
            #(#fields_without_attr,)*
        }
        impl #try_into_generics TryInto<document::TransactItem<'a, AbortReason>>
            for #delete_struct_name #generics_without_bounds {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
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
        pk_ref_fielder:
            RefFielder {
                generics,
                generics_without_bounds,
                fields_without_attr,
                ..
            },
        ..
    } = parsed;
    let query_struct_name = Ident::new(&format!("{}Query", name), name.span());

    quote! {
        pub struct #query_struct_name #generics {
            #(#fields_without_attr,)*
        }
        impl #generics document::DocumentQuery for #query_struct_name #generics_without_bounds {
            type Output = #name;

            fn pk<'b>(&'b self) -> document::Result<std::borrow::Cow<'b, [u8]>> {
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
