use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl crate::ecs::Component for #name {}

        impl<'component> crate::ecs::ComponentQueryArgument<'component> for #name {
            type Output = &'component #name;
            fn filter(
                contained_component: &Box<dyn crate::ecs::ContainedComponent>,
                filtered: &Option<&'component Box<dyn crate::ecs::ContainedComponent>>,
            ) -> bool {
                if filtered.is_some() {
                    return false;
                }
                if contained_component.as_any().is::<crate::ecs::ComponentContainer<#name>>() {
                    return true;
                }
                false
            }
            fn output(filtered: Option<&'component Box<dyn crate::ecs::ContainedComponent>>) -> Option<Self::Output> {
                Some(
                    filtered?
                        .as_any()
                        .downcast_ref::<crate::ecs::ComponentContainer<#name>>()?
                        .as_ref(),
                )
            }
        }

        impl<'component> crate::ecs::ComponentQueryArgumentMut<'component> for #name {
            type Output = &'component mut #name;
            fn filter(
                contained_component: &Box<dyn crate::ecs::ContainedComponent>,
                filtered: &Option<&'component mut Box<dyn crate::ecs::ContainedComponent>>,
            ) -> bool {
                if filtered.is_some() {
                    return false;
                }
                if contained_component.as_any().is::<crate::ecs::ComponentContainer<#name>>() {
                    return true;
                }
                false
            }
            fn output(filtered: Option<&'component mut Box<dyn crate::ecs::ContainedComponent>>) -> Option<Self::Output> {
                Some(
                    filtered?
                        .as_any_mut()
                        .downcast_mut::<crate::ecs::ComponentContainer<#name>>()?
                        .as_ref_mut(),
                )
            }
        }

        impl<'component> crate::ecs::ComponentQueryArgument<'component> for Option<#name> {
            type Output = Option<&'component #name>;
            fn filter(
                contained_component: &Box<dyn crate::ecs::ContainedComponent>,
                filtered: &Option<&'component Box<dyn crate::ecs::ContainedComponent>>,
            ) -> bool {
                if filtered.is_some() {
                    return false;
                }
                if contained_component.as_any().is::<crate::ecs::ComponentContainer<#name>>() {
                    return true;
                }
                false
            }
            fn output(filtered: Option<&'component Box<dyn crate::ecs::ContainedComponent>>) -> Option<Self::Output> {
                Some(filtered.and_then(|filtered| {
                    Some(
                        filtered
                            .as_any()
                            .downcast_ref::<crate::ecs::ComponentContainer<#name>>()?
                            .as_ref(),
                    )
                }))
            }
        }

        impl<'component> crate::ecs::ComponentQueryArgumentMut<'component> for Option<#name> {
            type Output = Option<&'component mut #name>;
            fn filter(
                contained_component: &Box<dyn crate::ecs::ContainedComponent>,
                filtered: &Option<&'component mut Box<dyn crate::ecs::ContainedComponent>>,
            ) -> bool {
                if filtered.is_some() {
                    return false;
                }
                if contained_component.as_any().is::<crate::ecs::ComponentContainer<#name>>() {
                    return true;
                }
                false
            }
            fn output(filtered: Option<&'component mut Box<dyn crate::ecs::ContainedComponent>>) -> Option<Self::Output> {
                Some(filtered.and_then(|filtered| {
                    Some(
                        filtered
                            .as_any_mut()
                            .downcast_mut::<crate::ecs::ComponentContainer<#name>>()?
                            .as_ref_mut(),
                    )
                }))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn define_component_query_combinations(_input: TokenStream) -> TokenStream {
    let expanded = (2..32).into_iter().map(|index| {
        let zero_to_index = 0..index;

        let generics = zero_to_index
            .clone()
            .map(|i| {
                let t_name = format_ident!("T{i}");
                quote! {
                    #t_name: ComponentQueryArgument<'entity>
                }
            })
            .collect::<Vec<_>>();

        let generics_mut = zero_to_index
            .clone()
            .map(|i| {
                let t_name = format_ident!("T{i}");
                quote! {
                    #t_name: ComponentQueryArgumentMut<'entity>
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

        let filtered_components = zero_to_index
            .clone()
            .map(|i| {
                let filtered_component_name = format_ident!("filtered{i}");
                quote!(
                    let mut #filtered_component_name = None;
                )
            })
            .collect::<Vec<_>>();

        let filter_statements = zero_to_index
            .clone()
            .map(|i| {
                let filtered_component_name = format_ident!("filtered{i}");
                let t_name = format_ident!("T{i}");
                quote! {
                    if #t_name::filter(component, &#filtered_component_name) {
                        #filtered_component_name = Some(component);
                    }
                }
            })
            .collect::<Vec<_>>();

        let tuple_content = zero_to_index
            .clone()
            .map(|i| {
                let filtered_component_name = format_ident!("filtered{i}");
                let t_name = format_ident!("T{i}");
                quote! {
                    #t_name::output(#filtered_component_name)?
                }
            })
            .collect::<Vec<_>>();

        quote! {
            impl<'entity, #(#generics),* > ComponentQueryCombination<'entity> for ( #(#for_target),* )
            {
                type Output = ( #(#outputs),* );
                fn filter(entity: &'entity Entity) -> Option<Self::Output> {
                    #(#filtered_components)*
                    for component in entity.components.iter() {
                        #(#filter_statements)*
                    }
                    Some((#(#tuple_content),*))
                }
            }
            impl<'entity, #(#generics_mut),* > ComponentQueryCombinationMut<'entity> for ( #(#for_target),* )
            {
                type Output = ( #(#outputs),* );
                fn filter(entity: &'entity mut Entity) -> Option<Self::Output> {
                    #(#filtered_components)*
                    for component in entity.components.iter_mut() {
                        #(#filter_statements)else*
                    }
                    Some((#(#tuple_content),*))
                }
            }
        }
    });

    TokenStream::from(quote!(#(#expanded)*))
}
