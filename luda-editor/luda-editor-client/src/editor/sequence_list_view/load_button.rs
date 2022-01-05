use crate::editor::events::EditorEvent;
use namui::{render, RenderingTree, Wh};

use super::SequenceListView;

impl SequenceListView {
    pub fn render_load_button(&self, wh: Wh<f32>, path: &String) -> RenderingTree {
        render![
            self.render_button_background(wh)
                .attach_event(move |builder| {
                    let path = path.clone();
                    builder.on_mouse_down(Box::new(move |_| {
                        let path = path.clone();
                        namui::event::send(Box::new(EditorEvent::SequenceLoadEvent { path }));
                    }))
                }),
            self.render_button_text(wh, "Load".to_string())
        ]
    }
}
