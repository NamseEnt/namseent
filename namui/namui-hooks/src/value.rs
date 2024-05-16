use std::{any::Any, rc::Rc};

pub trait Value: Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn into_rc(self: Rc<Self>) -> Rc<dyn Any>;
    fn into_rc_any(self: Rc<Self>) -> Rc<dyn std::any::Any>;
}

impl<T: Any + 'static> Value for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn into_rc(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }
    fn into_rc_any(self: Rc<Self>) -> Rc<dyn std::any::Any> {
        self
    }
}
