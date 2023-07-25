use std::{any::Any, fmt::Debug, sync::Arc};

pub trait Value: Send + Sync + Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn as_arc(self: Arc<Self>) -> Arc<dyn Send + Sync + Any>;
}

impl<T: Debug + Send + Sync + Any + 'static> Value for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_arc(self: Arc<Self>) -> Arc<dyn Send + Sync + Any> {
        self
    }
}

impl Debug for dyn Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Value::fmt(self, f)
    }
}
