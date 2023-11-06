use super::component::Component;
use std::sync::Arc;

pub struct Render {
    vec: Vec<Arc<dyn Component>>,
}
impl Render {
    pub(crate) fn new() -> Render {
        Render { vec: Vec::new() }
    }
    pub fn add(mut self, component: impl Component + 'static) -> Self {
        self.vec.push(Arc::new(component));
        self
    }
    pub fn add_arc(mut self, component: Arc<dyn Component>) -> Self {
        self.vec.push(component);
        self
    }

    pub fn add_component(&mut self, component: impl Component + 'static) {
        self.vec.push(Arc::new(component));
    }

    pub(crate) fn into_children(self) -> impl Iterator<Item = Arc<dyn Component>> {
        self.vec.into_iter()
    }
}
