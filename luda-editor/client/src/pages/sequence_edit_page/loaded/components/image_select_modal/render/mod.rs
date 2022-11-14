mod filtered_image_list;
mod image_preview;
mod label_list;
mod recent_images;

use super::*;
use crate::pages::sequence_edit_page::loaded::components::screen_image_list;
use namui_prebuilt::button::body_text_button;

impl ImageSelectModal {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        if let Some(image_edit_modal) = &self.image_edit_modal {
            return image_edit_modal.render(image_edit_modal::Props { wh: props.wh });
        }
        if let Some(screen_editor) = &self.screen_editor {
            return event_trap(screen_editor.render(screen_editor::Props {
                wh: props.wh,
                project_shared_data: props.project_shared_data,
                cut: props.cut,
            }));
        }

        let screen_images = props.cut.screen_images.clone();

        on_top(event_trap(
            render([
                simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
                table::vertical([
                    table::fixed(20.px(), |wh| {
                        table::horizontal([
                            table::fit(
                                table::FitAlign::RightBottom,
                                button::text_button_fit(
                                    wh.height,
                                    "Upload Bulk Images",
                                    Color::WHITE,
                                    Color::WHITE,
                                    2.px(),
                                    Color::BLACK,
                                    12.px(),
                                    || namui::event::send(InternalEvent::RequestUploadBulkImages),
                                ),
                            ),
                            table::ratio(1, |_| RenderingTree::Empty),
                            table::fit(
                                table::FitAlign::RightBottom,
                                button::text_button_fit(
                                    wh.height,
                                    "Exit",
                                    Color::WHITE,
                                    Color::WHITE,
                                    2.px(),
                                    Color::BLACK,
                                    12.px(),
                                    || namui::event::send(Event::Close),
                                ),
                            ),
                        ])(wh)
                    }),
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
                                    {
                                        let screen_images = screen_images.clone();
                                        move || {
                                            namui::event::send(InternalEvent::EditScreenPressed {
                                                screen_images: screen_images.clone(),
                                            });
                                        }
                                    },
                                )
                            }),
                        ]),
                    ),
                    table::ratio(4, |wh| {
                        if let Some(selected_screen_image_index) = self.selected_screen_image_index
                        {
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
                                    self.render_image_preview(
                                        image_preview::Props { wh },
                                        selected_screen_image_index,
                                        &screen_images,
                                    )
                                }),
                            ])(wh)
                        } else {
                            RenderingTree::Empty
                        }
                    }),
                ])(props.wh),
            ])
            .attach_event(|builder| {
                builder.on_mouse_down_out(|event| {
                    event.stop_propagation();
                    namui::event::send(Event::Close)
                });
            }),
        ))
    }
}
