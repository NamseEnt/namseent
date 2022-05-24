use std::sync::Arc;

#[derive(Debug)]
pub enum Event {
    AnimationUpdated(Arc<namui::animation::Animation>),
    Error(String),
}
