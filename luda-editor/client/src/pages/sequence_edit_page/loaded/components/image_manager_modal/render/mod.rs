use super::*;
use namui_prebuilt::typography::text_fit;

impl ImageManagerModal {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        let saving_count = self.image_table.saving_count;
        on_top(event_trap(
            render([
                simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
                table::vertical([
                    table::fixed(36.px(), |wh| {
                        table::horizontal([
                            table::ratio(1, |_| RenderingTree::Empty),
                            table::fit(
                                table::FitAlign::LeftTop,
                                text_fit(
                                    wh.height,
                                    format!(
                                        "Save Status: {}",
                                        if saving_count == 0 {
                                            format!("All data saved")
                                        } else {
                                            format!("Saving...(Please wait for it)")
                                        }
                                    ),
                                    Color::WHITE,
                                    100.px(),
                                ),
                            ),
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
                                    move || {
                                        if saving_count == 0 {
                                            namui::event::send(Event::Close)
                                        }
                                    },
                                ),
                            ),
                        ])(wh)
                    }),
                    table::ratio(1, |wh| self.image_table.render(image_table::Props { wh })),
                ])(props.wh),
            ])
            .attach_event(move |builder| {
                builder.on_mouse_down_out(move |event| {
                    event.stop_propagation();
                    if saving_count == 0 {
                        namui::event::send(Event::Close)
                    }
                });
            }),
        ))
    }
}
