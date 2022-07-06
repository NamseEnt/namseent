use super::*;

impl WysiwygWindow {
    pub(crate) fn render_viewport(&self) -> namui::RenderingTree {
        simple_rect(
            Wh {
                width: px(1920.0),
                height: px(1080.0),
            },
            Color::from_u8(0xED, 0x70, 0x14, 255),
            px(1.0),
            Color::TRANSPARENT,
        )
    }
}
