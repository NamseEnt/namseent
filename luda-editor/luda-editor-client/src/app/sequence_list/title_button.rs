use namui::{render, Wh};

use super::{
    events::SequenceListEvent, rounded_rectangle::RoundedRectangleColor, types::RenderingTreeRow,
    SequenceList, BUTTON_HEIGHT,
};

impl SequenceList {
    pub fn render_title_button(&self, width: f32, title: &String) -> RenderingTreeRow {
        let path = format!("sequence/{}", title);
        let button_wh = Wh {
            width,
            height: BUTTON_HEIGHT,
        };

        RenderingTreeRow::new(
            render![
                self.render_rounded_rectangle(button_wh, RoundedRectangleColor::DarkGray)
                    .with_mouse_cursor(namui::MouseCursor::Pointer)
                    .attach_event(move |builder| {
                        let path = path.clone();
                        builder.on_mouse_down(move |_| {
                            let path = path.clone();
                            namui::event::send(
                                SequenceListEvent::SequenceTitleButtonClickedEvent { path },
                            );
                        })
                    }),
                self.render_button_text(button_wh, title.clone()),
            ],
            BUTTON_HEIGHT,
        )
    }
}
