use super::*;
use namui::prelude::*;
use std::sync::{Arc, RwLock};

pub struct AnimationEditor {
    graph_editor: graph_editor::GraphEditor,
    time_point_editor: time_point_editor::TimePointEditor,
}

pub struct Props {
    pub wh: Wh<f32>,
}

pub(crate) enum Event {}

impl AnimationEditor {
    pub fn new(animation: Arc<RwLock<animation::Animation>>) -> Self {
        let animation = crate::ReadOnlyLock::new(animation);
        Self {
            graph_editor: graph_editor::GraphEditor::new(animation.clone()),
            time_point_editor: time_point_editor::TimePointEditor::new(animation.clone()),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.graph_editor.update(event);
        self.time_point_editor.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        self.time_point_editor
            .render(time_point_editor::Props { wh: props.wh })
    }
}
