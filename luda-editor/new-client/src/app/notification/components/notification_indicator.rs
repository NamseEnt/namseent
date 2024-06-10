use namui::*;

pub struct LoadingIndicator {
    pub wh: Wh<Px>,
    pub color: Color,
}
impl Component for LoadingIndicator {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, color } = self;
        let stroke_width = wh.height / 8.0;
        let wh = Wh {
            width: wh.width - stroke_width * 2.0,
            height: wh.height - stroke_width * 2.0,
        };
        let xy = Xy::single(stroke_width);
        let now = now();
        let start_angle = Angle::Degree(360.0 * (now.as_seconds() % 1.0));
        let delta_angle = Angle::Radian(now.as_seconds().sin() * 3.0);
        let path = Path::new().add_arc(Rect::from_xy_wh(Xy::zero(), wh), start_angle, delta_angle);
        let paint = Paint::new()
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(wh.height / 8.0)
            .set_color(color);
        ctx.compose(|ctx| {
            ctx.translate(xy).add(namui::path(path, paint));
        });

        
    }
}
