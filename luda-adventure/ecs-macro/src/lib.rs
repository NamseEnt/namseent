use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let set_name = format_ident!("{name}_SET");

    let expanded = quote! {
        static mut #set_name: once_cell::sync::OnceCell<sparseset::SparseSet<#name>> = once_cell::sync::OnceCell::new();
        impl crate::ecs::Component for #name {
            fn insert(self, id: usize) {
                unsafe {
                    #set_name.get_or_init(|| sparseset::SparseSet::with_capacity(2048));
                    #set_name.get_mut().unwrap().insert(id, self);
                }
            }

            fn drop(id: usize) {
                unsafe {
                    #set_name.get_mut().unwrap().remove(id);
                }
            }
        }

        impl crate::ecs::ComponentCombination for &#name {
            fn filter(entity: &crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name
                        .get_or_init(|| sparseset::SparseSet::with_capacity(2048))
                        .get(entity.id())
                }
            }
        }
        impl crate::ecs::ComponentCombinationMut for &#name {
            fn filter(entity: &mut crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name
                        .get_or_init(|| sparseset::SparseSet::with_capacity(2048))
                        .get(entity.id())
                }
            }
        }
        impl crate::ecs::ComponentCombinationMut for &mut #name {
            fn filter(entity: &mut crate::ecs::Entity) -> Option<Self> {
                unsafe {
                    #set_name.get_or_init(|| sparseset::SparseSet::with_capacity(2048));
                    #set_name.get_mut().unwrap().get_mut(entity.id())
                }
            }
        }
    };

    TokenStream::from(expanded)
}
