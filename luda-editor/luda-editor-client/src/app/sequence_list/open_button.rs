use namui::{render, RenderingTree, Wh};

use crate::app::{editor::Editor, events::RouterEvent, types::Sequence};

use super::SequenceList;

impl SequenceList {
    pub fn render_open_button(
        &self,
        wh: Wh<f32>,
        path: &String,
        sequence: &Sequence,
    ) -> RenderingTree {
        let sequence = sequence.clone();
        let _path = path.clone();
        render![
            self.render_button_background(wh)
                .attach_event(move |builder| {
                    let sequence = sequence.clone();
                    builder.on_mouse_down(Box::new(move |_| {
                        let sequence = sequence.clone();
                        namui::event::send(Box::new(RouterEvent::PageChangeToEditorEvent(
                            Box::new(move |context| -> Editor {
                                Editor::new(
                                    Wh {
                                        width: context.screen_size.width,
                                        height: context.screen_size.height,
                                    },
                                    context.socket.clone(),
                                    sequence.clone(),
                                )
                            }),
                        )));
                    }))
                }),
            self.render_button_text(wh, "Open".to_string())
        ]
    }
}
