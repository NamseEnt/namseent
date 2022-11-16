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
