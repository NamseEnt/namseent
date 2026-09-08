mod engravings;
mod stat_row;

use crate::card::Card;
use crate::card::render_polish_overlay;
use crate::game_state::modal::TowerDetailsModal;
use crate::game_state::tower::TowerKind;
use crate::game_state::{UserModal, dispatch_remove_tower, set_modal};
use crate::l10n::ui::TowerInfoPopupText;
use crate::theme;
use crate::theme::{
    button::{Button, ButtonColor, ButtonVariant},
    paper_container::{
        ArrowSide, PaperArrow, PaperContainerBackground, PaperTexture, PaperVariant,
    },
    typography::{FontSize, memoized_text},
};
use crate::{Damage, FixedRatio, SimTickSpan, TowerId, WorldDistance};
use engravings::{PopupEngravings, area_height};
use namui::*;
use namui_prebuilt::table;
use stat_row::PopupStatRow;

const BUBBLE_PADDING: Px = px(8.);
const BUBBLE_WIDTH: Px = px(220.);
const TOWER_NAME_ROW_HEIGHT: Px = px(32.);
const STAT_ROW_HEIGHT: Px = px(20.);
const REMOVE_BUTTON_HEIGHT: Px = px(28.);
const DETAILS_BUTTON_HEIGHT: Px = px(28.);
const BUTTON_GAP: Px = px(8.);
const SECTION_GAP: Px = px(8.);

pub struct TowerInfoPopup<'a> {
    pub tower: &'a td_core::TowerState,
}

struct PopupPolishOverlay {
    wh: Wh<Px>,
    bonus_pct: f32,
}

impl Component for PopupPolishOverlay {
    fn render(self, ctx: &RenderCtx) {
        render_polish_overlay(ctx, self.wh, self.bonus_pct, 1.0);
    }
}

impl Component for TowerInfoPopup<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { tower } = self;
        let game_state = crate::game_state::use_game_state(ctx);
        let text = game_state.text();
        let tower_id = TowerId::from_raw(tower.id.expect("placed tower must have an ID"));
        let tower_kind =
            TowerKind::from_core_raw(tower.template.kind).expect("raw tower kind must be valid");

        let damage = Damage::from_raw(tower.attack_damage_raw());
        let shoot_interval_secs =
            SimTickSpan::from_ticks(tower.shoot_interval_ticks()).as_secs_f32();
        let attack_speed = if shoot_interval_secs > 0.0 {
            1.0 / shoot_interval_secs
        } else {
            0.0
        };
        let dps = if shoot_interval_secs > 0.0 {
            damage.as_f32() / shoot_interval_secs
        } else {
            0.0
        };
        let range = WorldDistance::from_raw(tower.attack_range_raw());
        let total_damage = game_state
            .raw_core
            .metrics()
            .tower_damage_stats
            .iter()
            .find(|entry| entry.tower_id == tower_id.raw())
            .map(|entry| Damage::from_raw(entry.total_damage_raw).as_f32())
            .unwrap_or(0.0);
        let damage_label = text.tower_info_popup(TowerInfoPopupText::DamageLabel);
        let dps_label = text.tower_info_popup(TowerInfoPopupText::DpsLabel);
        let attack_speed_label = text.tower_info_popup(TowerInfoPopupText::AttackSpeedLabel);
        let range_label = text.tower_info_popup(TowerInfoPopupText::RangeLabel);
        let total_damage_label = text.tower_info_popup(TowerInfoPopupText::TotalDamageLabel);
        let reroll_count_label = text.tower_info_popup(TowerInfoPopupText::RerollCountLabel);
        let tower_name = text.tower(tower_kind.to_text()).to_string();
        let remove_text = text
            .tower_info_popup(TowerInfoPopupText::RemoveButton)
            .to_string();
        let details_text = text
            .tower_info_popup(TowerInfoPopupText::DetailsButton)
            .to_string();

        let used_cards = tower
            .template
            .used_cards
            .iter()
            .cloned()
            .filter_map(Card::from_core_state)
            .collect::<Vec<_>>();
        let card_polish_pct = FixedRatio::from_raw(
            tower
                .template
                .used_cards
                .iter()
                .map(|card| card.polish_pct_raw)
                .fold(0, i64::saturating_add),
        );
        let engravings = used_cards
            .iter()
            .filter_map(|card| card.engraving())
            .collect::<Vec<_>>();
        let engraving_overlay_height = area_height(engravings.len());
        let bubble_height = BUBBLE_PADDING * 2.0
            + TOWER_NAME_ROW_HEIGHT
            + STAT_ROW_HEIGHT * 6.0
            + engraving_overlay_height
            + SECTION_GAP
            + REMOVE_BUTTON_HEIGHT
            + BUTTON_GAP
            + DETAILS_BUTTON_HEIGHT;

        ctx.translate((-BUBBLE_WIDTH * 0.5, -bubble_height))
            .compose(|ctx| {
                ctx.compose(|ctx| {
                    table::padding_no_clip(BUBBLE_PADDING, |wh, ctx| {
                        let mut rows = vec![
                            table::fixed_no_clip(TOWER_NAME_ROW_HEIGHT, |wh, ctx| {
                                let tower_name_string = tower_name.clone();
                                ctx.add(memoized_text(
                                    (&tower_name_string, &wh.width),
                                    |mut builder| {
                                        builder
                                            .headline()
                                            .size(FontSize::Medium)
                                            .bold()
                                            .text(&tower_name_string)
                                            .render_center(wh)
                                    },
                                ));
                            }),
                            table::fixed_no_clip(STAT_ROW_HEIGHT, |wh, ctx| {
                                ctx.add(PopupStatRow {
                                    wh,
                                    label: dps_label,
                                    value: crate::format_compact_number(dps),
                                });
                            }),
                            table::fixed_no_clip(STAT_ROW_HEIGHT, |wh, ctx| {
                                ctx.add(PopupStatRow {
                                    wh,
                                    label: damage_label,
                                    value: format!("{:.1}", damage.as_f32()),
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
                                    value: format!(
                                        "{:.1}",
                                        range.raw() as f32
                                            / crate::world::WORLD_UNITS_PER_TILE as f32
                                    ),
                                });
                            }),
                            table::fixed_no_clip(STAT_ROW_HEIGHT, |wh, ctx| {
                                ctx.add(PopupStatRow {
                                    wh,
                                    label: total_damage_label,
                                    value: crate::format_compact_number(total_damage),
                                });
                            }),
                            table::fixed_no_clip(STAT_ROW_HEIGHT, |wh, ctx| {
                                ctx.add(PopupStatRow {
                                    wh,
                                    label: reroll_count_label,
                                    value: tower.template.rerolled_count.to_string(),
                                });
                            }),
                        ];

                        rows.push(table::fixed_no_clip(SECTION_GAP, |_, _| {}));
                        rows.push(table::fixed_no_clip(REMOVE_BUTTON_HEIGHT, {
                            let remove_text = remove_text.clone();
                            move |wh, ctx| {
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &move || {
                                            dispatch_remove_tower(tower_id);
                                        },
                                        &move |wh, _text_color, ctx| {
                                            let remove_text = remove_text.clone();
                                            ctx.add(memoized_text((), move |mut builder| {
                                                builder
                                                    .size(FontSize::Medium)
                                                    .bold()
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
                        }));
                        rows.push(table::fixed_no_clip(BUTTON_GAP, |_, _| {}));
                        rows.push(table::fixed_no_clip(DETAILS_BUTTON_HEIGHT, {
                            let detail_cards = used_cards.clone();
                            let details_text = details_text.clone();
                            move |wh, ctx| {
                                let detail_cards = detail_cards.clone();
                                ctx.add(
                                    Button::new(
                                        wh,
                                        &move || {
                                            set_modal(Some(UserModal::TowerDetails(
                                                TowerDetailsModal {
                                                    cards: detail_cards.clone(),
                                                },
                                            )));
                                        },
                                        &move |wh, _text_color, ctx| {
                                            let details_text = details_text.clone();
                                            ctx.add(memoized_text((), move |mut builder| {
                                                builder
                                                    .size(FontSize::Medium)
                                                    .bold()
                                                    .max_width(wh.width)
                                                    .text(details_text.clone())
                                                    .render_center(wh)
                                            }));
                                        },
                                    )
                                    .variant(ButtonVariant::Contained)
                                    .color(ButtonColor::Info),
                                );
                            }
                        }));

                        let stats_wh = Wh::new(wh.width, wh.height - engraving_overlay_height);
                        ctx.compose(|ctx| {
                            table::vertical(rows)(
                                stats_wh,
                                ctx.translate((0.px(), engraving_overlay_height)),
                            );
                        });
                        if !engravings.is_empty() {
                            ctx.add(PopupEngravings {
                                wh: Wh::new(wh.width, engraving_overlay_height),
                                engravings,
                            });
                        }
                    })(Wh::new(BUBBLE_WIDTH, bubble_height), ctx);
                });

                ctx.add(PopupPolishOverlay {
                    wh: Wh::new(BUBBLE_WIDTH, bubble_height),
                    bonus_pct: card_polish_pct.as_f32(),
                });

                ctx.add(PaperContainerBackground {
                    width: BUBBLE_WIDTH,
                    height: bubble_height,
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
