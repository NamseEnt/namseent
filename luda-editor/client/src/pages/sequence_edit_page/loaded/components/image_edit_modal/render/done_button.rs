use super::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageEditModal {
    pub fn render_done_button(&self, props: Props) -> namui::RenderingTree {
        let padding = 12.px();
        let button_height = props.wh.height - padding * 2;
        table::horizontal([
            table::ratio(1, |_wh| RenderingTree::Empty),
            table::fit(
                table::FitAlign::CenterMiddle,
                button::text_button_fit(
                    button_height,
                    "Cancel",
                    Color::WHITE,
                    Color::WHITE,
                    1.px(),
                    Color::BLACK,
                    padding,
                    [MouseButton::Left],
                    |_| namui::event::send(Event::Close),
                )
                .padding(12.px()),
            ),
            table::fit(
                table::FitAlign::CenterMiddle,
                button::text_button_fit(
                    button_height,
                    "Done",
                    Color::BLACK,
                    Color::BLACK,
                    1.px(),
                    Color::WHITE,
                    padding,
                    [MouseButton::Left],
                    move |_| namui::event::send(InternalEvent::DonePressed),
                )
                .padding(12.px()),
            ),
        ])(props.wh)
    }
}
