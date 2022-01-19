use super::{
    rounded_rectangle::RoundedRectangleColor,
    types::{RenderingTreeRow, RenderingTreeRows, SequenceLoadStateDetail},
    SequenceList, BUTTON_HEIGHT, MARGIN,
};
use crate::app::sequence_list::SPACING;
use namui::{render, Wh};

impl SequenceList {
    pub fn render_list_item(&self, width: f32, title: &String) -> RenderingTreeRow {
        let path = format!("sequence/{}", title);
        let element_width = width - 2.0 * MARGIN;
        let button_wh = Wh {
            width: element_width,
            height: BUTTON_HEIGHT,
        };
        let mut elements: Vec<RenderingTreeRow> = Vec::new();
        elements.push(RenderingTreeRow::new(
            self.render_button_text(
                Wh {
                    width: element_width,
                    height: BUTTON_HEIGHT,
                },
                title.clone(),
            ),
            BUTTON_HEIGHT,
        ));

        elements.push(RenderingTreeRow::new(
            match self.sequence_load_state_map.get(&path) {
                Some(load_state) => match &load_state.detail {
                    SequenceLoadStateDetail::Loading => render![
                        self.render_rounded_rectangle(button_wh, RoundedRectangleColor::Blue),
                        self.render_button_text(button_wh, "Loading...".to_string())
                    ],
                    SequenceLoadStateDetail::Loaded { sequence } => {
                        self.render_open_button(button_wh, title, sequence)
                    }
                    SequenceLoadStateDetail::Failed { error } => render![
                        self.render_rounded_rectangle(button_wh, RoundedRectangleColor::Blue),
                        self.render_button_text(button_wh, format!("Error: {}", error))
                    ],
                },
                None => self.render_load_button(button_wh, &path),
            },
            BUTTON_HEIGHT,
        ));

        let total_height = elements.height(SPACING) + 2.0 * MARGIN;

        RenderingTreeRow::new(
            render![
                self.render_rounded_rectangle(
                    Wh {
                        width,
                        height: total_height
                    },
                    RoundedRectangleColor::DarkGray
                ),
                namui::translate(MARGIN, MARGIN, elements.render(SPACING)),
            ],
            total_height,
        )
    }
}
