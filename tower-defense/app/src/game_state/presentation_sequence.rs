use crate::animation::CubicBezier;
use crate::game_state::monster::{MonsterKind, monster_wh};
use crate::game_state::presentation_director::DEFENSE_INTRO_DURATION_SECS;
use crate::theme::{
    palette,
    typography::{FontSize, memoized_text},
};
use crate::time::PresentationInstant;
use namui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StrongestEnemySelection {
    pub(crate) monster_kind: MonsterKind,
    pub(crate) used_fallback: bool,
}

const INTRO_BEZIER: CubicBezier = CubicBezier::new(0.05, 0.95, 0.95, 0.05);
const LETTERBOX_ROTATION_DEG: f32 = 2.5;
const LETTERBOX_HEIGHT_RATIO: f32 = 0.2;
const LETTERBOX_MAX_OPACITY: f32 = 0.8;
const LETTERBOX_ENTRY_END: f32 = 0.10;
const LETTERBOX_EXIT_START: f32 = 0.90;
const LETTERBOX_IN_BEZIER: CubicBezier = CubicBezier::new(0.0, 0.0, 0.0, 1.0);
const LETTERBOX_OUT_BEZIER: CubicBezier = CubicBezier::new(1.0, 0.0, 1.0, 1.0);
const INTRO_INFO_MARGIN: Px = px(12.0);
const INTRO_INFO_FONT_SIZE_RATIO: f32 = 0.1;
const INTRO_INFO_STROKE_SIZE_RATIO: f32 = 0.05;
const INTRO_INFO_TOP_MARGIN_RATIO: f32 = 0.04;
const MARQUEE_SPEED_PX_PER_SEC: f32 = 220.0;

pub(crate) fn intro_progress(elapsed_secs: f32) -> f32 {
    (elapsed_secs / DEFENSE_INTRO_DURATION_SECS).clamp(0.0, 1.0)
}

pub(crate) fn intro_opacity(progress: f32) -> f32 {
    let eased_progress = INTRO_BEZIER.sample(progress);
    1.0 - (eased_progress * 2.0 - 1.0).abs()
}

pub(crate) fn intro_horizontal_progress(progress: f32) -> f32 {
    INTRO_BEZIER.sample(progress)
}

fn intro_info_text_offset(progress: f32, screen_width: Px) -> Px {
    let eased_progress = INTRO_BEZIER.sample(progress);
    (screen_width * 0.5) - INTRO_INFO_MARGIN - screen_width * eased_progress
}

fn letterbox_presence(progress: f32) -> f32 {
    let progress = progress.clamp(0.0, 1.0);
    if progress < LETTERBOX_ENTRY_END {
        LETTERBOX_IN_BEZIER.sample(progress / LETTERBOX_ENTRY_END)
    } else if progress < LETTERBOX_EXIT_START {
        1.0
    } else {
        1.0 - LETTERBOX_OUT_BEZIER
            .sample((progress - LETTERBOX_EXIT_START) / (1.0 - LETTERBOX_EXIT_START))
    }
}

fn letterbox_opacity(progress: f32) -> f32 {
    letterbox_presence(progress) * LETTERBOX_MAX_OPACITY
}

fn letterbox_center(screen_wh: Wh<Px>, bar_y: Px, bar_height: Px, top_bar: bool) -> Xy<Px> {
    let bar_width = screen_wh.width + bar_height * 2.0;
    let rotation_offset = px(bar_width.as_f32() * LETTERBOX_ROTATION_DEG.to_radians().sin() * 0.5);
    let center_y = bar_y
        + bar_height * 0.5
        + if top_bar {
            -rotation_offset
        } else {
            rotation_offset
        };
    Xy::new(screen_wh.width * 0.5, center_y)
}

pub(crate) fn strongest_enemy_for_stage(
    config: &td_core::GameConfigState,
    stage: usize,
) -> StrongestEnemySelection {
    let mut best: Option<(MonsterKind, i64)> = None;
    if let Some(wave) = config
        .monsters
        .stage_waves
        .iter()
        .find(|wave| wave.stage == stage)
    {
        for entry in wave.entries.iter().filter(|entry| entry.count > 0) {
            let Some(monster_kind) = MonsterKind::from_core_raw(entry.kind) else {
                continue;
            };
            let Some(stats) = config
                .monsters
                .stats
                .iter()
                .find(|stats| stats.kind == entry.kind)
            else {
                continue;
            };

            let candidate = (monster_kind, stats.base_hp_raw);
            let is_boss = !monster_kind.is_normal_monster();
            let should_replace = best.is_none_or(|(current_kind, current_hp)| {
                let current_is_boss = !current_kind.is_normal_monster();
                (
                    is_boss,
                    candidate.1,
                    std::cmp::Reverse(monster_kind.to_core_raw()),
                ) > (
                    current_is_boss,
                    current_hp,
                    std::cmp::Reverse(current_kind.to_core_raw()),
                )
            });
            if should_replace {
                best = Some(candidate);
            }
        }
    }

    if let Some((monster_kind, _)) = best {
        StrongestEnemySelection {
            monster_kind,
            used_fallback: false,
        }
    } else {
        StrongestEnemySelection {
            monster_kind: MonsterKind::Mob01,
            used_fallback: true,
        }
    }
}

pub(crate) struct PresentationSequenceLayer {
    pub(crate) presentation_instant: PresentationInstant,
}

impl Component for PresentationSequenceLayer {
    fn render(self, ctx: &RenderCtx) {
        let game_state = crate::game_state::use_game_state(ctx);
        let Some(intro) = game_state.presentation_director.active_defense_intro() else {
            return;
        };

        let screen_wh = screen::size().map(IntPx::into_px);
        let elapsed = self
            .presentation_instant
            .delta_since(intro.started_at)
            .as_secs_f32();
        let progress = intro_progress(elapsed);
        let opacity = intro_opacity(progress);
        render_intro_info(ctx, screen_wh, opacity, progress);

        let native_wh = monster_wh(intro.monster_kind);
        let height_factor = if !intro.boss { 0.62 } else { 0.78 };
        let image_height = screen_wh.height * height_factor;
        let image_width = image_height * native_wh.width.as_f32() / native_wh.height.as_f32();
        let x_progress = intro_horizontal_progress(progress);
        let x = -image_width * 0.5 + (screen_wh.width * 0.3 + image_width) * x_progress;
        let y = screen_wh.height - image_height / 6.0;
        let image_wh = Wh::new(image_width, image_height);
        let image_opacity = (opacity * 255.0).round() as u8;

        ctx.translate(Xy::new(x, y)).add(namui::image(ImageParam {
            rect: Rect::from_xy_wh(image_wh.to_xy() * -0.5, image_wh),
            image: intro.monster_kind.image(),
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: Some(Paint::new(Color::WHITE.with_alpha(image_opacity))),
            },
        }));

        render_letterbox(ctx, screen_wh, progress, elapsed, intro.boss);
    }
}

fn render_intro_info(ctx: &RenderCtx, screen_wh: Wh<Px>, opacity: f32, progress: f32) {
    let text_color = palette::WARM_WHITE.with_alpha((opacity * 255.0).round() as u8);
    let text_offset = intro_info_text_offset(progress, screen_wh.width);
    let info_font_size = screen_wh.height * INTRO_INFO_FONT_SIZE_RATIO;
    let info_stroke_size = info_font_size * INTRO_INFO_STROKE_SIZE_RATIO;
    let info_top_margin = screen_wh.height * INTRO_INFO_TOP_MARGIN_RATIO;

    ctx.translate(Xy::new(text_offset, info_top_margin))
        .add(memoized_text((), move |mut builder| {
            builder
                .headline()
                .bold()
                .size(FontSize::Custom {
                    size: info_font_size,
                })
                .color(text_color)
                .stroke(info_stroke_size, palette::DARK_CHARCOAL)
                .static_text("PLACEHOLDER_FOR_STAGE_NAME")
                .render_right_top(screen_wh.width)
        }));
}

fn render_letterbox(
    ctx: &RenderCtx,
    screen_wh: Wh<Px>,
    progress: f32,
    elapsed_secs: f32,
    boss: bool,
) {
    let bar_height = screen_wh.height * LETTERBOX_HEIGHT_RATIO;
    let visibility = letterbox_presence(progress);
    let opacity = letterbox_opacity(progress);
    let alpha = (opacity * 255.0).round() as u8;
    let top_y = -bar_height + bar_height * visibility;
    let bottom_y = screen_wh.height - bar_height * visibility;

    render_letterbox_marquee(
        ctx,
        screen_wh,
        top_y,
        bar_height,
        LetterboxMarqueeParams {
            elapsed_secs,
            boss,
            opacity,
            top_bar: true,
        },
    );
    render_solid_letterbox_bar(ctx, screen_wh, top_y, bar_height, true, alpha);
    render_letterbox_marquee(
        ctx,
        screen_wh,
        bottom_y,
        bar_height,
        LetterboxMarqueeParams {
            elapsed_secs,
            boss,
            opacity,
            top_bar: false,
        },
    );
    render_solid_letterbox_bar(ctx, screen_wh, bottom_y, bar_height, false, alpha);
}

fn render_solid_letterbox_bar(
    ctx: &RenderCtx,
    screen_wh: Wh<Px>,
    bar_y: Px,
    bar_height: Px,
    top_bar: bool,
    alpha: u8,
) {
    let bar_width = screen_wh.width + bar_height * 2.0;
    let center = letterbox_center(screen_wh, bar_y, bar_height, top_bar);
    ctx.compose(|ctx| {
        ctx.translate(center)
            .rotate(LETTERBOX_ROTATION_DEG.deg())
            .add(namui::path(
                Path::new()
                    .move_to(-bar_width * 0.5, -bar_height * 0.5)
                    .line_to(bar_width * 0.5, -bar_height * 0.5)
                    .line_to(bar_width * 0.5, bar_height * 0.5)
                    .line_to(-bar_width * 0.5, bar_height * 0.5)
                    .close(),
                Paint::new(Color::BLACK.with_alpha(alpha)),
            ));
    });
}

#[derive(Clone, Copy)]
struct LetterboxMarqueeParams {
    elapsed_secs: f32,
    boss: bool,
    opacity: f32,
    top_bar: bool,
}

fn render_letterbox_marquee(
    ctx: &RenderCtx,
    screen_wh: Wh<Px>,
    bar_y: Px,
    bar_height: Px,
    params: LetterboxMarqueeParams,
) {
    let LetterboxMarqueeParams {
        elapsed_secs,
        boss,
        opacity,
        top_bar,
    } = params;
    let text_color = if boss {
        palette::RED
    } else {
        palette::DISABLED_TEXT
    }
    .with_alpha((opacity * 255.0).round() as u8);
    let font_size = bar_height * 0.28;
    let marquee_step = bar_height * 1.35;
    let scroll = px((elapsed_secs * MARQUEE_SPEED_PX_PER_SEC) % marquee_step.as_f32());
    let repeat_count = (screen_wh.width.as_f32() / marquee_step.as_f32()).ceil() as usize + 3;
    let center = letterbox_center(screen_wh, bar_y, bar_height, top_bar);

    for index in 0..repeat_count {
        let x = if top_bar {
            scroll + marquee_step * (index as f32 - 1.0)
        } else {
            -scroll + marquee_step * index as f32
        };
        ctx.compose(|ctx| {
            ctx.translate(center)
                .rotate(LETTERBOX_ROTATION_DEG.deg())
                .translate(Xy::new(x - screen_wh.width * 0.5, -font_size * 0.5))
                .add(memoized_text((), move |mut builder| {
                    builder
                        .headline()
                        .bold()
                        .size(FontSize::Custom { size: font_size })
                        .color(text_color)
                        .static_text(if boss { "BOSS" } else { "ENEMY" })
                        .render_left_top()
                }));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(waves: Vec<td_core::StageWaveState>) -> td_core::GameConfigState {
        td_core::GameConfigState {
            player: td_core::PlayerConfigState {
                max_hp_raw: 1,
                starting_gold: 0,
                starting_hp_raw: 1,
                base_dice_chance: 0,
                max_stages: 1,
                base_hand_slots: 1,
            },
            towers: td_core::TowerConfigState { entries: vec![] },
            monsters: td_core::MonsterConfigState {
                stats: vec![
                    td_core::MonsterConfigEntryState {
                        kind: MonsterKind::Mob01.to_core_raw(),
                        base_hp_raw: 10,
                        velocity_mul_raw: 1,
                        damage_raw: 1,
                        reward: 1,
                    },
                    td_core::MonsterConfigEntryState {
                        kind: MonsterKind::Mob02.to_core_raw(),
                        base_hp_raw: 100,
                        velocity_mul_raw: 1,
                        damage_raw: 1,
                        reward: 1,
                    },
                    td_core::MonsterConfigEntryState {
                        kind: MonsterKind::Boss01.to_core_raw(),
                        base_hp_raw: 1,
                        velocity_mul_raw: 1,
                        damage_raw: 1,
                        reward: 1,
                    },
                ],
                stage_waves: waves,
            },
        }
    }

    #[test]
    fn boss_wins_over_a_stronger_normal_monster() {
        let selected = strongest_enemy_for_stage(
            &config(vec![td_core::StageWaveState {
                stage: 3,
                entries: vec![
                    td_core::StageWaveEntryState {
                        kind: MonsterKind::Mob02.to_core_raw(),
                        count: 1,
                    },
                    td_core::StageWaveEntryState {
                        kind: MonsterKind::Boss01.to_core_raw(),
                        count: 1,
                    },
                ],
            }]),
            3,
        );
        assert_eq!(selected.monster_kind, MonsterKind::Boss01);
        assert!(!selected.used_fallback);
    }

    #[test]
    fn normal_selection_uses_highest_hp() {
        let selected = strongest_enemy_for_stage(
            &config(vec![td_core::StageWaveState {
                stage: 3,
                entries: vec![
                    td_core::StageWaveEntryState {
                        kind: MonsterKind::Mob01.to_core_raw(),
                        count: 1,
                    },
                    td_core::StageWaveEntryState {
                        kind: MonsterKind::Mob02.to_core_raw(),
                        count: 1,
                    },
                ],
            }]),
            3,
        );
        assert_eq!(selected.monster_kind, MonsterKind::Mob02);
    }

    #[test]
    fn missing_or_invalid_wave_uses_deterministic_fallback() {
        let selected = strongest_enemy_for_stage(
            &config(vec![td_core::StageWaveState {
                stage: 3,
                entries: vec![td_core::StageWaveEntryState {
                    kind: 255,
                    count: 1,
                }],
            }]),
            3,
        );
        assert_eq!(selected.monster_kind, MonsterKind::Mob01);
        assert!(selected.used_fallback);
    }

    #[test]
    fn intro_uses_one_curve_through_entry_middle_and_exit() {
        assert_eq!(intro_horizontal_progress(0.0), 0.0);
        assert!(intro_horizontal_progress(0.30) < intro_horizontal_progress(0.5));
        assert!((intro_horizontal_progress(0.5) - INTRO_BEZIER.sample(0.5)).abs() < 0.001);
        assert!(intro_horizontal_progress(0.5) < intro_horizontal_progress(0.70));
        assert_eq!(intro_horizontal_progress(1.0), 1.0);
        assert_eq!(intro_opacity(0.0), 0.0);
        assert!(intro_opacity(0.5) > 0.0);
        assert!(intro_opacity(0.70) > 0.0);
        assert!(intro_opacity(0.99) < intro_opacity(0.70));
        assert_eq!(intro_opacity(1.0), 0.0);
        assert_eq!(letterbox_opacity(0.0), 0.0);
        assert!((letterbox_opacity(0.5) - LETTERBOX_MAX_OPACITY).abs() < 0.001);
        assert!(letterbox_opacity(0.99) < LETTERBOX_MAX_OPACITY);

        let screen_width = px(100.0);
        assert_eq!(
            intro_info_text_offset(0.0, screen_width),
            screen_width + INTRO_INFO_MARGIN
        );
        assert!(
            intro_info_text_offset(1.0, screen_width) < intro_info_text_offset(0.0, screen_width)
        );
        assert_eq!(
            intro_info_text_offset(1.0, screen_width),
            -screen_width - INTRO_INFO_MARGIN
        );
    }

    #[test]
    fn intro_uses_the_shared_cubic_bezier_shape() {
        assert_eq!(INTRO_BEZIER.sample(0.0), 0.0);
        assert!((0.0..=1.0).contains(&INTRO_BEZIER.sample(0.2)));
        assert!((0.0..=1.0).contains(&INTRO_BEZIER.sample(0.5)));
        assert!((0.0..=1.0).contains(&INTRO_BEZIER.sample(0.8)));
        assert_eq!(INTRO_BEZIER.sample(1.0), 1.0);
    }
}
