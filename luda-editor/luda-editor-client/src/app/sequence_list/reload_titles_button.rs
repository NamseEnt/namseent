use crate::app::sequence_list::{
    common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    events::SequenceListEvent,
};
use namui::{render, RenderingTree, Wh};

pub fn render_reload_titles_button(wh: Wh<f32>) -> RenderingTree {
    render![
        render_rounded_rectangle(wh, RoundedRectangleColor::Blue)
            .attach_event(move |builder| {
                builder.on_mouse_down(move |_| {
                    namui::event::send(
                        SequenceListEvent::SequenceReloadTitlesButtonClickedEvent {},
                    );
                })
            })
            .with_mouse_cursor(namui::MouseCursor::Pointer),
        render_button_text(wh, "Reload titles".to_string())
    ]
}
