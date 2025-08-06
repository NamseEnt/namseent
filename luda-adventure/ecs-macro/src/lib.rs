use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn component(_attribute_input: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident.clone();

    let set_name = format_ident!("{name}_SET");

    let expanded = quote! {
        type AppId = namui::Uuid;
        type EntityId = namui::Uuid;
        use once_cell::sync::OnceCell;
        use rustc_hash::FxHashMap;
        #[allow(non_upper_case_globals)]
        pub static mut #set_name: OnceCell<FxHashMap<AppId, FxHashMap<EntityId, #name>>> = OnceCell::new();
        impl crate::ecs::Component for #name {
            fn insert(self, app_id: AppId, entity_id: EntityId) {
                unsafe {
                    #set_name.get_or_init(|| rustc_hash::FxHashMap::default());
                    #set_name.get_mut()
                        .unwrap()
                        .entry(app_id)
                        .or_insert_with(|| rustc_hash::FxHashMap::default())
                        .insert(entity_id, self);
                }
            }

            fn drop(app_id: AppId, entity_id: EntityId) {
                unsafe {
                    #set_name.get_mut()
                        .unwrap()
                        .get_mut(&app_id)
                        .unwrap()
                        .remove(&entity_id);
                }
            }
        }

        impl crate::ecs::ComponentCombination for &#name {
            fn filter(app_id: AppId, entity: &crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name
                        .get_or_init(|| rustc_hash::FxHashMap::default())
                        .get(&app_id)?
                        .get(&entity.id())
                }
            }
        }
        impl crate::ecs::ComponentCombinationMut for &#name {
            fn filter(app_id: AppId, entity: &mut crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name
                        .get_or_init(|| rustc_hash::FxHashMap::default())
                        .get(&app_id)?
                        .get(&entity.id())
                }
            }
        }
        impl crate::ecs::ComponentCombinationMut for &mut #name {
            fn filter(app_id: AppId, entity: &mut crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name.get_or_init(|| rustc_hash::FxHashMap::default());
                    #set_name.get_mut()
                        .unwrap()
                        .entry(app_id)
                        .or_insert_with(|| rustc_hash::FxHashMap::default());
                    #set_name.get_mut()
                        .unwrap()
                        .get_mut(&app_id)
                        .unwrap()
                        .get_mut(&entity.id())
                }
            }
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        #input
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
                    let #result_name = #t_name::filter(app_id, entity)?;
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
                fn filter(app_id: namui::Uuid, entity: &Entity) -> Option<Self> {
                    #(#filter_statements)*
                    Some((#(#tuple_content),*))
                }
            }
            impl< #(#generics_mut),* > ComponentCombinationMut for ( #(#for_target),* )
            {
                fn filter(app_id: namui::Uuid, entity: &mut Entity) -> Option<Self> {
                    #(#filter_statements)*
                    Some((#(#tuple_content),*))
                }
            }
        }
    });

    TokenStream::from(quote!(#(#expanded)*))
}
