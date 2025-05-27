use crate::*;

pub struct PositionTool<'a> {
    pub wh: Wh<Px>,
    pub position: Xy<Percent>,
    pub on_change_position: &'a dyn Fn(Xy<Percent>),
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
                    const ROWS: isize = 5;
                    const COLS: isize = 5;

                    let to_nearest_point = |xy: Xy<f32>| {
                        let cols = COLS as f32;
                        let rows = ROWS as f32;
                        let x = ((cols * xy.x).floor().min(cols).max(0.0) + 0.5) / cols;
                        let y = ((rows * xy.y).floor().min(rows).max(0.0) + 0.5) / rows;
                        Xy::new(x, y)
                    };

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

                            let new_xy = to_nearest_point(event.local_xy() / wh.to_xy());
                            on_change_position(new_xy.map(|xy| (100.0 * xy).percent()));
                        }),
                    );

                    let radius = 12.px();
                    let circle = namui::Path::new().add_oval(Rect::from_xy_wh(
                        Xy::new(-radius, -radius),
                        Wh::new(radius * 2, radius * 2),
                    ));
                    let default_paint = Paint::new(Color::WHITE).set_style(PaintStyle::Stroke);
                    let active_paint = Paint::new(Color::BLUE).set_style(PaintStyle::Fill);
                    let default_rendering_tree = path(circle.clone(), default_paint);
                    let active_rendering_tree = path(circle, active_paint);

                    for row in 0..ROWS {
                        for col in 0..COLS {
                            let x = wh.width * ((col as f32 + 0.5) / (COLS) as f32);
                            let y = wh.height * ((row as f32 + 0.5) / (ROWS) as f32);

                            ctx.translate((x, y)).add(default_rendering_tree.clone());
                        }
                    }

                    let active_xy = wh.to_xy() * to_nearest_point(position.map(|x| x.as_f32()));
                    ctx.translate(active_xy).add(active_rendering_tree);
                }),
            ])(wh, ctx)
        });
    }
}
