use super::{
    events::SequenceListEvent,
    rounded_rectangle::RoundedRectangleColor,
    types::{RenderingTreeRow, RenderingTreeRows, SequenceLoadStateDetail},
    SequenceList, BUTTON_HEIGHT, MARGIN,
};
use crate::app::sequence_list::SPACING;
use namui::{render, Wh};

impl SequenceList {
    pub fn render_list_item(&self, width: f32, title: &String) -> RenderingTreeRow {
        let path = format!("sequence/{}", title);
        let element_width = width - 2.0 * MARGIN;
        let button_wh = Wh {
            width: element_width,
            height: BUTTON_HEIGHT,
        };
        let title_load_state = self.sequence_load_state_map.get(&path);
        let is_open = title_load_state.is_some();
        let mut elements: Vec<RenderingTreeRow> = Vec::new();

        elements.push(RenderingTreeRow::new(
            render![
                self.render_rounded_rectangle(button_wh, RoundedRectangleColor::DarkGray)
                    .with_mouse_cursor(namui::MouseCursor::Pointer)
                    .attach_event(move |builder| {
                        let path = path.clone();
                        builder.on_mouse_down(move |_| match is_open {
                            true => {
                                let path = path.clone();
                                namui::event::send(
                                    SequenceListEvent::SequenceLoadStateUpdateEvent {
                                        path,
                                        state: None,
                                    },
                                );
                            }
                            false => {
                                let path = path.clone();
                                namui::event::send(SequenceListEvent::SequenceLoadEvent { path });
                            }
                        })
                    }),
                self.render_button_text(button_wh, title.clone()),
            ],
            BUTTON_HEIGHT,
        ));

        if let Some(load_state) = title_load_state {
            elements.push(RenderingTreeRow::new(
                match &load_state.detail {
                    SequenceLoadStateDetail::Loading => render![
                        self.render_rounded_rectangle(button_wh, RoundedRectangleColor::Blue),
                        self.render_button_text(button_wh, "Loading...".to_string())
                    ],
                    SequenceLoadStateDetail::Loaded { sequence } => {
                        self.render_open_button(button_wh, title, sequence)
                    }
                    SequenceLoadStateDetail::Failed { error } => render![
                        self.render_rounded_rectangle(button_wh, RoundedRectangleColor::Blue),
                        self.render_button_text(button_wh, format!("Error: {}", error))
                    ],
                },
                BUTTON_HEIGHT,
            ));
        };

        let total_height = elements.height(SPACING) + 2.0 * MARGIN;

        RenderingTreeRow::new(
            render![
                self.render_rounded_rectangle(
                    Wh {
                        width,
                        height: total_height
                    },
                    RoundedRectangleColor::DarkGray
                ),
                namui::translate(MARGIN, MARGIN, elements.render(SPACING)),
            ],
            total_height,
        )
    }
}
