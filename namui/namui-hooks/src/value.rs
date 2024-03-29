use std::{any::Any, fmt::Debug, rc::Rc};

pub trait Value: Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn into_rc(self: Rc<Self>) -> Rc<dyn Any>;
    fn into_rc_any(self: Rc<Self>) -> Rc<dyn std::any::Any>;
    // fn as_box(self: Box<Self>) -> Box<dyn  Any>;
    // fn as_value_mut(&mut self) -> &mut dyn Value;
}

impl<T: Debug + Any + 'static> Value for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
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
    // fn as_box(self: Box<Self>) -> Box<dyn  Any> {
    //     self
    // }
    // fn as_value_mut(&mut self) -> &mut dyn Value {
    //     self
    // }
}

impl Debug for dyn Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Value::fmt(self, f)
    }
}
