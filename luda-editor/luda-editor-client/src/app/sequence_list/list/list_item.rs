use super::{
    open_button::render_open_button, preview_slider::render_preview_slider,
    title_button::render_title_button,
};
use crate::app::{
    sequence_list::{
        common::{render_rounded_rectangle, RoundedRectangleColor},
        types::{RenderingTreeRow, RenderingTreeRows, SequencePreviewProgressMap},
        BUTTON_HEIGHT, MARGIN, SPACING,
    },
    types::Sequence,
};
use namui::prelude::*;
use std::sync::Arc;

pub fn render_list_item(
    width: Px,
    title: &String,
    path: &String,
    sequence: &Arc<Sequence>,
    sequence_preview_progress_map: &SequencePreviewProgressMap,
    is_item_opened: bool,
) -> RenderingTreeRow {
    let element_width = width - 2.0 * MARGIN;
    let button_wh = Wh {
        width: element_width,
        height: BUTTON_HEIGHT,
    };
    let mut elements: Vec<RenderingTreeRow> = Vec::new();

    elements.push(render_title_button(element_width, title));

    if is_item_opened {
        elements.push(RenderingTreeRow::new(
            render_preview_slider(button_wh, title, sequence_preview_progress_map),
            button_wh.height,
        ));
        elements.push(RenderingTreeRow::new(
            render_open_button(button_wh, path, &sequence, title),
            button_wh.height,
        ));
    }
    let total_height = elements.height(SPACING) + 2.0 * MARGIN;

    RenderingTreeRow::new(
        render([
            render_rounded_rectangle(
                Wh {
                    width,
                    height: total_height,
                },
                RoundedRectangleColor::DarkGray,
            ),
            namui::translate(MARGIN, MARGIN, elements.render(SPACING)),
        ]),
        total_height,
    )
}
