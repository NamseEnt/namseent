use super::SequenceList;
use crate::app::sequence_list::events::SequenceListEvent;
use namui::{render, RenderingTree, Wh};

impl SequenceList {
    pub fn render_load_button(&self, wh: Wh<f32>, path: &String) -> RenderingTree {
        render![
            self.render_button_background(wh)
                .attach_event(move |builder| {
                    let path = path.clone();
                    builder.on_mouse_down(Box::new(move |_| {
                        let path = path.clone();
                        namui::event::send(Box::new(SequenceListEvent::SequenceLoadEvent { path }));
                    }))
                })
                .with_mouse_cursor(namui::MouseCursor::Pointer),
            self.render_button_text(wh, "Load".to_string())
        ]
    }
}
