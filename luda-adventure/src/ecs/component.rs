use super::Entity;
use std::fmt::Debug;

pub trait Component: Debug {}

#[derive(Debug)]
pub struct ComponentContainer<T> {
    pub component: T,
}
impl<T> ComponentContainer<T>
where
    T: Component,
{
    pub fn new(component: T) -> Self {
        Self { component }
    }
    pub fn as_ref(&self) -> &T {
        &self.component
    }
    pub fn as_ref_mut(&mut self) -> &mut T {
        &mut self.component
    }
}

pub trait ContainedComponent: Debug + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
impl<T: Debug + 'static> ContainedComponent for ComponentContainer<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

pub trait ComponentQueryArgument<'component> {
    type Output;
    fn filter(
        contained_component: &Box<dyn ContainedComponent>,
        filtered: &Option<&'component Box<dyn ContainedComponent>>,
    ) -> bool;
    fn output(filtered: Option<&'component Box<dyn ContainedComponent>>) -> Option<Self::Output>;
}
pub trait ComponentQueryArgumentMut<'component> {
    type Output;
    fn filter(
        contained_component: &Box<dyn ContainedComponent>,
        filtered: &Option<&'component mut Box<dyn ContainedComponent>>,
    ) -> bool;
    fn output(
        filtered: Option<&'component mut Box<dyn ContainedComponent>>,
    ) -> Option<Self::Output>;
}

pub trait ComponentQueryCombination<'entity> {
    type Output;
    fn filter(entity: &'entity Entity) -> Option<Self::Output>
    where
        Self: Sized;
}
pub trait ComponentQueryCombinationMut<'entity> {
    type Output;
    fn filter(entity: &'entity mut Entity) -> Option<Self::Output>
    where
        Self: Sized;
}
