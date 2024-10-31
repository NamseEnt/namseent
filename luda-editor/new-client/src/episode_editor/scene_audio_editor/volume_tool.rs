use crate::*;
use luda_rpc::*;

pub struct VolumeTool<'a> {
    pub wh: Wh<Px>,
    pub selected_audio: &'a Option<SceneSound>,
    pub set_audio: &'a dyn Fn(Option<SceneSound>),
}

impl Component for VolumeTool<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            selected_audio,
            set_audio,
        } = self;

        let set_volume = |volume: Percent| {
            let Some(selected_audio) = selected_audio else {
                return;
            };
            let mut audio = selected_audio.clone();
            audio.volume = volume;
            set_audio(Some(audio));
        };

        ctx.compose(|ctx| {
            table::vertical([
                table::fixed(32.px(), |wh, ctx| {
                    ctx.add(typography::title::left(wh.height, "볼륨", Color::WHITE));
                }),
                table::ratio(1, |wh, ctx| {
                    ctx.add(Slider {
                        wh,
                        value: selected_audio
                            .as_ref()
                            .map_or(0.percent(), |selected_audio| selected_audio.volume),
                        on_change: &set_volume,
                        disabled: selected_audio.is_none(),
                    });
                }),
            ])(wh, ctx)
        });
    }
}

struct Slider<'a> {
    wh: Wh<Px>,
    value: Percent,
    on_change: &'a dyn Fn(Percent),
    disabled: bool,
}
impl Component for Slider<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            value,
            on_change,
            disabled,
        } = self;

        let (dragging, set_dragging) = ctx.state::<Option<Percent>>(|| None);

        let displaying_value = dragging.as_ref().unwrap_or(value);
        let thumb_radius = wh.height * 0.5;
        let body_rect = Rect::Xywh {
            x: thumb_radius,
            y: wh.height * 0.25,
            width: wh.width - (thumb_radius * 2.0),
            height: wh.height * 0.5,
        };
        let thumb_rect = Rect::Xywh {
            x: (body_rect.width() * displaying_value),
            y: 0.px(),
            width: thumb_radius * 2.0,
            height: thumb_radius * 2.0,
        };
        let (thumb_color, active_body_color) = match disabled {
            true => (
                Color::from_u8(0xBB, 0xBB, 0xBB, 0xFF),
                Color::from_u8(0xAA, 0xAA, 0xAA, 0xFF),
            ),
            false => (
                Color::from_u8(0x42, 0xA5, 0xF5, 0xFF),
                Color::from_u8(0x21, 0x96, 0xF3, 0xFF),
            ),
        };

        ctx.add(path(
            Path::new().add_oval(thumb_rect),
            Paint::new(thumb_color),
        ));
        ctx.add(rect(RectParam {
            rect: Rect::Xywh {
                x: body_rect.x(),
                y: body_rect.y(),
                width: body_rect.width() * displaying_value,
                height: body_rect.height(),
            },
            style: RectStyle {
                fill: Some(RectFill {
                    color: active_body_color,
                }),
                stroke: None,
                ..Default::default()
            },
        }));
        ctx.add(rect(RectParam {
            rect: body_rect,
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::from_u8(0x88, 0x88, 0x88, 0xFF),
                }),
                stroke: None,
                ..Default::default()
            },
        }));

        ctx.add(
            rect(RectParam {
                rect: Rect::Xywh {
                    x: body_rect.x(),
                    y: 0.px(),
                    width: body_rect.width(),
                    height: wh.height,
                },
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::TRANSPARENT,
                    }),
                    stroke: None,
                    ..Default::default()
                },
            })
            .attach_event(|event| {
                if disabled {
                    return;
                }

                let update_dragging = |event: &MouseEvent| {
                    set_dragging.set(Some(
                        (((event.local_xy().x - thumb_radius) / body_rect.width()).clamp(0.0, 1.0)
                            * 100.0)
                            .percent(),
                    ));
                };

                match dragging.as_ref() {
                    Some(dragging) => match event {
                        Event::MouseMove { event } => {
                            update_dragging(&event);
                        }
                        Event::MouseUp { .. } | Event::VisibilityChange => {
                            set_dragging.set(None);
                            on_change(*dragging);
                        }
                        _ => (),
                    },
                    None => {
                        let Event::MouseDown { event } = event else {
                            return;
                        };
                        if !event.is_local_xy_in() {
                            return;
                        }
                        event.stop_propagation();
                        update_dragging(&event);
                    }
                }
            }),
        );
    }
}
