use super::Entity;
use std::{cell::RefCell, fmt::Debug};

pub trait Component: Debug {}
pub trait WrappedComponent: Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
impl<T: Debug + 'static> WrappedComponent for RefCell<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

pub trait ComponentCombination<'a> {
    type Output;
    fn filter(entity: &'a Entity) -> Option<Self::Output>
    where
        Self: Sized;
}

pub trait ComponentCombinationMut<'a> {
    type Output;
    fn filter(entity: &'a Entity) -> Option<Self::Output>
    where
        Self: Sized;
}

ecs_macro::define_combinations!();
