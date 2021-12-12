use crate::engine;

pub struct TextInput {}

impl engine::Update for TextInput {
    fn update(&mut self, event: &dyn std::any::Any) {
        todo!()
    }
}

impl engine::Render for TextInput {
    fn render(&self) -> engine::RenderingTree {
        todo!()
    }
}
