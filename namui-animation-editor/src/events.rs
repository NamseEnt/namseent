use std::sync::Arc;

pub enum Event {
    AddLayerButtonClicked,
    UpdateLayer(Arc<namui::animation::Layer>),
    Error(String),
}
