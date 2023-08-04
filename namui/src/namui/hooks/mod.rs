pub(crate) mod channel;
mod component;
mod instance;
mod native;
mod render;
mod sig;
mod tree;
mod value;

pub(crate) use channel::*;
pub use component::*;
pub use hooks_macro::*;
pub(crate) use instance::*;
pub use native::*;
pub use render::*;
pub use render::*;
pub use sig::*;
use std::{
    any::{Any, TypeId},
    collections::HashSet,
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};
pub(crate) use tree::*;
pub use value::*;

fn update_or_push<T>(vector: &mut Vec<T>, index: usize, value: T) {
    if let Some(prev) = vector.get_mut(index) {
        *prev = value;
    } else {
        assert_eq!(vector.len(), index);
        vector.insert(index, value);
    }
}

pub fn boxed<T>(value: T) -> Box<T> {
    Box::new(value)
}

pub fn arc<'a, T: 'a>(value: T) -> Box<T> {
    Box::new(value)
}

/// callback!('a, A)
/// -> &'a (dyn 'a + Fn(A))
#[macro_export]
macro_rules! callback {
    ($lifetime: lifetime, $param: ty) => {
        // &$lifetime (dyn $lifetime + Fn($param))
        Box<dyn $lifetime + Fn($param)>
    };
    ($lifetime: lifetime) => {
        // &$lifetime (dyn $lifetime + Fn())
        Box<dyn $lifetime + Fn()>
    };
}

pub use callback;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use wasm_bindgen_test::wasm_bindgen_test;

//     #[test]
//     #[wasm_bindgen_test]
//     fn single_param() {
//         let name_of_a = {
//             struct A<'a> {
//                 _a: callback!(i32),
//             }
//             std::any::type_name::<A<'static>>()
//         };

//         struct A<'a> {
//             _a: Arc<dyn 'a + Send + Sync + Fn(i32)>,
//         }

//         assert_eq!(name_of_a, std::any::type_name::<A<'static>>());
//     }

//     #[test]
//     #[wasm_bindgen_test]
//     fn single_param_with_return() {
//         let name_of_a = {
//             struct A<'a> {
//                 _a: callback!(i32 -> i32),
//             }
//             std::any::type_name::<A<'static>>()
//         };

//         struct A<'a> {
//             _a: Arc<dyn 'a + Send + Sync + Fn(i32) -> i32>,
//         }

//         assert_eq!(name_of_a, std::any::type_name::<A<'static>>());
//     }

//     #[test]
//     #[wasm_bindgen_test]
//     fn multiple_params() {
//         let name_of_a = {
//             struct A<'a> {
//                 _a: callback!(i32, i32),
//             }
//             std::any::type_name::<A<'static>>()
//         };

//         struct A<'a> {
//             _a: Arc<dyn 'a + Send + Sync + Fn((i32, i32))>,
//         }

//         assert_eq!(name_of_a, std::any::type_name::<A<'static>>());
//     }

//     #[test]
//     #[wasm_bindgen_test]
//     fn multiple_params_with_return() {
//         let name_of_a = {
//             struct A<'a> {
//                 _a: callback!(i32, i32 -> i32),
//             }
//             std::any::type_name::<A<'static>>()
//         };

//         struct A<'a> {
//             _a: Arc<dyn 'a + Send + Sync + Fn((i32, i32)) -> i32>,
//         }

//         assert_eq!(name_of_a, std::any::type_name::<A<'static>>());
//     }
// }
