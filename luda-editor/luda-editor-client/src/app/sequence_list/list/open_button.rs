use crate::app::{
    sequence_list::{
        common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
        events::SequenceListEvent,
        types::SequenceOpenState,
    },
    types::Sequence,
};
use namui::prelude::*;
use std::sync::Arc;

pub fn render_open_button(
    wh: Wh<Px>,
    title: &String,
    sequence: &Arc<Sequence>,
    open_state: &SequenceOpenState,
) -> RenderingTree {
    render([
        render_rounded_rectangle(wh, RoundedRectangleColor::Blue)
            .attach_event(move |builder| {
                let title = title.clone();
                let sequence = sequence.clone();
                builder.on_mouse_down_in(move |_| {
                    let title = title.clone();
                    let sequence = sequence.clone();
                    namui::event::send(SequenceListEvent::SequenceOpenButtonClickedEvent {
                        title,
                        sequence,
                    });
                });
            })
            .with_mouse_cursor(namui::MouseCursor::Pointer),
        render_button_text(wh, button_text(open_state)),
    ])
}

fn button_text(open_state: &SequenceOpenState) -> String {
    match open_state {
        SequenceOpenState::Idle => "Open".to_string(),
        SequenceOpenState::Opening => "Opening...".to_string(),
        SequenceOpenState::Failed { message } => format!("{}", message),
    }
}
