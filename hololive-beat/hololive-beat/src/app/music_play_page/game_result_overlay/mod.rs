use crate::app::{
    components::{Backdrop, FilledButton},
    play_state::{restart_game, JudgeContext, PlayState, Rank, PLAY_STATE_ATOM},
    theme::THEME,
    MUSIC_BEST_SCORE_MAP_ATOM,
};
use namui::prelude::*;
use namui_prebuilt::typography::{self, adjust_font_size, effect::glow};

const PADDING: Px = px(32.0);

#[component]
pub struct GameResultOverlay<'a> {
    pub wh: Wh<Px>,
    pub judge_context: &'a JudgeContext,
    pub music_id: &'a str,
}
impl Component for GameResultOverlay<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self {
            wh,
            judge_context,
            music_id,
        } = self;

        let (best_score_map, _) = ctx.atom(&MUSIC_BEST_SCORE_MAP_ATOM);
        let best_score = (*best_score_map)
            .as_ref()
            .map(|best_score_map| best_score_map.get(music_id))
            .unwrap_or(0);

        let frame_height = wh.height * 0.5;
        let frame_inner_wh = Wh {
            width: wh.width - (PADDING * 2),
            height: frame_height - (PADDING * 2),
        };
        let center_wh = Wh {
            width: frame_height,
            height: frame_inner_wh.height,
        };
        let side_wh = Wh {
            width: (frame_inner_wh.width - center_wh.width) / 2,
            height: frame_inner_wh.height,
        };
        let small_score_wh = Wh {
            width: side_wh.width,
            height: frame_inner_wh.height / 6,
        };
        let large_score_wh = Wh {
            width: side_wh.width,
            height: frame_inner_wh.height * 0.35,
        };

        ctx.component(text(TextParam {
            text: "Result".to_string(),
            x: wh.width / 2,
            y: (wh.height / 2) - (frame_height / 2) - 64.px(),
            align: TextAlign::Center,
            baseline: TextBaseline::Bottom,
            font: Font {
                size: 96.int_px(),
                name: THEME.font_name.to_string(),
            },
            style: TextStyle {
                color: THEME.text,
                ..Default::default()
            },
            max_width: None,
        }));

        ctx.compose(|ctx| {
            let mut ctx = ctx.translate((PADDING, (wh.height / 2) - (frame_inner_wh.height / 2)));

            ctx.add(SmallScore {
                wh: small_score_wh,
                label: "Perfect".to_string(),
                score: judge_context.perfect_count,
            })
            .translate((0.px(), small_score_wh.height))
            .add(SmallScore {
                wh: small_score_wh,
                label: "Good".to_string(),
                score: judge_context.good_count,
            })
            .translate((0.px(), small_score_wh.height))
            .add(SmallScore {
                wh: small_score_wh,
                label: "Miss".to_string(),
                score: judge_context.miss_count,
            })
            .translate((0.px(), small_score_wh.height))
            .add(SmallScore {
                wh: small_score_wh,
                label: "MaxCombo".to_string(),
                score: judge_context.max_combo,
            });

            let mut ctx = ctx.translate((side_wh.width, 0.px()));
            ctx.add(RankText {
                wh: center_wh,
                rank: judge_context.rank,
            });

            let mut ctx = ctx.translate((center_wh.width, 0.px()));
            ctx.add(LargeScore {
                wh: large_score_wh,
                label: "Best score".to_string(),
                score: best_score,
            })
            .translate((0.px(), side_wh.height - large_score_wh.height))
            .add(LargeScore {
                wh: large_score_wh,
                label: "Score".to_string(),
                score: judge_context.score,
            })
            .add(NewRecord {
                wh: large_score_wh,
                show: best_score == judge_context.score,
            });
        });

        ctx.compose(|ctx| {
            let button_wh = Wh::new(324.px(), 96.px());
            let mut ctx = ctx.translate((
                (wh.width / 2),
                (wh.height / 2) + (frame_height / 2) + 64.px(),
            ));

            ctx.translate((-(256.px() + button_wh.width), 0.px()))
                .add(FilledButton {
                    wh: button_wh,
                    text: "Music".to_string(),
                    on_click: &|| {
                        PLAY_STATE_ATOM.set(PlayState::Idle);
                    },
                    focused: false,
                });

            ctx.translate((256.px(), 0.px())).add(FilledButton {
                wh: button_wh,
                text: "Retry".to_string(),
                on_click: &|| {
                    restart_game();
                },
                focused: false,
            });
        });

        ctx.compose(|ctx| {
            ctx.translate((0.px(), (wh.height - frame_height) / 2))
                .add(Frame {
                    wh: Wh::new(wh.width, frame_height),
                });
        });

        ctx.component(Backdrop { wh });

        ctx.done()
    }
}

#[component]
struct Frame {
    wh: Wh<Px>,
}
impl Component for Frame {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;
        let path = Path::new().add_rect(Rect::zero_wh(wh));

        ctx.component(namui::path(
            path.clone(),
            Paint::new(THEME.primary.dark).set_blend_mode(BlendMode::Multiply),
        ));

        ctx.component(namui::path(
            path.clone(),
            Paint::new(THEME.primary.main.with_alpha(38))
                .set_blend_mode(BlendMode::Screen)
                .set_mask_filter(MaskFilter::Blur {
                    blur: Blur::Outer {
                        sigma: Blur::convert_radius_to_sigma(64.0),
                    },
                }),
        ));

        ctx.done()
    }
}

#[component]
struct SmallScore {
    wh: Wh<Px>,
    label: String,
    score: usize,
}
impl Component for SmallScore {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, label, score } = self;

        let middle_y = wh.height / 2;

        ctx.component(glow(
            label,
            Font {
                size: typography::adjust_font_size(wh.height),
                name: THEME.font_name.to_string(),
            },
            Xy::new(0.px(), middle_y),
            Paint::new(THEME.text),
            TextAlign::Left,
            TextBaseline::Middle,
            Blur::Outer {
                sigma: Blur::convert_radius_to_sigma(wh.height.as_f32() * 0.2),
            },
            0.px(),
            THEME.primary.main,
        ));

        ctx.component(text(TextParam {
            text: score.to_string(),
            x: wh.width,
            y: middle_y,
            align: TextAlign::Right,
            baseline: TextBaseline::Middle,
            font: Font {
                size: typography::adjust_font_size(wh.height),
                name: THEME.font_name.to_string(),
            },
            style: TextStyle {
                color: THEME.text.with_alpha(191),
                ..Default::default()
            },
            max_width: None,
        }));

        ctx.done()
    }
}

#[component]
struct RankText {
    wh: Wh<Px>,
    rank: Rank,
}
impl Component for RankText {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, rank } = self;

        ctx.component(TextDrawCommand {
            text: rank.to_string(),
            font: Font {
                size: wh.height.into(),
                name: THEME.font_name.to_string(),
            },
            x: wh.width / 2,
            y: wh.height / 2,
            paint: Paint::new(THEME.primary.main).set_shader(Shader::LinearGradient {
                start_xy: Xy::new(0.px(), 0.px()),
                end_xy: Xy::new(0.px(), wh.height / 2),
                colors: vec![THEME.primary.main.brighter(0.25), THEME.primary.main],
                tile_mode: TileMode::Clamp,
            }),
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            max_width: None,
            line_height_percent: 100.percent(),
            underline: None,
        });

        ctx.done()
    }
}

#[component]
struct LargeScore {
    wh: Wh<Px>,
    label: String,
    score: usize,
}
impl Component for LargeScore {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, label, score } = self;

        let label_height = wh.height / 2;
        let score_height = label_height * 0.9;

        ctx.component(glow(
            label,
            Font {
                size: typography::adjust_font_size(label_height),
                name: THEME.font_name.to_string(),
            },
            Xy::new(0.px(), label_height / 2),
            Paint::new(THEME.text),
            TextAlign::Left,
            TextBaseline::Middle,
            Blur::Outer {
                sigma: Blur::convert_radius_to_sigma(label_height.as_f32() * 0.2),
            },
            0.px(),
            THEME.primary.main,
        ));

        ctx.component(namui::text(TextParam {
            text: score.to_string(),
            x: wh.width,
            y: label_height,
            align: TextAlign::Right,
            baseline: TextBaseline::Top,
            font: Font {
                size: adjust_font_size(score_height),
                name: THEME.font_name.to_string(),
            },
            style: TextStyle {
                color: THEME.text.with_alpha(191),
                ..Default::default()
            },
            max_width: None,
        }));

        ctx.done()
    }
}

#[component]
struct NewRecord {
    wh: Wh<Px>,
    show: bool,
}
impl Component for NewRecord {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, show } = self;

        let height = wh.height / 2;

        ctx.compose(|ctx| {
            if !show {
                return;
            }
            let text = ctx.ghost_add(
                None,
                glow(
                    "New\nRecord",
                    Font {
                        size: typography::adjust_font_size(height / 2),
                        name: THEME.font_name.to_string(),
                    },
                    Xy::new(wh.width, height / 2),
                    Paint::new(THEME.text),
                    TextAlign::Center,
                    TextBaseline::Middle,
                    Blur::Outer {
                        sigma: Blur::convert_radius_to_sigma(height.as_f32() * 0.2),
                    },
                    0.px(),
                    THEME.primary.main,
                ),
                GhostComposeOption {
                    enable_event_handling: false,
                },
            );
            let width = text
                .bounding_box()
                .map(|bounding_box| bounding_box.width())
                .unwrap();
            ctx.translate((-(width / 2), 0.px())).add(text);
        });

        ctx.done()
    }
}
