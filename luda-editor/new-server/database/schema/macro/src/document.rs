use crate::document_parsed::*;
use macro_common_lib::*;
use quote::quote;
use syn::*;

pub fn document(
    _attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    let struct_name = input.ident.clone();

    let parsed = DocumentParsed::new(input);

    let input_redefine = &parsed.input_redefine;

    let ref_struct = parsed.ref_struct();

    let struct_get_define = struct_get_define(&parsed);
    let struct_put_define = struct_put_define(&parsed);
    let struct_create_define = struct_create_define(&parsed);
    let struct_update_define = struct_update_define(&parsed);
    let struct_delete_define = struct_delete_define(&parsed);
    let debug_define = debug_define(&parsed);

    let output = quote! {
        #input_redefine

        impl document::Document for #struct_name {
            fn name() -> &'static str {
                stringify!(#struct_name)
            }

            fn from_slice(bytes: &[u8]) -> document::Result<Self> {
                serializer::deserialize(bytes)
            }

            fn to_bytes(&self) -> document::Result<Vec<u8>> {
                serializer::serialize(self)
            }
        }

        #ref_struct

        #struct_get_define
        #struct_put_define
        #struct_create_define
        #struct_update_define
        #struct_delete_define

        #debug_define
    };

    output.into()
}

fn struct_get_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed {
        name,
        id_fields,
        id_field_idents,
        ..
    } = parsed;
    let get_struct_name = Ident::new(&format!("{name}Get"), name.span());

    quote! {
        pub struct #get_struct_name {
            #(#id_fields,)*
        }
        impl document::DocumentGet for #get_struct_name {
            type Output = #name;

            fn id(&self) -> u128 {
                document::id_to_u128(&(#(self.#id_field_idents),*))
            }
        }
    }
}

fn struct_put_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed {
        name,
        ref_struct_value,
        id_field_idents,
        ref_fielder:
            RefFielder {
                generics,
                generics_without_bounds,
                fields,
                ..
            },
        ..
    } = parsed;
    let put_struct_name = Ident::new(&format!("{name}Put"), name.span());

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
            #(#fields,)*
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
                    id: document::id_to_u128(&(#(self.#id_field_idents),*)),
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
                fields,
                ..
            },
        id_field_idents,
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let create_struct_name = Ident::new(&format!("{name}Create"), name.span());

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
            #(#fields,)*
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
                    id: document::id_to_u128(&(#(self.#id_field_idents),*)),
                    value_fn: Some(Box::new(move || Ok(#ref_struct_value))),
                })
            }
        }
    }
}

fn struct_update_define(
    DocumentParsed {
        name,
        id_field_idents,
        id_ref_fielder: RefFielder {
            generics, fields, ..
        },
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let update_struct_name = Ident::new(&format!("{name}Update"), name.span());

    let has_lifetime = generics
        .params
        .iter()
        .any(|param| matches!(param, GenericParam::Lifetime(_)));

    let generics = {
        let mut generics = generics.clone();

        generics.params.push(parse_quote!(AbortReason));
        generics
            .params
            .push(parse_quote!(WantUpdateFn: Send + FnOnce(&#name) -> WantUpdate<AbortReason>));
        generics
            .params
            .push(parse_quote!(UpdateFn: Send + FnOnce(&mut #name)));

        if has_lifetime {
            generics.type_params_mut().for_each(|param| {
                if param.ident == "WantUpdateFn" || param.ident == "UpdateFn" {
                    param.bounds.insert(0, parse_quote! { 'a });
                }
            });
        }
        generics
    };

    let try_into_generics = {
        let mut generics = generics.clone();
        if !has_lifetime {
            generics.params.insert(0, parse_quote! { 'a });
            generics.type_params_mut().for_each(|param| {
                if param.ident == "WantUpdateFn" || param.ident == "UpdateFn" {
                    param.bounds.insert(0, parse_quote! { 'a });
                }
            });
        }
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
            #(#fields,)*
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
                    id: document::id_to_u128(&(#(self.#id_field_idents),*)),
                    update_fn: Some(Box::new(|vec| {
                        let mut doc = serializer::deserialize(&vec)?;
                        let want_update = (self.want_update)(&doc);

                        if let WantUpdate::Yes = want_update {
                            (self.update)(&mut doc);
                            *vec = serializer::serialize(&doc)?;
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
        id_fields,
        id_field_idents,
        ..
    }: &DocumentParsed,
) -> impl quote::ToTokens {
    let delete_struct_name = Ident::new(&format!("{name}Delete"), name.span());

    quote! {
        pub struct #delete_struct_name {
            #(#id_fields,)*
        }
        impl<'a, AbortReason>
            TryInto<document::TransactItem<'a, AbortReason>>
            for #delete_struct_name
        {
            type Error = document::SerErr;
            fn try_into(self) -> document::Result<document::TransactItem<'a, AbortReason>> {
                Ok(document::TransactItem::Delete {
                    name: stringify!(#name),
                    id: document::id_to_u128(&(#(self.#id_field_idents),*)),
                })
            }
        }
    }
}

fn debug_define(parsed: &DocumentParsed) -> impl quote::ToTokens {
    let DocumentParsed { name, .. } = parsed;

    quote! {
        document::inventory::submit! {
            document::DocumentLogPlugin::new(stringify!(#name), |value| {
                let Ok(deserialized) = serializer::deserialize::<
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
