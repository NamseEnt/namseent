use super::*;
use namui_prebuilt::button::text_button;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageEditModal {
    pub fn render_done_button(&self, props: Props) -> namui::RenderingTree {
        table::padding(16.px(), |wh| {
            text_button(
                Rect::zero_wh(wh),
                "Done",
                Color::BLACK,
                Color::BLACK,
                2.px(),
                Color::WHITE,
                || {
                    namui::event::send(InternalEvent::DonePressed);
                },
            )
        })(props.wh)
    }
}
