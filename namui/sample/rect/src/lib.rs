use namui::*;

pub fn main() {
    namui::start(|ctx| {
        ctx.add(RectExample);
    })
}

#[namui::component]
struct RectExample;

impl Component for RectExample {
    fn render(self, ctx: &RenderCtx) {
        let (delta_xy, set_delta_xy) = ctx.state(Xy::<f32>::zero);

        let screen_wh = namui::screen::size();

        for x in 0..3 {
            for y in 0..11 {
                let border_position = [
                    BorderPosition::Inside,
                    BorderPosition::Middle,
                    BorderPosition::Outside,
                ][x];

                let xy_additional = match y {
                    0 => Xy::new(0.0.px(), 0.0.px()),
                    1 => Xy::new(0.5.px(), 0.0.px()),
                    2 => Xy::new(0.0.px(), 0.5.px()),
                    3 => Xy::new(0.5.px(), 0.5.px()),
                    4 => {
                        let time = namui::time::since_start();
                        let x = (time.as_secs_f32() * 20.0) % 40.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    5 => {
                        let time = namui::time::since_start();
                        let x = (time.as_secs_f32() * 10.0).round() % 40.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    6 => {
                        let time = namui::time::since_start();
                        let x = (time.as_secs_f32() * 160.0) % 160.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    7 => {
                        let time = namui::time::since_start();
                        let x = (time.as_secs_f32() * 80.0) % 160.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    8 => {
                        let time = namui::time::since_start();
                        let x = (time.as_secs_f32() * 80.0).round() % 160.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    9 => {
                        let y = delta_xy.y.floor() + 0.5;
                        Xy::new(0.5.px(), y.px())
                    }
                    10 => {
                        let y = delta_xy.y.round() + 0.5;
                        Xy::new(0.5.px(), y.px() + 100.px())
                    }
                    _ => unreachable!(),
                };

                let rect = rect(RectParam {
                    rect: Rect::Xywh {
                        x: (x as i32 * 20).px() + xy_additional.x + 300.px(),
                        y: (y * 20).px() + xy_additional.y + 300.px(),
                        width: 10.px(),
                        height: 10.px(),
                    },
                    style: RectStyle {
                        stroke: Some(RectStroke {
                            color: Color::BLACK,
                            width: 1.px(),
                            border_position,
                        }),
                        fill: Some(RectFill {
                            color: Color::from_f01(1.0, 1.0, 0.0, 1.0),
                        }),
                        round: None,
                    },
                });
                ctx.add(rect);
            }
        }

        ctx.add(rect(RectParam {
            rect: Rect::Xywh {
                x: 0.px(),
                y: 0.px(),
                width: 5.px(),
                height: 5.px(),
            },
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::BLACK,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: None,
                round: None,
            },
        }));

        ctx.add(
            rect(RectParam {
                rect: Rect::Xywh {
                    x: 0.px(),
                    y: 0.px(),
                    width: screen_wh.width.into(),
                    height: screen_wh.height.into(),
                },
                style: RectStyle {
                    stroke: Some(RectStroke {
                        color: Color::BLACK,
                        width: 1.px(),
                        border_position: BorderPosition::Inside,
                    }),
                    fill: None,
                    round: None,
                },
            })
            .attach_event(|event| {
                if let Event::Wheel { event } = event {
                    set_delta_xy.mutate(move |delta_xy| {
                        delta_xy.x += event.delta_xy.x;
                        delta_xy.y += event.delta_xy.y;
                    });
                }
            }),
        );
    }
}
