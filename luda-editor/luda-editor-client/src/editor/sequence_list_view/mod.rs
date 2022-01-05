use namui::{render, Wh, XywhRect};

use crate::editor::types::SequenceLoadStateDetail;

use super::types::SequenceLoadStateMap;

mod button_background;
mod button_text;
mod load_button;
mod open_button;

const BUTTON_HEIGHT: f32 = 36.0;

pub struct SequenceListViewProps<'a> {
    pub xywh: XywhRect<f32>,
    pub sequence_load_state_map: &'a SequenceLoadStateMap,
}

pub struct SequenceListView {}

impl SequenceListView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(&mut self, _: &dyn std::any::Any) {}
    pub fn render<'a>(&self, props: &SequenceListViewProps<'a>) -> namui::RenderingTree {
        let button_wh = Wh {
            width: props.xywh.width,
            height: BUTTON_HEIGHT,
        };
        let test_path: String = "sequence/testSequence.json".to_string();
        match props.sequence_load_state_map.get(&test_path) {
            Some(load_state) => match &load_state.detail {
                SequenceLoadStateDetail::Loading => render![
                    self.render_button_background(button_wh),
                    self.render_button_text(button_wh, "Loading...".to_string())
                ],
                SequenceLoadStateDetail::Loaded { sequence } => {
                    self.render_open_button(button_wh, &test_path, &sequence)
                }
                SequenceLoadStateDetail::Failed { error } => render![
                    self.render_button_background(button_wh),
                    self.render_button_text(button_wh, format!("Error: {}", error))
                ],
            },
            None => self.render_load_button(button_wh, &test_path),
        }
    }
}
