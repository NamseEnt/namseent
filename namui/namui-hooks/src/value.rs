use namui_type::*;
use std::any::Any;

pub trait Value: Any + Serialize {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn into_box_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any + 'static + State + Serialize> Value for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn into_box_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
