use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl crate::ecs::Component for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn define_component_combinations(_input: TokenStream) -> TokenStream {
    let expanded = (2..32).into_iter().map(|index| {
        let zero_to_index = 0..index;

        let generics = zero_to_index
            .clone()
            .map(|i| {
                let name = format_ident!("T{i}");
                quote! {
                    #name: Component + 'static
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
                    &'entity #t_name
                )
            })
            .collect::<Vec<_>>();

        let outputs_mut = zero_to_index
            .clone()
            .map(|i| {
                let t_name = format_ident!("T{i}");
                quote!(
                    &'entity mut #t_name
                )
            })
            .collect::<Vec<_>>();

        let picked_components = zero_to_index
            .clone()
            .map(|i| {
                let component_name = format_ident!("component{i}");
                quote!(
                    let mut #component_name = None;
                )
            })
            .collect::<Vec<_>>();

        let picker_statements = zero_to_index
            .clone()
            .map(|i| {
                let picked_component_name = format_ident!("component{i}");
                let t_name = format_ident!("T{i}");
                quote! {
                    if component.as_any().is::<ComponentContainer<#t_name>>() {
                        #picked_component_name = Some(component);
                    }
                }
            })
            .collect::<Vec<_>>();

        let tuple_content = zero_to_index
            .clone()
            .map(|i| {
                let picked_component_name = format_ident!("component{i}");
                let t_name = format_ident!("T{i}");
                quote! {
                    #picked_component_name?
                        .as_any()
                        .downcast_ref::<ComponentContainer<#t_name>>()?
                        .as_ref()
                }
            })
            .collect::<Vec<_>>();

        let tuple_content_mut = zero_to_index
            .clone()
            .map(|i| {
                let picked_component_name = format_ident!("component{i}");
                let t_name = format_ident!("T{i}");
                quote! {
                    #picked_component_name?
                        .as_any_mut()
                        .downcast_mut::<ComponentContainer<#t_name>>()?
                        .as_ref_mut()
                }
            })
            .collect::<Vec<_>>();

        quote! {
            impl<'entity, #(#generics),* > ComponentCombination<'entity> for ( #(#for_target),* )
            {
                type Output = ( #(#outputs),* );
                fn filter(entity: &'entity Entity) -> Option<Self::Output> {
                    #(#picked_components)*
                    for component in entity.components.iter() {
                        #(#picker_statements)else*
                    }
                    Some((#(#tuple_content),*))
                }
            }
            impl<'entity, #(#generics),* > ComponentCombinationMut<'entity> for ( #(#for_target),* )
            {
                type Output = ( #(#outputs_mut),* );
                fn filter(entity: &'entity mut Entity) -> Option<Self::Output> {
                    #(#picked_components)*
                    for component in entity.components.iter_mut() {
                        #(#picker_statements)else*
                    }
                    Some((#(#tuple_content_mut),*))
                }
            }
        }
    });

    TokenStream::from(quote!(#(#expanded)*))
}
