use quote::{quote, ToTokens};
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
///     #[skip_debug]
///     pub e: &dyn FnMut(),
/// }
/// ```
///
/// Above example expands to below,
/// ```rust
/// pub struct MyComponent<'a, C: Abc> {
///     pub a: A,
///     pub _b: &'a str,
///     pub c: Abc,
///     pub d: &dyn Fn(),
///     pub e: &dyn FnMut(),
/// }
///
/// impl<'a, C: Abc> Debug for MyComponent<'a, C> {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("MyComponent")
///             .field("a", &self.a)
///             .field("_b", &self._b)
///             .field("c", &self.c)
///             // ignore Fn series
///             // ignore by attribute #[skip_debug]
///             .finish()
///
/// impl<'a, C: Abc> namui::StaticType for MyComponent<'a, C> {
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
        .filter(|param| match param {
            syn::GenericParam::Lifetime(_) => false,
            _ => true,
        })
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

// ///
// /// callback!(A)
// /// -> std::sync::Arc<dyn 'a + Send + Sync + Fn(A)>
// ///
// /// callback!(A -> B)
// /// -> std::sync::Arc<dyn 'a + Send + Sync + Fn(A) -> B>
// ///
// /// callback!(A, B)
// /// -> std::sync::Arc<dyn 'a + Send + Sync + Fn((A, B))>
// ///
// /// callback!(A, B -> C)
// /// -> std::sync::Arc<dyn 'a + Send + Sync + Fn((A, B)) -> C>
// ///
// #[proc_macro]
// pub fn callback(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     struct Input {
//         params: Vec<(syn::Type, Option<syn::token::Comma>)>,
//         arrow_token: Option<syn::token::RArrow>,
//         return_type: Option<syn::Type>,
//     }

//     impl syn::parse::Parse for Input {
//         fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//             let mut params = Vec::new();

//             while !input.is_empty() && !input.peek(syn::token::RArrow) {
//                 let ty = input.parse()?;
//                 let comma = input.parse()?;
//                 params.push((ty, comma));
//             }

//             let arrow_token: Option<syn::Token![->]> = input.parse()?;
//             let return_type = if arrow_token.is_some() {
//                 Some(input.parse()?)
//             } else {
//                 None
//             };

//             Ok(Self {
//                 params,
//                 arrow_token,
//                 return_type,
//             })
//         }
//     }

//     let Input {
//         params,
//         arrow_token,
//         return_type,
//     } = parse_macro_input!(item as Input);

//     let param_list = params
//         .iter()
//         .map(|(ty, comma)| quote! { #ty #comma })
//         .collect::<Vec<_>>();

//     let expanded = quote! {
//         // std::sync::Arc<dyn 'a + Send + Sync + Fn(#(#param_list)*) #arrow_token #return_type>
//         &'a (dyn 'a + Fn(#(#param_list)*) #arrow_token #return_type)
//     };

//     proc_macro::TokenStream::from(expanded)
// }
