use crate::app::theme::THEME;
use namui::*;
use namui_prebuilt::simple_rect;

const THUMB_WIDTH: Px = px(16.0);
const MARGIN: Px = px(4.0);

#[component]
pub struct Slider<'a> {
    pub wh: Wh<Px>,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub on_change: &'a dyn Fn(f32),
}

impl Component for Slider<'_> {
    fn render(self, ctx: &RenderCtx) {
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
        let body_height = wh.height - (MARGIN * 2);

        let x = (wh.width - THUMB_WIDTH) * progress;
        ctx.add(rect(RectParam {
            rect: Rect::Xywh {
                x,
                y: 0.px(),
                width: THUMB_WIDTH,
                height: wh.height,
            },
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill { color: THEME.text }),
                round: None,
            },
        }));

        ctx.add(rect(RectParam {
            rect: Rect::Xywh {
                x: 0.px(),
                y: MARGIN,
                width: wh.width * progress,
                height: body_height,
            },
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: THEME.primary.main,
                }),
                round: None,
            },
        }));

        ctx.add(rect(RectParam {
            rect: Rect::Xywh {
                x: 0.px(),
                y: MARGIN,
                width: wh.width,
                height: body_height,
            },
            style: RectStyle {
                stroke: None,
                fill: Some(RectFill {
                    color: THEME.primary.dark,
                }),
                round: None,
            },
        }));

        ctx.add(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), THEME.primary.darker).attach_event(
                |event| match event {
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
                },
            ),
        );
    }
}
