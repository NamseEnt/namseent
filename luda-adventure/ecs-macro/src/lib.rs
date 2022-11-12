use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl crate::ecs::Component for #name {}

        impl<'a> crate::ecs::ComponentCombination<'a> for &#name {
            type Output = std::cell::Ref<'a, #name>;
            fn filter(entity: &'a crate::ecs::Entity) -> Option<Self::Output> {
                entity.get_component::<#name>()
            }
        }
        impl<'a> crate::ecs::ComponentCombinationMut<'a> for &#name {
            type Output = std::cell::Ref<'a, #name>;
            fn filter(entity: &'a crate::ecs::Entity) -> Option<Self::Output> {
                entity.get_component::<#name>()
            }
        }
        impl<'a> crate::ecs::ComponentCombinationMut<'a> for &mut #name {
            type Output = std::cell::RefMut<'a, #name>;
            fn filter(entity: &'a crate::ecs::Entity) -> Option<Self::Output> {
                entity.get_component_mut::<#name>()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn define_combinations(_input: TokenStream) -> TokenStream {
    let expanded = (2..32).into_iter().map(|index| {
        let zero_to_index = 0..index;

        let generics = zero_to_index
            .clone()
            .map(|i| {
                let name = format_ident!("T{i}");
                quote! {
                    #name: ComponentCombination<'a>
                }
            })
            .collect::<Vec<_>>();

        let generics_mut = zero_to_index
            .clone()
            .map(|i| {
                let name = format_ident!("T{i}");
                quote! {
                    #name: ComponentCombinationMut<'a>
                }
            })
            .collect::<Vec<_>>();

        let for_target = zero_to_index
            .clone()
            .map(|i| format_ident!("T{i}"))
            .collect::<Vec<_>>();

        let outputs = zero_to_index
            .clone()
            .map(|i| {
                let t_name = format_ident!("T{i}");
                quote!(
                    #t_name::Output
                )
            })
            .collect::<Vec<_>>();

        let filter_statements = zero_to_index
            .clone()
            .map(|i| {
                let result_name = format_ident!("result_{i}");
                let t_name = format_ident!("T{i}");
                quote! {
                    let #result_name = #t_name::filter(entity)?;
                }
            })
            .collect::<Vec<_>>();

        let tuple_content = zero_to_index
            .clone()
            .map(|i| format_ident!("result_{i}"))
            .collect::<Vec<_>>();

        quote! {
            impl<'a, #(#generics),* > ComponentCombination<'a> for ( #(#for_target),* )
            {
                type Output = ( #(#outputs),* );
                fn filter(entity: &'a Entity) -> Option<Self::Output> {
                    #(#filter_statements)*
                    Some((#(#tuple_content),*))
                }
            }
            impl<'a, #(#generics_mut),* > ComponentCombinationMut<'a> for ( #(#for_target),* )
            {
                type Output = ( #(#outputs),* );
                fn filter(entity: &'a Entity) -> Option<Self::Output> {
                    #(#filter_statements)*
                    Some((#(#tuple_content),*))
                }
            }
        }
    });

    TokenStream::from(quote!(#(#expanded)*))
}
