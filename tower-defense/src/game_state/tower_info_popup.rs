use super::{Tower, mutate_game_state};
use crate::game_state::GameEffectEvent;
use crate::l10n::ui::TowerInfoPopupText;
use crate::theme::{
    button::{Button, ButtonColor, ButtonVariant},
    paper_container::{
        ArrowSide, PaperArrow, PaperContainerBackground, PaperTexture, PaperVariant,
    },
    typography::{FontSize, memoized_text},
};
use crate::{sound, theme};
use namui::*;
use namui_prebuilt::table;

const BUBBLE_PADDING: Px = px(8.);
const BUBBLE_WIDTH: Px = px(220.);
const BUBBLE_HEIGHT: Px = px(132.);
const STAT_ROW_HEIGHT: Px = px(20.);
const REMOVE_BUTTON_HEIGHT: Px = px(28.);

pub struct TowerInfoPopup<'a> {
    pub tower: &'a Tower,
}

struct PopupStatRow {
    wh: Wh<Px>,
    label: &'static str,
    value: String,
}

impl Component for PopupStatRow {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, label, value } = self;

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio_no_clip(1, |wh, ctx| {
                    let label_string = label.to_string();
                    ctx.add(memoized_text((&label_string, &wh.width), |mut builder| {
                        builder
                            .paragraph()
                            .size(FontSize::Small)
                            .bold()
                            .color(theme::palette::WHITE)
                            .stroke(2.px(), theme::palette::DARK_CHARCOAL)
                            .text(&label_string)
                            .render_left_center(wh.height)
                    }));
                }),
                table::ratio_no_clip(1, |wh, ctx| {
                    let value_string = value.clone();
                    ctx.add(memoized_text((&value_string, &wh.width), |mut builder| {
                        builder
                            .paragraph()
                            .size(FontSize::Small)
                            .bold()
                            .color(theme::palette::WHITE)
                            .stroke(2.px(), theme::palette::DARK_CHARCOAL)
                            .text(&value_string)
                            .render_right_center(wh)
                    }));
                }),
            ])(wh, ctx);
        });
    }
}

impl Component for TowerInfoPopup<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { tower } = self;
        let game_state = crate::game_state::use_game_state(ctx);
        let text = game_state.text();

        let damage = tower.cached_upgrade_damage();
        let shoot_interval_secs = tower.shoot_interval.as_secs_f32();
        let attack_speed = if shoot_interval_secs > 0.0 {
            1.0 / shoot_interval_secs
        } else {
            0.0
        };
        let range = tower.attack_range_radius(1.0);
        let total_damage = game_state
            .metrics
            .tower_damage_stats
            .iter()
            .find(|entry| entry.tower_id == tower.id())
            .map(|entry| entry.total_damage)
            .unwrap_or(0.0);
        let damage_label = text.tower_info_popup(TowerInfoPopupText::DamageLabel);
        let attack_speed_label = text.tower_info_popup(TowerInfoPopupText::AttackSpeedLabel);
        let range_label = text.tower_info_popup(TowerInfoPopupText::RangeLabel);
        let total_damage_label = text.tower_info_popup(TowerInfoPopupText::TotalDamageLabel);

        ctx.translate((-BUBBLE_WIDTH * 0.5, -BUBBLE_HEIGHT))
            .compose(|ctx| {
                ctx.compose(|ctx| {
                    table::padding_no_clip(BUBBLE_PADDING, |wh, ctx| {
                        table::vertical([
                            table::fixed_no_clip(STAT_ROW_HEIGHT, |wh, ctx| {
                                ctx.add(PopupStatRow {
                                    wh,
                                    label: damage_label,
                                    value: format!("{damage:.1}"),
                                });
                            }),
                            table::fixed_no_clip(STAT_ROW_HEIGHT, |wh, ctx| {
                                ctx.add(PopupStatRow {
                                    wh,
                                    label: attack_speed_label,
                                    value: format!("{attack_speed:.2}"),
                                });
                            }),
                            table::fixed_no_clip(STAT_ROW_HEIGHT, |wh, ctx| {
                                ctx.add(PopupStatRow {
                                    wh,
                                    label: range_label,
                                    value: format!("{range:.1}"),
                                });
                            }),
                            table::fixed_no_clip(STAT_ROW_HEIGHT, |wh, ctx| {
                                ctx.add(PopupStatRow {
                                    wh,
                                    label: total_damage_label,
                                    value: crate::format_compact_number(total_damage),
                                });
                            }),
                            table::fixed_no_clip(px(8.0), |_, _| {}),
                            table::fixed_no_clip(REMOVE_BUTTON_HEIGHT, {
                                let remove_text =
                                    text.tower_info_popup(TowerInfoPopupText::RemoveButton).to_string();
                                move |wh, ctx| {
                                    let tower_id = tower.id();
                                    ctx.add(
                                        Button::new(
                                            wh,
                                            &move || {
                                                mutate_game_state(move |game_state| {
                                                    let tower_removed = game_state.action(
                                                        crate::game_state::GameStateAction::RemoveTower(
                                                            tower_id,
                                                        ),
                                                    );
                                                    if tower_removed {
                                                        game_state.effect_events.push(
                                                            GameEffectEvent::PlaySound(
                                                                sound::EmitSoundParams::one_shot(
                                                                    sound::random_paper_crumpling(),
                                                                    sound::SoundGroup::Sfx,
                                                                    sound::VolumePreset::High,
                                                                    sound::SpatialMode::NonSpatial,
                                                                ),
                                                            ),
                                                        );
                                                    }
                                                });
                                            },
                                            &move |wh, _text_color, ctx| {
                                                let remove_text = remove_text.clone();
                                                ctx.add(memoized_text((), move |mut builder| {
                                                    builder
                                                        .size(FontSize::Medium)
                                                        .bold()
                                                        .color(theme::palette::WHITE)
                                                        .stroke(2.px(), theme::palette::DARK_CHARCOAL)
                                                        .max_width(wh.width)
                                                        .text(remove_text.clone())
                                                        .render_center(wh)
                                                }));
                                            },
                                        )
                                        .variant(ButtonVariant::Contained)
                                        .color(ButtonColor::Error),
                                    );
                                }
                            }),
                        ])(wh, ctx);
                    })(Wh::new(BUBBLE_WIDTH, BUBBLE_HEIGHT), ctx);
                });

                ctx.add(PaperContainerBackground {
                    width: BUBBLE_WIDTH,
                    height: BUBBLE_HEIGHT,
                    texture: PaperTexture::Rough,
                    variant: PaperVariant::Sticky,
                    color: theme::palette::SURFACE_CONTAINER_HIGHEST,
                    outline_color: Some(theme::palette::SURFACE_CONTAINER_OUTLINE),
                    shadow: true,
                    arrow: Some(PaperArrow {
                        side: ArrowSide::Bottom,
                        width: px(16.0),
                        height: px(16.0),
                        offset: BUBBLE_WIDTH * 0.5,
                    }),
                });
            })
            .attach_event(|event| {
                if let Event::MouseDown { event } = event
                    && let Some(MouseButton::Left) = event.button
                    && event.is_local_xy_in()
                {
                    event.stop_propagation();
                }
            });
    }
}
