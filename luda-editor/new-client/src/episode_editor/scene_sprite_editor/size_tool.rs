use crate::*;

pub struct SizeTool<'a> {
    pub wh: Wh<Px>,
    pub size_radius: Percent,
    pub on_change_size_radius: &'a dyn Fn(Percent),
}

impl Component for SizeTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            size_radius,
            on_change_size_radius,
        } = self;

        let (is_dragging, set_is_dragging) = ctx.state(|| false);

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(64.px(), |wh, ctx| {
                    ctx.add(typography::body::left(
                        wh.height,
                        format!("크기 - {}", size_radius.round()),
                        Color::WHITE,
                    ));
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
                            let mouse_event = match event {
                                Event::MouseDown { event } => {
                                    if !event.is_local_xy_in() {
                                        return;
                                    }
                                    set_is_dragging.set(true);
                                    event
                                }
                                Event::MouseMove { event } => {
                                    if !*is_dragging {
                                        return;
                                    }
                                    event
                                }
                                Event::MouseUp { event } => {
                                    if !*is_dragging {
                                        return;
                                    }
                                    set_is_dragging.set(false);
                                    event
                                }
                                _ => {
                                    return;
                                }
                            };

                            let x = (mouse_event.local_xy().x / wh.width).clamp(0.0, 1.0);
                            on_change_size_radius(100.percent() * x);
                        }),
                    );

                    let cursor_width = 24.px();

                    let x = wh.width * size_radius;
                    ctx.translate((x, 0.px())).add(path(
                        namui::Path::new().add_rect(Rect::from_xy_wh(
                            Xy::new(-cursor_width / 2, 0.px()),
                            Wh::new(cursor_width, wh.height),
                        )),
                        Paint::new(Color::WHITE).set_style(PaintStyle::Stroke),
                    ));
                }),
            ])(wh, ctx)
        });
    }
}
