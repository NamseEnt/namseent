use quote::{quote, ToTokens};
use syn::parse_macro_input;

///
/// # Example
/// ```rust
/// #[component]
/// pub struct MyComponent<'a> {
///     pub a: A,
///     pub _b: &'a str,
///     pub c: EventClosure<dyn Fn()>,
/// }
/// ```
///
/// Above example expands to below,
/// ```rust
/// pub struct MyComponent<'a> {
///     pub a: A,
///     pub _b: &'a str,
///     pub c: EventClosure<dyn Fn()>,
/// }
///
/// impl<'a> Debug for MyComponent<'a> {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("MyComponent")
///             .field("a", &self.a)
///             .field("_b", &self._b)
///             // EventClosure doesn't implement Debug
///             .finish()
///
/// impl<'a> namui::StaticType for MyComponent<'a> {
///     fn static_type_id(&self) -> StaticTypeId {
///         StaticTypeId::Single(TypeId::of::<MyComponent>())
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
            let ty = &field.ty;
            !ty.to_token_stream().to_string().starts_with("EventClosure")
        })
        .map(|field| {
            let ident = &field.ident;
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        });

    let expanded = quote! {
        #item

        impl #struct_generics std::fmt::Debug for #struct_name #struct_generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(std::any::type_name::<#struct_name>())
                    #(#debug_struct_fields)*
                    .finish()
            }
        }

        impl #struct_generics namui::StaticType for #struct_name #struct_generics {
            fn static_type_id(&self) -> namui::StaticTypeId {
                namui::StaticTypeId::Single(std::any::TypeId::of::<#struct_name>())
            }

            fn static_type_name(&self) -> &'static str {
                std::any::type_name::<#struct_name>()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
