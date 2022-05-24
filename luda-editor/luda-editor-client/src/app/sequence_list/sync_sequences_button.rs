use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    events::SequenceListEvent,
};
use namui::{render, RenderingTree, Wh};

pub fn render_sync_sequences_button(wh: Wh<f32>) -> RenderingTree {
    render![
        render_rounded_rectangle(wh, RoundedRectangleColor::Blue)
            .attach_event(move |builder| {
                builder.on_mouse_down(move |_| {
                    namui::event::send(SequenceListEvent::SyncSequencesButtonClickedEvent {});
                });
            })
            .with_mouse_cursor(namui::MouseCursor::Pointer),
        render_button_text(wh, "Sync sequences from Google Spreadsheet".to_string())
            .with_mouse_cursor(namui::MouseCursor::Pointer)
    ]
}
