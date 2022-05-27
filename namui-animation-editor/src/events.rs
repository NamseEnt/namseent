use std::sync::Arc;

#[derive(Debug)]
pub enum Event {
    AddLayerButtonClicked,
    UpdateLayer(Arc<namui::animation::Layer>),
    Error(String),
}
