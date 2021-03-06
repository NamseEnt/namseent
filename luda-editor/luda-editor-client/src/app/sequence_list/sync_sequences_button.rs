use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    events::SequenceListEvent,
};
use namui::prelude::*;

pub fn render_sync_sequences_button(wh: Wh<Px>) -> RenderingTree {
    render([
        render_rounded_rectangle(wh, RoundedRectangleColor::Blue)
            .attach_event(move |builder| {
                builder.on_mouse_down_in(move |_| {
                    namui::event::send(SequenceListEvent::SyncSequencesButtonClickedEvent {});
                });
            })
            .with_mouse_cursor(namui::MouseCursor::Pointer),
        render_button_text(wh, "Sync sequences from Google Spreadsheet".to_string()),
    ])
}
