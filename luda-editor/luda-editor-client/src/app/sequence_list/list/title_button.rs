use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    events::SequenceListEvent,
    types::RenderingTreeRow,
    BUTTON_HEIGHT,
};
use namui::{render, Wh};

pub fn render_title_button(width: f32, title: &String) -> RenderingTreeRow {
    let path = format!("sequence/{}", title);
    let button_wh = Wh {
        width,
        height: BUTTON_HEIGHT,
    };

    RenderingTreeRow::new(
        render![
            render_rounded_rectangle(button_wh, RoundedRectangleColor::DarkGray)
                .with_mouse_cursor(namui::MouseCursor::Pointer)
                .attach_event(move |builder| {
                    let path = path.clone();
                    builder.on_mouse_down(move |_| {
                        let path = path.clone();
                        namui::event::send(SequenceListEvent::SequenceTitleButtonClickedEvent {
                            path,
                        });
                    })
                }),
            render_button_text(button_wh, title.clone()),
        ],
        BUTTON_HEIGHT,
    )
}
