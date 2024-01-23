use crate::app::{
    note::{Direction, Note},
    theme::THEME,
};
use namui::{math::num::traits::Pow, prelude::*, time::since_start};
use namui_prebuilt::{simple_rect, typography::adjust_font_size};

const NOTE_WIDTH: Px = px(32.0);
const MARGIN: Px = px(8.0);
const PAD_WIDTH: Px = px(16.0);
const ARROW_OFFSET: Px = px(48.0);
const HEAD_WIDTH: Px = px(4.0);

#[namui::component]
pub struct NotePlotter<'a> {
    pub wh: Wh<Px>,
    pub notes: &'a Vec<Note>,
    pub px_per_time: Per<Px, Duration>,
    pub timing_zero_x: Px,
    pub played_time: Duration,
}

impl Component for NotePlotter<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let NotePlotter {
            wh,
            notes,
            px_per_time,
            timing_zero_x,
            played_time,
        } = self;

        let (pressed_at, set_pressed_at) = ctx.state(|| [Duration::default(); 4]);

        let note_wh = Wh {
            width: NOTE_WIDTH,
            height: (wh.height - (MARGIN * 3)) / 4,
        };
        let lanes = {
            let step = note_wh.height + MARGIN;
            [
                (0.px(), Direction::Up),
                (step, Direction::Right),
                (step * 2, Direction::Left),
                (step * 3, Direction::Down),
            ]
        };

        ctx.compose(|ctx| {
            for (y, direction) in lanes {
                let mut ctx = ctx.translate((0.px(), y));

                ctx.add(Flash {
                    timing_zero_x,
                    height: note_wh.height,
                    color: direction.as_color(),
                    flashed_at: pressed_at[direction.lane()],
                });

                ctx.add(NoteHead {
                    x: timing_zero_x,
                    height: note_wh.height,
                    paint: Paint::new(THEME.text.with_alpha(178)),
                });

                ctx.add(Pad {
                    height: note_wh.height,
                    paint: Paint::new(direction.as_color()),
                });
            }
        });

        ctx.compose(|ctx| {
            let mut ctx = ctx.clip(
                Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), wh)),
                ClipOp::Intersect,
            );

            for note in notes {
                let note_x = (px_per_time * (note.start_time - played_time)) + timing_zero_x;
                if note_x < -wh.width {
                    continue;
                }
                if note_x > wh.width * 2 {
                    break;
                }
                let (note_y, _) = lanes[note.direction.lane()];
                let key = format!("{:?}-{:?}", note.instrument, note.start_time);
                ctx.translate((0.px(), note_y)).add_with_key(
                    key,
                    NoteGraphic {
                        x: note_x,
                        height: note_wh.height,
                        direction: note.direction,
                    },
                );
            }
        });

        ctx.compose(|ctx| {
            for (y, direction) in lanes {
                let mut ctx = ctx.translate((0.px(), y));

                ctx.add(NoteBody {
                    x: timing_zero_x,
                    height: note_wh.height,
                    paint: Paint::new(THEME.text.with_alpha(76)),
                });

                ctx.add(Lane {
                    wh: Wh::new(wh.width, note_wh.height),
                    arrow_offset: ARROW_OFFSET,
                    direction,
                });
            }
        });

        ctx.on_raw_event(|event| {
            let RawEvent::KeyDown { event } = event else {
                return;
            };
            let Ok(direction) = Direction::try_from(event.code) else {
                return;
            };
            set_pressed_at.mutate(move |pressed_at| pressed_at[direction.lane()] = since_start())
        });

        ctx.done()
    }
}

#[component]
struct Flash {
    timing_zero_x: Px,
    height: Px,
    color: Color,
    flashed_at: Duration,
}
impl Component for Flash {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            timing_zero_x,
            height,
            color,
            flashed_at,
        } = self;

        ctx.compose(|ctx| {
            let intensity = {
                let elapsed = since_start() - flashed_at;
                calculate_intensity(elapsed)
            };
            let Some(intensity) = intensity else {
                return;
            };
            let color = color.brighter(0.2).with_alpha(intensity);

            ctx.add(NoteHead {
                x: timing_zero_x,
                height,
                paint: Paint::new(color)
                    .set_blend_mode(BlendMode::Plus)
                    .set_mask_filter(MaskFilter::Blur {
                        blur: Blur::Outer {
                            sigma: Blur::convert_sigma_to_radius(16.0),
                        },
                    }),
            });
            ctx.add(NoteHead {
                x: timing_zero_x,
                height,
                paint: Paint::new(color)
                    .set_blend_mode(BlendMode::Plus)
                    .set_mask_filter(MaskFilter::Blur {
                        blur: Blur::Outer {
                            sigma: Blur::convert_sigma_to_radius(4.0),
                        },
                    }),
            });
            ctx.add(NoteHead {
                x: timing_zero_x,
                height,
                paint: Paint::new(color).set_blend_mode(BlendMode::Plus),
            });

            // TODO lay

            ctx.add(Pad {
                height,
                paint: Paint::new(color)
                    .set_blend_mode(BlendMode::Plus)
                    .set_mask_filter(MaskFilter::Blur {
                        blur: Blur::Outer {
                            sigma: Blur::convert_sigma_to_radius(16.0),
                        },
                    }),
            });
            ctx.add(Pad {
                height,
                paint: Paint::new(color)
                    .set_blend_mode(BlendMode::Plus)
                    .set_mask_filter(MaskFilter::Blur {
                        blur: Blur::Outer {
                            sigma: Blur::convert_sigma_to_radius(4.0),
                        },
                    }),
            });
            ctx.add(Pad {
                height,
                paint: Paint::new(color).set_blend_mode(BlendMode::Plus),
            });
        });

        ctx.done()
    }
}

#[component]
struct NoteGraphic {
    x: Px,
    height: Px,
    direction: Direction,
}
impl Component for NoteGraphic {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            x,
            height,
            direction,
        } = self;

        ctx.component(NoteHead {
            x,
            height,
            paint: Paint::new(THEME.text),
        });
        ctx.component(NoteBody {
            x,
            height,
            paint: Paint::new(direction.as_color()),
        });

        ctx.done()
    }
}

#[component]
struct NoteHead {
    x: Px,
    height: Px,
    paint: Paint,
}
impl Component for NoteHead {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { x, height, paint } = self;

        ctx.component(path(
            Path::new().add_rect(Rect::Xywh {
                x,
                y: 0.px(),
                width: HEAD_WIDTH,
                height,
            }),
            paint,
        ));

        ctx.done()
    }
}

#[component]
struct NoteBody {
    x: Px,
    height: Px,
    paint: Paint,
}
impl Component for NoteBody {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { x, height, paint } = self;

        ctx.component(path(
            Path::new().add_rect(Rect::Xywh {
                x,
                y: 0.px(),
                width: NOTE_WIDTH,
                height,
            }),
            paint,
        ));

        ctx.done()
    }
}

#[component]
struct Pad {
    height: Px,
    paint: Paint,
}
impl Component for Pad {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { height, paint } = self;

        ctx.component(path(
            Path::new().add_rect(Rect::Xywh {
                x: 0.px(),
                y: 0.px(),
                width: PAD_WIDTH,
                height,
            }),
            paint,
        ));

        ctx.done()
    }
}

#[component]
struct Lane {
    wh: Wh<Px>,
    arrow_offset: Px,
    direction: Direction,
}
impl Component for Lane {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            arrow_offset,
            direction,
        } = self;

        let text = match direction {
            // https://fontawesome.com/v5/icons/arrow-left?f=classic&s=solid
            Direction::Left => "",
            // https://fontawesome.com/v5/icons/arrow-up?f=classic&s=solid
            Direction::Up => "",
            // https://fontawesome.com/v5/icons/arrow-down?f=classic&s=solid
            Direction::Down => "",
            // https://fontawesome.com/v5/icons/arrow-right?f=classic&s=solid
            Direction::Right => "",
        }
        .to_string();

        ctx.component(namui::text(TextParam {
            text,
            x: arrow_offset,
            y: wh.height / 2,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font: Font {
                size: adjust_font_size(wh.height),
                name: THEME.icon_font_name.to_string(),
            },
            style: TextStyle {
                color: THEME.text.with_alpha(216),
                ..Default::default()
            },
            max_width: None,
        }));

        ctx.component(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK.with_alpha(178),
        ));

        ctx.done()
    }
}

fn calculate_intensity(duration: Duration) -> Option<u8> {
    const T1: f32 = 0.9;
    const T2: f32 = 0.95;

    let animation_duration = 0.3.sec();
    if duration > animation_duration {
        return None;
    }
    let progress = (duration / animation_duration).clamp(0.0, 1.0);
    if progress >= 1.0 {
        return None;
    }
    let reverse_progress = 1.0 - progress;
    let time_function = T1 * (3.0_f32 * reverse_progress.pow(2) * progress)
        + T2 * (3.0 * reverse_progress * progress.pow(2))
        + progress.pow(3);
    let alpha = (255.0_f32 * (1.0_f32 - time_function)) as u8;
    Some(alpha)
}
