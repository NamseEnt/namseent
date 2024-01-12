use namui::prelude::*;
use namui_prebuilt::simple_rect;

#[namui::component]
pub struct MiniMap {
    pub wh: Wh<Px>,
    pub length: usize,
    pub range: std::ops::Range<usize>,
}

impl Component for MiniMap {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let MiniMap { wh, length, range } = self;

        let left_px = wh.width * range.start.min(range.end) as f32 / length as f32;
        let right_px = wh.width * range.end.max(range.start) as f32 / length as f32;

        let front = translate(
            left_px,
            0.px(),
            simple_rect(
                Wh::new(right_px - left_px, wh.height),
                Color::grayscale_f01(1.0),
                1.px(),
                Color::grayscale_f01(0.8),
            ),
        );

        let background = simple_rect(
            wh,
            Color::grayscale_f01(0.6),
            1.px(),
            Color::grayscale_f01(0.5),
        );

        ctx.component(front);
        ctx.component(background);

        ctx.done()
    }
}
