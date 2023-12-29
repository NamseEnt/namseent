use crate::app::color::THEME;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const THUMB_WIDTH: Px = px(32.0);
const THUMB_STROKE_WIDTH: Px = px(2.0);

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

        ctx.compose(|ctx| {
            let x = (wh.width - THUMB_WIDTH) * progress;
            ctx.translate((x, 0.px())).add(simple_rect(
                Wh {
                    width: THUMB_WIDTH,
                    height: wh.height,
                },
                THEME.primary.contrast_text,
                THUMB_STROKE_WIDTH,
                THEME.primary.main,
            ));
        });

        ctx.component(simple_rect(
            Wh {
                width: wh.width * progress,
                height: wh.height,
            },
            Color::TRANSPARENT,
            0.px(),
            THEME.primary.main,
        ));

        ctx.component(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), THEME.surface.main).attach_event(|event| {
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
