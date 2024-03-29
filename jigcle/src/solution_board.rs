use namui::*;
use namui_prebuilt::simple_rect;

#[component]
pub struct SolutionBoard {
    pub wh_counts: Wh<usize>,
    pub image_wh: Wh<Px>,
}

impl Component for SolutionBoard {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh_counts,
            image_wh,
        } = self;

        let piece_wh = Wh::new(
            image_wh.width / wh_counts.width as f32,
            image_wh.height / wh_counts.height as f32,
        );

        for y in 0..wh_counts.height {
            for x in 0..wh_counts.width {
                let radius = 3.px();
                ctx.compose(|ctx| {
                    ctx.translate(
                        piece_wh.as_xy() * Xy::new(x as f32 + 0.5, y as f32 + 0.5)
                            - Xy::single(radius),
                    )
                    .add(simple_rect(
                        Wh::new(radius * 2, radius * 2),
                        Color::TRANSPARENT,
                        0.px(),
                        Color::grayscale_f01(0.5),
                    ));
                });
            }
        }

        ctx.add(simple_rect(image_wh, Color::BLACK, 2.px(), Color::WHITE));
    }
}
