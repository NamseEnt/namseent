use crate::document_parsed::*;
use crate::to_snake_case::ToSnakeCase;
use macro_common_lib::*;
use quote::quote;
use spanned::Spanned;
use syn::*;

pub fn document(
    _attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    let Data::Struct(data_struct) = &mut input.data else {
        unimplemented!()
    };
    let Fields::Named(fields_named) = &mut data_struct.fields else {
        unimplemented!()
    };

    fields_named.named.insert(
        0,
        parse_quote! {
            id: u128
        },
    );

    if let Some(attr) = input.attrs.iter().find(|attr| {
        matches!(attr.style, AttrStyle::Outer) && attr.meta.path().is_ident("belongs_to")
    }) {
        let meta_list = attr.meta.require_list().unwrap();
        let owner = meta_list.parse_args::<Ident>().unwrap();

        let owner_snake = owner.to_string().to_snake_case();

        let owner_id = Ident::new(&format!("{}_id", owner_snake), owner.span());

        fields_named.named.insert(
            1,
            parse_quote! {
                #owner_id: u128
            },
        );
    }

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

            fn heap_archived(bytes: document::Bytes) -> document::HeapArchived<Self> {
                document::HeapArchived::new(bytes)
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
    let DocumentParsed { name, .. } = parsed;
    let get_struct_name = Ident::new(&format!("{}Get", name), name.span());

    quote! {
        pub struct #get_struct_name {
            pub id: u128,
        }
        impl document::DocumentGet for #get_struct_name {
            type Output = #name;

            fn id(&self) -> u128 {
                self.id
            }
        }
    }
}

fn struct_put_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed {
        name,
        ref_struct_value,
        ref_fielder:
            RefFielder {
                generics,
                generics_without_bounds,
                fields_without_attr,
                ..
            },
        ..
    } = parsed;
    let put_struct_name = Ident::new(&format!("{}Put", name), name.span());

    let try_into_generics = {
        let mut generics = generics.clone();
        generics.params.push(parse_quote! { AbortReason });

        if generics
            .params
            .iter()
            .all(|param| !matches!(param, GenericParam::Lifetime(_)))
        {
            generics.params.push(parse_quote! { 'a });
        }
        generics
    };

    quote! {
        pub struct #put_struct_name #generics {
            #(#fields_without_attr,)*
        }

        impl #try_into_generics
            TryInto<document::TransactItem<'a, AbortReason>>
            for #put_struct_name
            #generics_without_bounds
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
                Ok(document::TransactItem::Put {
                    name: stringify!(#name),
                    id: self.id,
                    value: #ref_struct_value,
                })
            }
        }
    }
}

fn struct_create_define(
    DocumentParsed {
        name,
        ref_struct_value,
        ref_fielder:
            RefFielder {
                generics,
                generics_without_bounds,
                fields_without_attr,
                ..
            },
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let create_struct_name = Ident::new(&format!("{}Create", name), name.span());

    let try_into_generics = {
        let mut generics = generics.clone();
        generics.params.push(parse_quote! { AbortReason });
        if generics
            .params
            .iter()
            .all(|param| !matches!(param, GenericParam::Lifetime(_)))
        {
            generics.params.push(parse_quote! { 'a });
        }
        generics
    };

    quote! {
        pub struct #create_struct_name #generics {
            #(#fields_without_attr,)*
        }
        impl #try_into_generics
            TryInto<document::TransactItem<'a, AbortReason>>
            for #create_struct_name
            #generics_without_bounds
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
                Ok(document::TransactItem::Create {
                    name: stringify!(#name),
                    id: self.id,
                    value_fn: Some(Box::new(move || Ok(#ref_struct_value))),
                })
            }
        }
    }
}

fn struct_update_define(
    DocumentParsed {
        name,
        ref_fielder: RefFielder { generics, .. },
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let update_struct_name = Ident::new(&format!("{}Update", name), name.span());

    let have_lifetime = generics
        .params
        .iter()
        .any(|param| matches!(param, GenericParam::Lifetime(_)));

    let generics = {
        let mut generics = generics.clone();
        generics.params.push(parse_quote!(AbortReason));

        if have_lifetime {
            generics.params.push(parse_quote!('a));
        }

        let mut param: TypeParam = parse_quote!(WantUpdateFn: Send + FnOnce(&rkyv::Archived<#name>) -> WantUpdate<AbortReason>);
        if have_lifetime {
            param.bounds.insert(0, parse_quote!('a));
        }
        generics.params.push(GenericParam::Type(param));

        let mut param: TypeParam = parse_quote!(UpdateFn: Send + FnOnce(&mut #name));
        if have_lifetime {
            param.bounds.insert(0, parse_quote!('a));
        }
        generics.params.push(GenericParam::Type(param));
        generics
    };

    let try_into_generics = {
        let mut generics = generics.clone();
        if !have_lifetime {
            generics.params.insert(0, parse_quote! { 'a });
        }
        generics.type_params_mut().for_each(|param| {
            if param.ident == "WantUpdateFn" || param.ident == "UpdateFn" {
                param.bounds.insert(0, parse_quote! { 'a });
            }
        });
        generics
    };

    let try_into_generics_without_bounds = {
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
            pub id: u128,
            pub want_update: WantUpdateFn,
            pub update: UpdateFn,
        }

        impl #try_into_generics
            TryInto<document::TransactItem<'a, AbortReason>>
            for #update_struct_name
            #try_into_generics_without_bounds
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
                Ok(document::TransactItem::Update {
                    name: stringify!(#name),
                    id: self.id,
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

fn struct_delete_define(DocumentParsed { name, .. }: &DocumentParsed) -> impl quote::ToTokens {
    let delete_struct_name = Ident::new(&format!("{}Delete", name), name.span());

    quote! {
        pub struct #delete_struct_name {
            pub id: u128,
        }
        impl<'a, AbortReason>
            TryInto<document::TransactItem<'a, AbortReason>>
            for #delete_struct_name
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
                Ok(document::TransactItem::Delete {
                    name: stringify!(#name),
                    id: self.id,
                })
            }
        }
    }
}

fn struct_query_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed { name, .. } = parsed;
    let query_struct_name = Ident::new(&format!("{}Query", name), name.span());
    // todo

    quote! {
        // pub struct #query_struct_name #generics {
        //     #(#fields_without_attr,)*
        // }
        // impl #generics document::DocumentQuery for #query_struct_name #generics_without_bounds {
        //     type Output = #name;

        //     fn pk<'b>(&'b self) -> document::Result<u128> {
        //         Ok(#pk_cow)
        //     }
        // }
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
