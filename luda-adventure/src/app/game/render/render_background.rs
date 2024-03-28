use namui::*;
use namui_prebuilt::simple_rect;

pub fn render_background() -> namui::RenderingTree {
    simple_rect(
        namui::screen::size(),
        Color::TRANSPARENT,
        0.px(),
        Color::grayscale_f01(0.2),
    )
}
