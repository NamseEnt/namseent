use crate::*;

pub struct PositionTool<'a> {
    pub wh: Wh<Px>,
    pub position: Xy<f32>,
    pub on_change_position: &'a dyn Fn(Xy<f32>),
}

impl Component for PositionTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            position,
            on_change_position,
        } = self;

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(64.px(), |wh, ctx| {
                    ctx.add(typography::body::left(wh.height, "위치", Color::WHITE));
                }),
                table::ratio(1, |wh, ctx| {
                    ctx.add(
                        rect(RectParam {
                            rect: Rect::zero_wh(wh),
                            style: RectStyle {
                                stroke: Some(RectStroke {
                                    color: Color::WHITE,
                                    width: 1.px(),
                                    border_position: BorderPosition::Inside,
                                }),
                                fill: None,
                                round: None,
                            },
                        })
                        .attach_event(|event| {
                            let Event::MouseUp { event } = event else {
                                return;
                            };
                            if !event.is_local_xy_in() {
                                return;
                            }

                            on_change_position(event.local_xy() / wh.as_xy());
                        }),
                    );

                    let radius = 12.px();
                    let circle = namui::Path::new().add_oval(Rect::from_xy_wh(
                        Xy::new(-radius, -radius),
                        Wh::new(radius * 2, radius * 2),
                    ));
                    let paint = Paint::new(Color::WHITE).set_style(PaintStyle::Stroke);
                    let rendering_tree = path(circle, paint);

                    let rows = 5;
                    let cols = 5;

                    for row in 0..rows {
                        for col in 0..cols {
                            let x = wh.width * ((col as f32 + 0.5) / (cols - 1) as f32);
                            let y = wh.height * ((row as f32 + 0.5) / (rows - 1) as f32);

                            ctx.translate((x, y)).add(rendering_tree.clone());
                        }
                    }
                }),
            ])(wh, ctx)
        });
    }
}
