use super::{SequenceList, BUTTON_HEIGHT, MARGIN, SPACING};
use crate::app::sequence_list::types::*;
use namui::{RenderingTree, Wh};

impl SequenceList {
    pub fn render_list(&self, wh: Wh<f32>) -> RenderingTree {
        let inner_width = wh.width - 2.0 * MARGIN;
        let button_wh = Wh {
            width: inner_width,
            height: BUTTON_HEIGHT,
        };
        namui::translate(
            MARGIN,
            MARGIN,
            match &self.sequence_titles_load_state {
                Some(state) => match &state.detail {
                    SequenceTitlesLoadStateDetail::Loading => {
                        self.render_button_text(button_wh, "Loading...".to_string())
                    }
                    SequenceTitlesLoadStateDetail::Loaded { titles } => {
                        let rows: Vec<RenderingTreeRow> = titles
                            .iter()
                            .map(|title| self.render_list_item(inner_width, title))
                            .collect();

                        rows.height(SPACING);
                        rows.render(SPACING)
                    }
                    SequenceTitlesLoadStateDetail::Failed { error } => {
                        self.render_button_text(button_wh, format!("Error: {}", error))
                    }
                },
                None => RenderingTree::Empty,
            },
        )
    }
}
