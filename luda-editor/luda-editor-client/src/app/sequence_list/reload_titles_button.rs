use super::SequenceList;
use crate::app::sequence_list::{
    events::SequenceListEvent, rounded_rectangle::RoundedRectangleColor,
};
use namui::{render, RenderingTree, Wh};

impl SequenceList {
    pub fn render_reload_titles_button(&self, wh: Wh<f32>) -> RenderingTree {
        render![
            self.render_rounded_rectangle(wh, RoundedRectangleColor::Blue)
                .attach_event(move |builder| {
                    builder.on_mouse_down(Box::new(move |_| {
                        namui::event::send(SequenceListEvent::SequenceTitlesLoadEvent {});
                    }))
                })
                .with_mouse_cursor(namui::MouseCursor::Pointer),
            self.render_button_text(wh, "Reload titles".to_string())
        ]
    }
}
