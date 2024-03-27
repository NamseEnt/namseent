use crate::app::{
    note::{Direction, Note},
    play_state::JudgeContext,
    theme::THEME,
};
use keyframe::{
    ease,
    functions::{EaseOutQuart, EaseOutQuint},
};
use namui::{math::num::traits::Pow, prelude::*, time::since_start};
use namui_prebuilt::{simple_rect, typography::adjust_font_size};
use rand::Rng;
use std::collections::VecDeque;

static PARRY_EFFECT_REQUEST: Atom<VecDeque<ParryEffectRequest>> = Atom::uninitialized_new();
static PARRY_EFFECT_PARTICLES: Atom<VecDeque<ParryEffectParticle>> = Atom::uninitialized_new();

const NOTE_WIDTH: Px = px(32.0);
const MARGIN: Px = px(8.0);
const PAD_WIDTH: Px = px(16.0);
const ARROW_OFFSET: Px = px(48.0);
const HEAD_WIDTH: Px = px(4.0);
const LAY_WIDTH: Px = px(512.0);

#[namui::component]
pub struct NotePlotter<'a> {
    pub wh: Wh<Px>,
    pub notes: &'a Vec<Note>,
    pub px_per_time: Per<Px, Duration>,
    pub timing_zero_x: Px,
    pub played_time: Duration,
    pub judge_context: &'a JudgeContext,
}

impl Component for NotePlotter<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let NotePlotter {
            wh,
            notes,
            px_per_time,
            timing_zero_x,
            played_time,
            judge_context,
        } = self;

        let (_particles, set_particles) = ctx.atom_init(&PARRY_EFFECT_PARTICLES, VecDeque::new);
        let (parry_effect_requests, set_parry_effect_requests) =
            ctx.atom_init(&PARRY_EFFECT_REQUEST, VecDeque::new);
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

        ctx.effect("Emit parry effects", move || {
            let parry_effect_requests = parry_effect_requests.as_ref();
            if parry_effect_requests.is_empty() {
                return;
            }
            let new_particles = parry_effect_requests
                .iter()
                .flat_map(|request| {
                    let (baseline_y, _) = lanes[request.note_direction.lane()];
                    request.to_particles(px_per_time, note_wh, baseline_y)
                })
                .collect::<Vec<_>>();

            set_parry_effect_requests.mutate(|parry_effect_requests| parry_effect_requests.clear());
            set_particles.mutate(move |particles| {
                particles.extend(new_particles);
            });
        });

        ctx.component(ParryEffect { timing_zero_x });

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

            for (index, note) in notes.iter().enumerate() {
                let note_x = (px_per_time * (note.start_time - played_time)) + timing_zero_x;
                if note_x < -wh.width {
                    continue;
                }
                if note_x > wh.width * 2 {
                    break;
                }
                if judge_context.judged_note_index.contains(&index) {
                    continue;
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
    fn render(self, ctx: &RenderCtx)  {
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

            ctx.add(Lay { height, color });

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

        
    }
}

#[component]
struct NoteGraphic {
    x: Px,
    height: Px,
    direction: Direction,
}
impl Component for NoteGraphic {
    fn render(self, ctx: &RenderCtx)  {
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

        
    }
}

#[component]
struct NoteHead {
    x: Px,
    height: Px,
    paint: Paint,
}
impl Component for NoteHead {
    fn render(self, ctx: &RenderCtx)  {
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

        
    }
}

#[component]
struct NoteBody {
    x: Px,
    height: Px,
    paint: Paint,
}
impl Component for NoteBody {
    fn render(self, ctx: &RenderCtx)  {
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

        
    }
}

#[component]
struct Pad {
    height: Px,
    paint: Paint,
}
impl Component for Pad {
    fn render(self, ctx: &RenderCtx)  {
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

        
    }
}

#[component]
struct Lane {
    wh: Wh<Px>,
    arrow_offset: Px,
    direction: Direction,
}
impl Component for Lane {
    fn render(self, ctx: &RenderCtx)  {
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

        
    }
}

#[component]
struct Lay {
    height: Px,
    color: Color,
}
impl Component for Lay {
    fn render(self, ctx: &RenderCtx)  {
        let Self { height, color } = self;

        ctx.component(path(
            Path::new().add_rect(Rect::Xywh {
                x: 0.px(),
                y: 0.px(),
                width: LAY_WIDTH,
                height,
            }),
            Paint::new(color)
                .set_blend_mode(BlendMode::Plus)
                .set_shader(Shader::LinearGradient {
                    start_xy: Xy::zero(),
                    end_xy: Xy::new(LAY_WIDTH, 0.px()),
                    colors: vec![color.with_alpha(255), Color::BLACK],
                    tile_mode: TileMode::Mirror,
                }),
        ));

        
    }
}

fn calculate_intensity(duration: Duration) -> Option<u8> {
    let animation_duration = 0.3.sec();
    if duration > animation_duration {
        return None;
    }
    let progress = (duration / animation_duration).clamp(0.0, 1.0);
    if progress >= 1.0 {
        return None;
    }
    let time_function = ease(EaseOutQuart, 1.0, 0.0, progress);
    let alpha = (255.0_f32 * (time_function)) as u8;
    Some(alpha)
}

#[derive(Debug)]
struct ParryEffectRequest {
    time_from_zero_line: Duration,
    note_direction: Direction,
}
impl ParryEffectRequest {
    pub fn to_particles(
        &self,
        px_per_time: Per<Px, Duration>,
        note_wh: Wh<Px>,
        baseline_y: Px,
    ) -> Vec<ParryEffectParticle> {
        let duration_min: Duration = 333.ms();
        let duration_range: Duration = 333.ms();
        let dest_min: Xy<Px> = Xy {
            x: 32.px(),
            y: -(64.px()),
        };
        let dest_range: Xy<Px> = Xy {
            x: 256.px(),
            y: 128.px(),
        };
        let size_min: f32 = 0.5;
        let size_range: f32 = 0.5;
        let rotation_min: Angle = Angle::Degree(-1440.0);
        let rotation_range: Angle = Angle::Degree(2880.0);

        (0..5)
            .map(|_| ParryEffectParticle {
                start_at: since_start(),
                duration: duration_min + duration_range * rand::thread_rng().gen_range(0.0..1.0),
                start_xy: Xy {
                    x: px_per_time * self.time_from_zero_line,
                    y: baseline_y + note_wh.height / 2,
                },
                delta_xy: dest_min + dest_range * rand::thread_rng().gen_range(0.0..1.0),
                start_rotation: rotation_min
                    + rotation_range * rand::thread_rng().gen_range(0.0..1.0),
                delta_rotation: rotation_range * rand::thread_rng().gen_range(0.0..1.0),
                size: size_min + size_range * rand::thread_rng().gen_range(0.0..1.0),
                color: self.note_direction.as_color(),
            })
            .collect()
    }
}

pub fn request_emit_parry_effect(time_from_zero_line: Duration, note_direction: Direction) {
    PARRY_EFFECT_REQUEST.mutate(move |requests| {
        requests.push_back(ParryEffectRequest {
            time_from_zero_line,
            note_direction,
        });
    });
}

#[component]
struct ParryEffect {
    pub timing_zero_x: Px,
}
impl Component for ParryEffect {
    fn render(self, ctx: &RenderCtx)  {
        let Self { timing_zero_x } = self;

        let (particles, set_particles) = ctx.atom(&PARRY_EFFECT_PARTICLES);

        ctx.on_raw_event(|event| {
            let RawEvent::ScreenRedraw { .. } = event else {
                return;
            };
            set_particles.mutate(|particles| {
                let now = since_start();
                while let Some(particle) = particles.front() {
                    if particle.duration < now - particle.start_at {
                        particles.pop_front();
                    } else {
                        break;
                    }
                }
            });
        });

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate((timing_zero_x, 0.px()));

            for particle in particles.iter() {
                let elapsed = since_start() - particle.start_at;
                let progress = time_function(elapsed, particle.duration);
                let xy = particle.start_xy + particle.delta_xy * progress;
                let rotation = particle.start_rotation + particle.delta_rotation * progress;
                let size = particle.size * (1.0 - progress);
                let alpha = (((1.0 - progress).pow(2) * 255.0_f32) as f32).clamp(0.0, 255.0) as u8;
                let color = particle.color.with_alpha(alpha);

                let mut ctx = ctx.translate(xy).rotate(rotation).scale(Xy::single(size));
                for paint in [
                    Paint::new(color).set_blend_mode(BlendMode::Plus),
                    Paint::new(color)
                        .set_blend_mode(BlendMode::Plus)
                        .set_mask_filter(MaskFilter::Blur {
                            blur: Blur::Outer {
                                sigma: Blur::convert_sigma_to_radius(4.0),
                            },
                        }),
                    Paint::new(color)
                        .set_blend_mode(BlendMode::Plus)
                        .set_mask_filter(MaskFilter::Blur {
                            blur: Blur::Outer {
                                sigma: Blur::convert_sigma_to_radius(16.0),
                            },
                        }),
                ] {
                    ctx.add(TextDrawCommand {
                        // https://fontawesome.com/v5/icons/star?f=classic&s=solid
                        text: "".to_string(),
                        font: Font {
                            size: 32.int_px(),
                            name: THEME.icon_font_name.to_string(),
                        },
                        x: 0.px(),
                        y: 0.px(),
                        paint,
                        align: TextAlign::Center,
                        baseline: TextBaseline::Middle,
                        max_width: None,
                        line_height_percent: 100.percent(),
                        underline: None,
                    });
                }
            }
        });

        
    }
}

#[derive(Debug)]
struct ParryEffectParticle {
    start_at: Duration,
    duration: Duration,
    start_xy: Xy<Px>,
    delta_xy: Xy<Px>,
    start_rotation: Angle,
    delta_rotation: Angle,
    size: f32,
    color: Color,
}

fn time_function(elapsed: Duration, duration: Duration) -> f32 {
    let progress = elapsed / duration;
    ease(EaseOutQuint, 0.0, 1.0, progress)
}
