mod filtered_image_list;
mod label_filter;
mod recently_used;
mod selected_image_preview;

use super::*;

impl CharacterEditModal {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let modal_wh = props.wh * 2.0 / 3.0;
        let modal = translate(
            props.wh.width / 2 - modal_wh.width / 2,
            props.wh.height / 2 - modal_wh.height / 2,
            render([
                simple_rect(modal_wh, Color::WHITE, 1.px(), Color::BLACK),
                table::horizontal([
                    table::ratio(
                        2,
                        table::vertical([
                            table::fixed(80.px(), |wh| {
                                self.render_recently_used(recently_used::Props {})
                            }),
                            table::ratio(2, |wh| self.render_label_filter(label_filter::Props {})),
                            table::ratio(1, |wh| {
                                self.render_filtered_image_list(filtered_image_list::Props {})
                            }),
                        ]),
                    ),
                    table::ratio(2, |wh| {
                        self.render_selected_image_preview(selected_image_preview::Props {})
                    }),
                ])(modal_wh),
            ]),
        );

        on_top(modal.attach_event(|builder| {
            builder
                .on_mouse_down_in(|event| {
                    event.stop_propagation();
                })
                .on_mouse_down_out(|event| {
                    event.stop_propagation();
                    namui::event::send(Event::Close)
                });
        }))
    }
}
