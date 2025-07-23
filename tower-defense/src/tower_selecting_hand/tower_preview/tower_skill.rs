use crate::{
    game_state::tower::{TowerSkillKind, TowerSkillTemplate},
    game_state::use_game_state,
    l10n::tower_skill::TowerSkillText,
    theme::{
        palette,
        typography::{FontSize, TextAlign, headline, paragraph},
    },
    tower_selecting_hand::PADDING,
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
// TODO: Use image instead of text
impl Component for TowerSkillTemplateIcon<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            skill,
            on_mouse_move_in_effect_icon,
            on_mouse_move_out_effect_icon,
        } = self;
        // let symbol = match skill.kind {
        //     TowerSkillKind::NearbyTowerDamageMul { .. } => "E",
        //     TowerSkillKind::NearbyTowerDamageAdd { .. } => "E",
        //     TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => "H",
        //     TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => "H",
        //     TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => "R",
        //     TowerSkillKind::NearbyMonsterSpeedMul { .. } => "D",
        //     TowerSkillKind::MoneyIncomeAdd { .. } => "B",
        //     TowerSkillKind::TopCardBonus { rank, .. } => match rank {
        //         Rank::Seven => "7",
        //         Rank::Eight => "8",
        //         Rank::Nine => "9",
        //         Rank::Ten => "10",
        //         Rank::Jack => "J",
        //         Rank::Queen => "Q",
        //         Rank::King => "K",
        //         Rank::Ace => "A",
        //     },
        // };
        // ctx.add(typography::body::center(wh, symbol, palette::ON_SURFACE));
        ctx.add(simple_rect(
            wh,
            palette::OUTLINE,
            1.px(),
            palette::SURFACE_CONTAINER_HIGH,
        ))
        .attach_event(|event| {
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

pub(super) struct TowerEffectDescription<'a> {
    pub skill: &'a TowerSkillTemplate,
}
impl Component for TowerEffectDescription<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { skill } = self;
        let game_state = use_game_state(ctx);
        let title = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { .. } => game_state
                .text()
                .tower_skill(TowerSkillText::NearbyTowerDamageMulTitle),
            TowerSkillKind::NearbyTowerDamageAdd { .. } => game_state
                .text()
                .tower_skill(TowerSkillText::NearbyTowerDamageAddTitle),
            TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => game_state
                .text()
                .tower_skill(TowerSkillText::NearbyTowerAttackSpeedAddTitle),
            TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => game_state
                .text()
                .tower_skill(TowerSkillText::NearbyTowerAttackSpeedMulTitle),
            TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => game_state
                .text()
                .tower_skill(TowerSkillText::NearbyTowerAttackRangeAddTitle),
            TowerSkillKind::NearbyMonsterSpeedMul { .. } => game_state
                .text()
                .tower_skill(TowerSkillText::NearbyMonsterSpeedMulTitle),
            TowerSkillKind::MoneyIncomeAdd { .. } => game_state
                .text()
                .tower_skill(TowerSkillText::MoneyIncomeAddTitle),
            TowerSkillKind::TopCardBonus { .. } => game_state
                .text()
                .tower_skill(TowerSkillText::TopCardBonusTitle),
        };
        let description =
            match skill.kind {
                TowerSkillKind::NearbyTowerDamageMul { mul, range_radius } => game_state
                    .text()
                    .tower_skill(TowerSkillText::NearbyTowerDamageMulDesc {
                        mul,
                        range_radius: range_radius as usize,
                    }),
                TowerSkillKind::NearbyTowerDamageAdd { add, range_radius } => game_state
                    .text()
                    .tower_skill(TowerSkillText::NearbyTowerDamageAddDesc {
                        add,
                        range_radius: range_radius as usize,
                    }),
                TowerSkillKind::NearbyTowerAttackSpeedAdd { add, range_radius } => game_state
                    .text()
                    .tower_skill(TowerSkillText::NearbyTowerAttackSpeedAddDesc {
                        add,
                        range_radius: range_radius as usize,
                    }),
                TowerSkillKind::NearbyTowerAttackSpeedMul { mul, range_radius } => game_state
                    .text()
                    .tower_skill(TowerSkillText::NearbyTowerAttackSpeedMulDesc {
                        mul,
                        range_radius: range_radius as usize,
                    }),
                TowerSkillKind::NearbyTowerAttackRangeAdd { add, range_radius } => game_state
                    .text()
                    .tower_skill(TowerSkillText::NearbyTowerAttackRangeAddDesc {
                        add,
                        range_radius: range_radius as usize,
                    }),
                TowerSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => game_state
                    .text()
                    .tower_skill(TowerSkillText::NearbyMonsterSpeedMulDesc {
                        mul,
                        range_radius: range_radius as usize,
                    }),
                TowerSkillKind::MoneyIncomeAdd { add } => game_state
                    .text()
                    .tower_skill(TowerSkillText::MoneyIncomeAddDesc { add }),
                TowerSkillKind::TopCardBonus { rank, bonus_damage } => game_state
                    .text()
                    .tower_skill(TowerSkillText::TopCardBonusDesc {
                        rank: format!("{rank:?}"),
                        bonus_damage,
                    }),
            };

        ctx.compose(|ctx| {
            let text_content = ctx.ghost_compose("TowerEffect description tooltip", |ctx| {
                table::vertical([
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(
                            headline(title)
                                .size(FontSize::Small)
                                .align(TextAlign::LeftTop)
                                .max_width(TOWER_EFFECT_DESCRIPTION_MAXWIDTH)
                                .build(),
                        );
                    }),
                    table::fixed(PADDING, |_, _| {}),
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(
                            paragraph(description)
                                .size(FontSize::Medium)
                                .align(TextAlign::LeftTop)
                                .max_width(TOWER_EFFECT_DESCRIPTION_MAXWIDTH)
                                .build(),
                        );
                    }),
                ])(
                    Wh {
                        width: TOWER_EFFECT_DESCRIPTION_MAXWIDTH,
                        height: f32::MAX.px(),
                    },
                    ctx,
                );
            });

            let Some(text_content_wh) = bounding_box(&text_content).map(|rect| rect.wh()) else {
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

#[derive(Clone, Copy, PartialEq)]
pub(super) struct MouseHoveringSkill {
    pub skill: TowerSkillTemplate,
    pub offset: Xy<Px>,
}
