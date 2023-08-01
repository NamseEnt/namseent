use quote::{format_ident, quote, ToTokens};
use syn::parse_macro_input;

///
/// # Example
/// ```rust
/// #[component]
/// pub struct MyComponent<'a, C: Abc> {
///     pub a: A,
///     pub _b: &'a str,
///     pub c: Abc,
///     pub d: &dyn Fn(),
/// }
/// ```
///
/// Above example expands to below,
/// ```rust
/// pub struct MyComponent<'a, C: Abc> {
///     pub a: A,
///     pub _b: &'a str,
///     pub c: Abc,
/// }
///
/// impl<'a, C: Abc> Debug for MyComponent<'a, C> {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("MyComponent")
///             .field("a", &self.a)
///             .field("_b", &self._b)
///             .field("c", &self.c)
///             // ignore Fn series
///             .finish()
///
/// impl<'a, C: Abc> namui::StaticType for MyComponent<'a, C> {
///     fn static_type_id(&self) -> StaticTypeId {
///         // 'a become 'static
///         StaticTypeId::Single(TypeId::of::<MyComponent<'static, C>>())
///     }
/// }
/// ```
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
            !(ty.contains(" Fn(") || ty.contains(" FnMut(") || ty.contains(" FnOnce("))
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
        .filter(|param| match param {
            syn::GenericParam::Lifetime(_) => false,
            _ => true,
        })
        .map(|param| {
            quote! { #param }
        })
        .collect::<Vec<_>>();

    let where_clause = &struct_generics.where_clause;

    // Abc<'a, 'b, 'c, D, E> -> Abc<'static, 'static, 'static, D, E>
    let struct_type_with_static_lifetime = {
        let static_struct_generics = if struct_generics.lt_token.is_none() {
            None
        } else {
            let static_ed = struct_generics.params.iter().map(|param| match param {
                syn::GenericParam::Lifetime(_) => quote! { 'static },
                syn::GenericParam::Type(generic_type) => generic_type.ident.to_token_stream(),
                syn::GenericParam::Const(generic_const) => generic_const.ident.to_token_stream(),
            });

            Some(quote! {<#(#static_ed),*>})
        };

        quote! {
            #struct_name #static_struct_generics
        }
    };

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

    let expanded = quote! {
        #item

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
            // fn static_type_id(&self) -> namui::StaticTypeId {
            //     // 'a become 'static
            //     namui::StaticTypeId::Single(std::any::TypeId::of::<#struct_type_with_static_lifetime>())
            // }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn component_debug(
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
            let ty = &field.ty;
            let ty = ty.to_token_stream().to_string();
            !(ty.contains(" Fn(") || ty.contains(" FnMut(") || ty.contains(" FnOnce("))
        })
        // .map(|field| {
        //     let ident = &field.ident;
        //     quote! {
        //         .field(stringify!(#ident), &self.#ident)
        //     }
        // })
        .collect::<Vec<_>>();

    panic!(
        "{}",
        quote! {
            #(#debug_struct_fields)*
        }
    );

    proc_macro::TokenStream::from(quote! {
        #item

        // #(#debug_struct_fields)*
    })

    // let generic_next_to_impl_except_lifetime = struct_generics
    //     .params
    //     .iter()
    //     .filter(|param| match param {
    //         syn::GenericParam::Lifetime(_) => false,
    //         _ => true,
    //     })
    //     .map(|param| {
    //         quote! { #param }
    //     })
    //     .collect::<Vec<_>>();

    // let where_clause = &struct_generics.where_clause;

    // // Abc<'a, 'b, 'c, D, E> -> Abc<'static, 'static, 'static, D, E>
    // let struct_type_with_static_lifetime = {
    //     let static_struct_generics = if struct_generics.lt_token.is_none() {
    //         None
    //     } else {
    //         let static_ed = struct_generics.params.iter().map(|param| match param {
    //             syn::GenericParam::Lifetime(_) => quote! { 'static },
    //             syn::GenericParam::Type(generic_type) => generic_type.ident.to_token_stream(),
    //             syn::GenericParam::Const(generic_const) => generic_const.ident.to_token_stream(),
    //         });

    //         Some(quote! {<#(#static_ed),*>})
    //     };

    //     quote! {
    //         #struct_name #static_struct_generics
    //     }
    // };

    // let struct_generics_next_to_for_struct = {
    //     if struct_generics.lt_token.is_none() {
    //         quote! {}
    //     } else {
    //         let idents = struct_generics.params.iter().map(|param| match param {
    //             syn::GenericParam::Lifetime(_) => quote! { '_ },
    //             syn::GenericParam::Type(generic_type) => generic_type.ident.to_token_stream(),
    //             syn::GenericParam::Const(generic_const) => generic_const.ident.to_token_stream(),
    //         });

    //         quote! {<#(#idents),*>}
    //     }
    // };

    // let expanded = quote! {
    //     #item

    //     impl<#(#generic_next_to_impl_except_lifetime),*> std::fmt::Debug for #struct_name #struct_generics_next_to_for_struct
    //     #where_clause
    //     {
    //         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //             f.debug_struct(stringify!(#struct_name))
    //                 #(#debug_struct_fields)*
    //                 .finish()
    //         }
    //     }

    //     impl<#(#generic_next_to_impl_except_lifetime),*> namui::StaticType for #struct_name #struct_generics_next_to_for_struct
    //     #where_clause
    //     {
    //         // fn static_type_id(&self) -> namui::StaticTypeId {
    //         //     // 'a become 'static
    //         //     namui::StaticTypeId::Single(std::any::TypeId::of::<#struct_type_with_static_lifetime>())
    //         // }
    //     }
    // };

    // proc_macro::TokenStream::from(expanded)
}
