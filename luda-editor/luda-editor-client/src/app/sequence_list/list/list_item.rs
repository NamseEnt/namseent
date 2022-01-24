use super::{open_button::render_open_button, title_button::render_title_button};
use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    types::{RenderingTreeRow, RenderingTreeRows, SequenceLoadState, SequenceLoadStateDetail},
    BUTTON_HEIGHT, MARGIN, SPACING,
};
use namui::{render, Wh};

pub fn render_list_item(
    width: f32,
    title: &String,
    title_load_state: Option<&SequenceLoadState>,
) -> RenderingTreeRow {
    let element_width = width - 2.0 * MARGIN;
    let button_wh = Wh {
        width: element_width,
        height: BUTTON_HEIGHT,
    };
    let mut elements: Vec<RenderingTreeRow> = Vec::new();

    elements.push(render_title_button(element_width, title));

    if let Some(load_state) = title_load_state {
        elements.push(RenderingTreeRow::new(
            match &load_state.detail {
                SequenceLoadStateDetail::Loading => render![
                    render_rounded_rectangle(button_wh, RoundedRectangleColor::Blue),
                    render_button_text(button_wh, "Loading...".to_string())
                ],
                SequenceLoadStateDetail::Loaded { sequence } => {
                    render_open_button(button_wh, title, &sequence)
                }
                SequenceLoadStateDetail::Failed { error } => render![
                    render_rounded_rectangle(button_wh, RoundedRectangleColor::Blue),
                    render_button_text(button_wh, format!("Error: {}", error))
                ],
            },
            BUTTON_HEIGHT,
        ));
    };

    let total_height = elements.height(SPACING) + 2.0 * MARGIN;

    RenderingTreeRow::new(
        render![
            render_rounded_rectangle(
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
