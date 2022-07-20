use super::*;
use namui::{prelude::*, types::Px};
use namui_prebuilt::table::{ratio, vertical};

pub struct AnimationEditor {
    time_point_editor: time_point_editor::TimePointEditor,
    animation_history: AnimationHistory,
    layer_list_window: layer_list_window::LayerListWindow,
}

pub struct Props {
    pub wh: Wh<Px>,
}

impl AnimationEditor {
    pub fn new(animation: &animation::Animation) -> Self {
        let animation_history = AnimationHistory::new(animation.clone());
        Self {
            time_point_editor: time_point_editor::TimePointEditor::new(animation_history.clone()),
            layer_list_window: layer_list_window::LayerListWindow::new(animation_history.clone()),
            animation_history,
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.time_point_editor.update(event);
        self.layer_list_window.update(event);
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let animation = self.animation_history.get_preview();

        vertical([ratio(1.0, |wh| {
            self.time_point_editor.render(time_point_editor::Props {
                wh,
                animation: &animation,
                layer_list_window: &self.layer_list_window,
            })
        })])(props.wh)
    }
}
