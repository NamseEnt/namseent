use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    events::SequenceListEvent,
    types::RenderingTreeRow,
    BUTTON_HEIGHT,
};
use namui::prelude::*;

pub fn render_title_button(width: Px, title: &String) -> RenderingTreeRow {
    let button_wh = Wh {
        width,
        height: BUTTON_HEIGHT,
    };

    RenderingTreeRow::new(
        render([
            render_rounded_rectangle(button_wh, RoundedRectangleColor::DarkGray)
                .with_mouse_cursor(namui::MouseCursor::Pointer)
                .attach_event(move |builder| {
                    let title = title.clone();
                    builder.on_mouse_down(move |_| {
                        let title = title.clone();
                        namui::event::send(SequenceListEvent::SequenceTitleButtonClickedEvent {
                            title,
                        });
                    });
                }),
            render_button_text(button_wh, title.clone()),
        ]),
        BUTTON_HEIGHT,
    )
}
