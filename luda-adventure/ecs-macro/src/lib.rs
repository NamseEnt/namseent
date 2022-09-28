use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let set_name = format_ident!("{name}_SET");

    let expanded = quote! {
        static mut #set_name: once_cell::sync::OnceCell<rustc_hash::FxHashMap<namui::Uuid, #name>> = once_cell::sync::OnceCell::new();
        impl crate::ecs::Component for #name {
            fn insert(self, id: namui::Uuid) {
                unsafe {
                    #set_name.get_or_init(|| rustc_hash::FxHashMap::default());
                    #set_name.get_mut().unwrap().insert(id, self);
                }
            }

            fn drop(id: namui::Uuid) {
                unsafe {
                    #set_name.get_mut().unwrap().remove(&id);
                }
            }
        }

        impl crate::ecs::ComponentCombination for &#name {
            fn filter(entity: &crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name
                        .get_or_init(|| rustc_hash::FxHashMap::default())
                        .get(&entity.id())
                }
            }
        }
        impl crate::ecs::ComponentCombinationMut for &#name {
            fn filter(entity: &mut crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name
                        .get_or_init(|| rustc_hash::FxHashMap::default())
                        .get(&entity.id())
                }
            }
        }
        impl crate::ecs::ComponentCombinationMut for &mut #name {
            fn filter(entity: &mut crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name.get_or_init(|| rustc_hash::FxHashMap::default());
                    #set_name.get_mut().unwrap().get_mut(&entity.id())
                }
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
                    #name: ComponentCombination
                }
            })
            .collect::<Vec<_>>();

        let generics_mut = zero_to_index
            .clone()
            .map(|i| {
                let name = format_ident!("T{i}");
                quote! {
                    #name: ComponentCombinationMut
                }
            })
            .collect::<Vec<_>>();

        let for_target = zero_to_index
            .clone()
            .map(|i| format_ident!("T{i}"))
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
            impl< #(#generics),* > ComponentCombination for ( #(#for_target),* )
            {
                fn filter(entity: &Entity) -> Option<Self> {
                    #(#filter_statements)*
                    Some((#(#tuple_content),*))
                }
            }
            impl< #(#generics_mut),* > ComponentCombinationMut for ( #(#for_target),* )
            {
                fn filter(entity: &mut Entity) -> Option<Self> {
                    #(#filter_statements)*
                    Some((#(#tuple_content),*))
                }
            }
        }
    });

    TokenStream::from(quote!(#(#expanded)*))
}
