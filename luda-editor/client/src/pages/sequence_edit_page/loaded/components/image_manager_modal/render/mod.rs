use super::*;

impl ImageManagerModal {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        on_top(event_trap(
            render([
                simple_rect(props.wh, Color::WHITE, 1.px(), Color::BLACK),
                table::vertical([
                    table::fixed(36.px(), |wh| {
                        table::horizontal([
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
                    table::ratio(1, |wh| self.image_table.render(image_table::Props { wh })),
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
