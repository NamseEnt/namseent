use super::{
    open_button::render_open_button, preview_slider::render_preview_slider,
    title_button::render_title_button,
};
use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    types::{
        RenderingTreeRow, RenderingTreeRows, SequenceLoadState, SequenceLoadStateDetail,
        SequencePreviewProgressMap,
    },
    BUTTON_HEIGHT, MARGIN, SPACING,
};
use namui::{render, Wh};

pub fn render_list_item(
    width: f32,
    title: &String,
    path: &String,
    title_load_state: Option<&SequenceLoadState>,
    sequence_preview_progress_map: &SequencePreviewProgressMap,
) -> RenderingTreeRow {
    let element_width = width - 2.0 * MARGIN;
    let button_wh = Wh {
        width: element_width,
        height: BUTTON_HEIGHT,
    };
    let mut elements: Vec<RenderingTreeRow> = Vec::new();

    elements.push(render_title_button(element_width, title, path));

    if let Some(load_state) = title_load_state {
        match &load_state.detail {
            SequenceLoadStateDetail::Loading => elements.push(RenderingTreeRow::new(
                render![
                    render_rounded_rectangle(button_wh, RoundedRectangleColor::Blue),
                    render_button_text(button_wh, "Loading...".to_string())
                ],
                button_wh.height,
            )),
            SequenceLoadStateDetail::Loaded { sequence } => {
                elements.push(RenderingTreeRow::new(
                    render_preview_slider(button_wh, path, sequence_preview_progress_map),
                    button_wh.height,
                ));
                elements.push(RenderingTreeRow::new(
                    render_open_button(button_wh, path, &sequence, title),
                    button_wh.height,
                ));
            }
            SequenceLoadStateDetail::Failed { error } => {
                elements.push(RenderingTreeRow::new(
                    render![
                        render_rounded_rectangle(button_wh, RoundedRectangleColor::Blue),
                        render_button_text(button_wh, format!("Error: {}", error))
                    ],
                    button_wh.height,
                ));
            }
        };
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
