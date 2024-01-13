use namui::prelude::*;
use namui_prebuilt::simple_rect;

const THUMB_WIDTH: Px = px(32.0);

#[component]
pub struct Slider<'a> {
    pub wh: Wh<Px>,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub on_change: &'a dyn Fn(f32),
}

impl Component for Slider<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            value,
            min,
            max,
            on_change,
        } = self;

        let progress = {
            let value = f32::clamp(value, min, max);
            (value - min) / (max - min)
        };

        let x = (wh.width - THUMB_WIDTH) * progress;
        ctx.component(rect(RectParam {
            rect: Rect::Xywh {
                x,
                y: 0.px(),
                width: THUMB_WIDTH,
                height: wh.height,
            },
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: Color::WHITE,
                }),
                round: Some(RectRound {
                    radius: wh.height / 3,
                }),
            },
        }));

        ctx.component(rect(RectParam {
            rect: Rect::Xywh {
                x: 0.px(),
                y: wh.height / 4,
                width: wh.width * progress,
                height: wh.height / 2,
            },
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: Color::WHITE,
                }),
                round: Some(RectRound {
                    radius: wh.height / 3,
                }),
            },
        }));

        ctx.component(rect(RectParam {
            rect: Rect::Xywh {
                x: 0.px(),
                y: wh.height / 4,
                width: wh.width,
                height: wh.height / 2,
            },
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: Color::WHITE,
                }),
                round: Some(RectRound {
                    radius: wh.height / 3,
                }),
            },
        }));

        ctx.component(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(|event| {
                match event {
                    Event::MouseDown { event }
                    | Event::MouseMove { event }
                    | Event::MouseUp { event } => {
                        if !event.is_local_xy_in() {
                            return;
                        }
                        if !event.pressing_buttons.contains(&MouseButton::Left) {
                            return;
                        }

                        let progress = {
                            let min_x = THUMB_WIDTH / 2;
                            let max_x = wh.width - (THUMB_WIDTH / 2);
                            let mouse_x = event.local_xy().x.clamp(min_x, max_x);
                            (mouse_x - min_x) / (max_x - min_x)
                        };
                        on_change(max * progress);
                    }
                    _ => {}
                }
            }),
        );

        ctx.done()
    }
}
