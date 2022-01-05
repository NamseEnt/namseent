use namui::{render, RenderingTree, Wh};

use crate::editor::{
    events::EditorEvent,
    types::{EditorPageChangeEventDetail, Sequence},
};

use super::SequenceListView;

impl SequenceListView {
    pub fn render_open_button(
        &self,
        wh: Wh<f32>,
        path: &String,
        sequence: &Sequence,
    ) -> RenderingTree {
        render![
            self.render_button_background(wh)
                .attach_event(move |builder| {
                    let sequence = sequence.clone();
                    let path = path.clone();
                    builder.on_mouse_down(Box::new(move |_| {
                        let sequence = sequence.clone();
                        let path = path.clone();
                        namui::event::send(Box::new(EditorEvent::PageChangeEvent {
                            detail: EditorPageChangeEventDetail::Editor { path, sequence },
                        }));
                    }))
                }),
            self.render_button_text(wh, "Open".to_string())
        ]
    }
}
