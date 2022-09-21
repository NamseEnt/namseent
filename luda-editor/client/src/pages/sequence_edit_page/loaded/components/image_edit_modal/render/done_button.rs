use super::*;
use namui_prebuilt::button::text_button;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageEditModal {
    pub fn render_done_button(&self, props: Props) -> namui::RenderingTree {
        text_button(
            Rect::zero_wh(props.wh),
            "Done",
            Color::BLACK,
            Color::BLACK,
            1.px(),
            Color::WHITE,
            || {
                namui::event::send(InternalEvent::DonePressed);
            },
        )
    }
}
