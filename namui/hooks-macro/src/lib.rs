use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn component(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as syn::ItemStruct);

    let struct_name = &item.ident;
    let struct_generics = &item.generics;

    let debug_struct_fields = item
        .fields
        .iter()
        .filter(|field| {
            let ty = &field.ty.to_token_stream().to_string();
            !(ty.contains("callback!")
                || ty.contains(" Fn(")
                || ty.contains(" FnMut(")
                || ty.contains(" FnOnce("))
        })
        .filter(|field| {
            !field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("skip_debug"))
        })
        .map(|field| {
            let ident = &field.ident;
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        })
        .collect::<Vec<_>>();

    let generic_next_to_impl_except_lifetime = struct_generics
        .params
        .iter()
        .filter(|param| !matches!(param, syn::GenericParam::Lifetime(_)))
        .map(|param| {
            quote! { #param }
        })
        .collect::<Vec<_>>();

    let where_clause = &struct_generics.where_clause;

    let struct_generics_next_to_for_struct = {
        if struct_generics.lt_token.is_none() {
            quote! {}
        } else {
            let idents = struct_generics.params.iter().map(|param| match param {
                syn::GenericParam::Lifetime(_) => quote! { '_ },
                syn::GenericParam::Type(generic_type) => generic_type.ident.to_token_stream(),
                syn::GenericParam::Const(generic_const) => generic_const.ident.to_token_stream(),
            });

            quote! {<#(#idents),*>}
        }
    };

    let attribute_removed_item = {
        let mut item = item.clone();
        item.fields.iter_mut().for_each(|field| {
            field
                .attrs
                .retain(|attr| !attr.path().is_ident("skip_debug"));
        });
        item
    };

    let expanded = quote! {
        #attribute_removed_item

        impl<#(#generic_next_to_impl_except_lifetime),*> std::fmt::Debug for #struct_name #struct_generics_next_to_for_struct
        #where_clause
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#struct_name))
                    #(#debug_struct_fields)*
                    .finish()
            }
        }

        impl<#(#generic_next_to_impl_except_lifetime),*> namui::StaticType for #struct_name #struct_generics_next_to_for_struct
        #where_clause
        {
        }
    };

    proc_macro::TokenStream::from(expanded)
}
