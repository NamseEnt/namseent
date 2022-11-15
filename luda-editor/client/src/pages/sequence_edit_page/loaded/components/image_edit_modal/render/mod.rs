mod done_button;
mod image_viewer;
mod label_input;
mod label_list;

use super::*;

impl ImageEditModal {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        on_top(event_trap(
            render([
                simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
                table::horizontal([
                    table::ratio(1.0, |wh| {
                        self.render_image_viewer(image_viewer::Props { wh })
                    }),
                    table::ratio(
                        2.0,
                        table::vertical([
                            table::ratio(1.0, |wh| {
                                self.render_label_input(label_input::Props { wh })
                            }),
                            table::ratio(4.0, |wh| {
                                self.render_label_list(label_list::Props { wh })
                            }),
                            table::ratio(1.0, |wh| {
                                self.render_done_button(done_button::Props { wh })
                            }),
                        ]),
                    ),
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
