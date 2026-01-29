use crate::flow_ui::selecting_tower::tower_selecting_hand::PADDING;
use crate::icon::IconKind;
use crate::thumbnail::ThumbnailComposer;
use crate::thumbnail::constants::OVERLAY_SIZE_RATIO;
use crate::thumbnail::overlay_rendering::OverlayPosition;
use crate::{
    game_state::tower::{TowerSkillKind, TowerSkillTemplate},
    game_state::use_game_state,
    l10n::tower_skill::TowerSkillText,
    theme::{
        palette,
        typography::{self, FontSize},
    },
};
use namui::*;
use namui_prebuilt::{simple_rect, table};

const TOWER_EFFECT_DESCRIPTION_MAXWIDTH: Px = px(192.);

pub struct TowerSkillTemplateIcon<'a> {
    pub wh: Wh<Px>,
    pub skill: &'a TowerSkillTemplate,
    pub on_mouse_move_in_effect_icon: &'a dyn Fn(&TowerSkillTemplate, Xy<Px>),
    pub on_mouse_move_out_effect_icon: &'a dyn Fn(&TowerSkillTemplate),
}
impl Component for TowerSkillTemplateIcon<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            skill,
            on_mouse_move_in_effect_icon,
            on_mouse_move_out_effect_icon,
        } = self;
        // compose skill thumbnail using existing thumbnail composer utilities
        let thumbnail = compose_skill_thumbnail(wh, &skill.kind);

        // background frame + composed thumbnail
        let bg = simple_rect(
            wh,
            palette::OUTLINE,
            1.px(),
            palette::SURFACE_CONTAINER_HIGH,
        );
        let composed = namui::render(vec![thumbnail, bg]);

        ctx.add(composed).attach_event(|event| {
            match event {
                Event::MouseMove { event } => {
                    match event.is_local_xy_in() {
                        true => on_mouse_move_in_effect_icon(skill, event.global_xy),
                        false => on_mouse_move_out_effect_icon(skill),
                    };
                }
                Event::VisibilityChange => {
                    on_mouse_move_out_effect_icon(skill);
                }
                _ => {}
            };
        });
    }
}

fn compose_skill_thumbnail(wh: Wh<Px>, kind: &TowerSkillKind) -> RenderingTree {
    match *kind {
        TowerSkillKind::NearbyTowerDamageMul { .. } => ThumbnailComposer::new(wh)
            .with_icon_base(IconKind::AttackDamage)
            .add_icon_overlay(
                IconKind::Multiply,
                OverlayPosition::BottomRight,
                OVERLAY_SIZE_RATIO,
            )
            .build(),
        TowerSkillKind::NearbyTowerDamageAdd { .. } => ThumbnailComposer::new(wh)
            .with_icon_base(IconKind::AttackDamage)
            .add_plus_overlay()
            .build(),
        TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => ThumbnailComposer::new(wh)
            .with_icon_base(IconKind::AttackSpeed)
            .add_plus_overlay()
            .build(),
        TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => ThumbnailComposer::new(wh)
            .with_icon_base(IconKind::AttackSpeed)
            .add_icon_overlay(
                IconKind::Multiply,
                OverlayPosition::BottomRight,
                OVERLAY_SIZE_RATIO,
            )
            .build(),
        TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => ThumbnailComposer::new(wh)
            .with_icon_base(IconKind::AttackRange)
            .add_plus_overlay()
            .build(),
        TowerSkillKind::NearbyMonsterSpeedMul { mul, .. } => {
            let dir_icon = if mul < 1.0 {
                IconKind::Down
            } else {
                IconKind::Up
            };
            ThumbnailComposer::new(wh)
                .with_icon_base(IconKind::MoveSpeed)
                .add_icon_overlay(dir_icon, OverlayPosition::BottomRight, OVERLAY_SIZE_RATIO)
                .build()
        }
        TowerSkillKind::MoneyIncomeAdd { .. } => ThumbnailComposer::new(wh)
            .with_icon_base(IconKind::Gold)
            .add_plus_overlay()
            .build(),
        TowerSkillKind::TopCardBonus { rank, .. } => ThumbnailComposer::new(wh)
            .with_icon_base(IconKind::AttackDamage)
            .add_rank_overlay(rank)
            .add_plus_overlay()
            .build(),
    }
}

pub(super) struct TowerEffectDescription<'a> {
    pub skill: &'a TowerSkillTemplate,
}
impl Component for TowerEffectDescription<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { skill } = self;
        let game_state = use_game_state(ctx);

        ctx.compose(|ctx| {
            let text_content = ctx.ghost_compose("TowerEffect description tooltip", |ctx| {
                let locale = &game_state.locale;
                table::vertical([
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        let skill_title = match skill.kind {
                            TowerSkillKind::NearbyTowerDamageMul { .. } => {
                                TowerSkillText::NearbyTowerDamageMulTitle
                            }
                            TowerSkillKind::NearbyTowerDamageAdd { .. } => {
                                TowerSkillText::NearbyTowerDamageAddTitle
                            }
                            TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => {
                                TowerSkillText::NearbyTowerAttackSpeedAddTitle
                            }
                            TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => {
                                TowerSkillText::NearbyTowerAttackSpeedMulTitle
                            }
                            TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => {
                                TowerSkillText::NearbyTowerAttackRangeAddTitle
                            }
                            TowerSkillKind::NearbyMonsterSpeedMul { .. } => {
                                TowerSkillText::NearbyMonsterSpeedMulTitle
                            }
                            TowerSkillKind::MoneyIncomeAdd { .. } => {
                                TowerSkillText::MoneyIncomeAddTitle
                            }
                            TowerSkillKind::TopCardBonus { .. } => {
                                TowerSkillText::TopCardBonusTitle
                            }
                        };
                        ctx.add(typography::memoized_text((), |builder| {
                            builder
                                .headline()
                                .size(FontSize::Small)
                                .max_width(TOWER_EFFECT_DESCRIPTION_MAXWIDTH)
                                .l10n(skill_title.clone(), locale)
                                .render_left_top()
                        }));
                    }),
                    table::fixed(PADDING, |_, _| {}),
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        let skill_desc = match skill.kind {
                            TowerSkillKind::NearbyTowerDamageMul { mul, range_radius } => {
                                TowerSkillText::NearbyTowerDamageMulDesc {
                                    mul,
                                    range_radius: range_radius as usize,
                                }
                            }
                            TowerSkillKind::NearbyTowerDamageAdd { add, range_radius } => {
                                TowerSkillText::NearbyTowerDamageAddDesc {
                                    add,
                                    range_radius: range_radius as usize,
                                }
                            }
                            TowerSkillKind::NearbyTowerAttackSpeedAdd { add, range_radius } => {
                                TowerSkillText::NearbyTowerAttackSpeedAddDesc {
                                    add,
                                    range_radius: range_radius as usize,
                                }
                            }
                            TowerSkillKind::NearbyTowerAttackSpeedMul { mul, range_radius } => {
                                TowerSkillText::NearbyTowerAttackSpeedMulDesc {
                                    mul,
                                    range_radius: range_radius as usize,
                                }
                            }
                            TowerSkillKind::NearbyTowerAttackRangeAdd { add, range_radius } => {
                                TowerSkillText::NearbyTowerAttackRangeAddDesc {
                                    add,
                                    range_radius: range_radius as usize,
                                }
                            }
                            TowerSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => {
                                TowerSkillText::NearbyMonsterSpeedMulDesc {
                                    mul,
                                    range_radius: range_radius as usize,
                                }
                            }
                            TowerSkillKind::MoneyIncomeAdd { add } => {
                                TowerSkillText::MoneyIncomeAddDesc { add }
                            }
                            TowerSkillKind::TopCardBonus { rank, bonus_damage } => {
                                TowerSkillText::TopCardBonusDesc {
                                    rank: format!("{rank:?}"),
                                    bonus_damage,
                                }
                            }
                        };
                        ctx.add(typography::memoized_text((), |builder| {
                            builder
                                .paragraph()
                                .size(FontSize::Medium)
                                .max_width(TOWER_EFFECT_DESCRIPTION_MAXWIDTH)
                                .l10n(skill_desc.clone(), locale)
                                .render_left_top()
                        }));
                    }),
                ])(
                    Wh {
                        width: TOWER_EFFECT_DESCRIPTION_MAXWIDTH,
                        height: f32::MAX.px(),
                    },
                    ctx,
                );
            });

            let Some(text_content_wh) = text_content.bounding_box().map(|rect| rect.wh()) else {
                return;
            };

            let ctx = ctx.translate((0.px(), -text_content_wh.height - PADDING * 2.0));

            ctx.translate(Xy::single(PADDING)).add(text_content);

            ctx.add(simple_rect(
                text_content_wh + Wh::single(PADDING * 2.0),
                palette::OUTLINE,
                1.px(),
                palette::SURFACE_CONTAINER_HIGH,
            ));
        });
    }
}
