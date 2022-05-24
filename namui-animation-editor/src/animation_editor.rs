use super::*;
use namui::prelude::*;
// mod graph_editor;

pub struct AnimationEditor {
    // graph_editor: graph_editor::GraphEditor,
    time_point_editor: time_point_editor::TimePointEditor,
    animation_history: AnimationHistory,
}

pub struct Props {
    pub wh: Wh<f32>,
}

pub(crate) enum Event {}

impl AnimationEditor {
    pub fn new(animation: &animation::Animation) -> Self {
        let animation_history = AnimationHistory::new(animation.clone());
        Self {
            // graph_editor: graph_editor::GraphEditor::new(animation.clone()),
            time_point_editor: time_point_editor::TimePointEditor::new(animation_history.clone()),
            animation_history,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        // self.graph_editor.update(event);
        self.time_point_editor.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = self.animation_history.get_preview();
        self.time_point_editor.render(time_point_editor::Props {
            wh: props.wh,
            animation: &animation,
        })
    }
}
