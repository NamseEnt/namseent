mod filtered_image_list;
mod image_preview;
mod label_list;
mod recent_images;

use super::*;

impl ImageSelectModal {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        if let Some(image_edit_modal) = &self.image_edit_modal {
            return image_edit_modal.render(image_edit_modal::Props { wh: props.wh });
        }
        on_top(
            render([
                simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
                table::horizontal([
                    table::ratio(
                        1.0,
                        table::vertical([
                            table::ratio(1.0, |wh| {
                                self.render_recent_images(recent_images::Props { wh })
                            }),
                            table::ratio(2.0, |wh| {
                                self.render_label_list(label_list::Props { wh })
                            }),
                            table::ratio(2.0, |wh| {
                                self.render_filtered_image_list(filtered_image_list::Props { wh })
                            }),
                        ]),
                    ),
                    table::ratio(1.0, |wh| {
                        self.render_image_preview(image_preview::Props { wh })
                    }),
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
