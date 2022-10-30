use namui::prelude::*;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut RectExample::new(), &()).await
}

struct RectExample {
    delta_xy: Xy<f32>,
}

enum Event {
    DeltaXy(Xy<f32>),
}

impl RectExample {
    fn new() -> Self {
        Self {
            delta_xy: Xy::zero(),
        }
    }
}

impl Entity for RectExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let screen_wh = namui::screen::size();
        let mut rendering_tree = vec![];
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
                        let time = namui::time::now();
                        let x = (time.as_seconds() * 20.0) % 40.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    5 => {
                        let time = namui::time::now();
                        let x = (time.as_seconds() * 10.0).round() % 40.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    6 => {
                        let time = namui::time::now();
                        let x = (time.as_seconds() * 160.0) % 160.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    7 => {
                        let time = namui::time::now();
                        let x = (time.as_seconds() * 80.0) % 160.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    8 => {
                        let time = namui::time::now();
                        let x = (time.as_seconds() * 80.0).round() % 160.0;
                        Xy::new(x.px(), 0.0.px())
                    }
                    9 => {
                        let y = self.delta_xy.y.floor() + 0.5;
                        Xy::new(0.5.px(), y.px())
                    }
                    10 => {
                        let y = self.delta_xy.y.round() + 0.5;
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
                        fill: None,
                        round: None,
                    },
                });

                rendering_tree.push(rect);
            }
        }

        rendering_tree.push(
            rect(RectParam {
                rect: Rect::Xywh {
                    x: 0.px(),
                    y: 0.px(),
                    width: screen_wh.width,
                    height: screen_wh.height,
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
            .attach_event(|builder| {
                builder.on_wheel(|event| namui::event::send(Event::DeltaXy(event.delta_xy)));
            }),
        );

        render(rendering_tree)
    }

    fn update(&mut self, _event: &dyn std::any::Any) {
        if let Some(Event::DeltaXy(delta_xy)) = _event.downcast_ref::<Event>() {
            self.delta_xy.x += delta_xy.x;
            self.delta_xy.y += delta_xy.y;
        }
    }
}
