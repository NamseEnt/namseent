use crate::app::{
    editor::Editor,
    events::RouterEvent,
    sequence_list::common::{render_button_text, render_rounded_rectangle, RoundedRectangleColor},
    types::Sequence,
};
use namui::prelude::*;
use std::sync::Arc;

pub fn render_open_button(wh: Wh<Px>, sequence: &Arc<Sequence>, title: &String) -> RenderingTree {
    render([
        render_rounded_rectangle(wh, RoundedRectangleColor::Blue)
            .attach_event(move |builder| {
                let sequence = sequence.clone();
                let title = title.clone();
                builder.on_mouse_down_in(move |_| {
                    let sequence = sequence.clone();
                    let title = title.clone();
                    namui::event::send(RouterEvent::PageChangeToEditorEvent(Box::new(
                        move |context| -> Editor {
                            Editor::new(
                                context.storage.clone(),
                                sequence.clone(),
                                &title,
                                context.meta_container.clone(),
                                context.camera_angle_image_loader.clone(),
                            )
                        },
                    )));
                });
            })
            .with_mouse_cursor(namui::MouseCursor::Pointer),
        render_button_text(wh, "Open".to_string()),
    ])
}
