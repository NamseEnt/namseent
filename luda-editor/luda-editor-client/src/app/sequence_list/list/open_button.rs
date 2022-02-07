use crate::app::{
    editor::Editor,
    events::RouterEvent,
    sequence_list::common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    types::Sequence,
};
use namui::{render, RenderingTree, Wh};
use std::sync::Arc;

pub fn render_open_button(wh: Wh<f32>, path: &String, sequence: &Arc<Sequence>) -> RenderingTree {
    render![
        render_rounded_rectangle(wh, RoundedRectangleColor::Blue)
            .attach_event(move |builder| {
                let sequence = sequence.clone();
                let path = path.clone();
                builder.on_mouse_down(move |_| {
                    let sequence = sequence.clone();
                    let path = path.clone();
                    namui::event::send(RouterEvent::PageChangeToEditorEvent(Box::new(
                        move |context| -> Editor {
                            Editor::new(context.socket.clone(), sequence.clone(), &path)
                        },
                    )));
                })
            })
            .with_mouse_cursor(namui::MouseCursor::Pointer),
        render_button_text(wh, "Open".to_string())
    ]
}
