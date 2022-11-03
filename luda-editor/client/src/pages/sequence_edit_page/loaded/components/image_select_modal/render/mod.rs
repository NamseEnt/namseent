mod filtered_image_list;
mod image_preview;
mod label_list;
mod recent_images;

use super::*;
use crate::pages::sequence_edit_page::loaded::components::screen_image_list;
use namui_prebuilt::button::{body_text_button, text_button, text_button_fit};

impl ImageSelectModal {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        if let Some(image_edit_modal) = &self.image_edit_modal {
            return image_edit_modal.render(image_edit_modal::Props { wh: props.wh });
        }
        on_top(
            render([
                simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
                table::vertical([
                    table::ratio(
                        1,
                        table::horizontal([
                            table::ratio(5, |wh| {
                                screen_image_list::render(screen_image_list::Props {
                                    wh,
                                    cut: &props.cut,
                                    project_id: self.project_id,
                                    selected_index: self.selected_screen_image_index,
                                    on_click: move |index| {
                                        namui::event::send(InternalEvent::SelectScreenImageIndex {
                                            index,
                                        })
                                    },
                                })
                            }),
                            table::ratio(1, |wh| {
                                body_text_button(
                                    Rect::from_xy_wh(Xy::zero(), wh),
                                    "Edit Screen",
                                    Color::WHITE,
                                    Color::WHITE,
                                    1.px(),
                                    Color::BLACK,
                                    TextAlign::Center,
                                    || {
                                        namui::event::send(InternalEvent::EditScreenPressed);
                                    },
                                )
                            }),
                        ]),
                    ),
                    table::ratio(
                        4,
                        table::horizontal([
                            table::ratio(
                                1.0,
                                table::vertical([
                                    table::ratio(1.0, |wh| {
                                        self.render_recent_images(recent_images::Props {
                                            wh,
                                            recent_selected_image_ids: props
                                                .recent_selected_image_ids,
                                        })
                                    }),
                                    table::ratio(2.0, |wh| {
                                        self.render_label_list(label_list::Props { wh })
                                    }),
                                    table::ratio(2.0, |wh| {
                                        self.render_filtered_image_list(
                                            filtered_image_list::Props { wh },
                                        )
                                    }),
                                ]),
                            ),
                            table::ratio(1.0, |wh| {
                                self.render_image_preview(image_preview::Props { wh })
                            }),
                        ]),
                    ),
                ])(props.wh),
            ])
            .attach_event(|builder| {
                builder
                    .on_mouse_down_in(|event| {
                        event.stop_propagation();
                    })
                    .on_mouse_down_out(|event| {
                        event.stop_propagation();
                        namui::event::send(Event::Close)
                    });
            }),
        )
    }
}
