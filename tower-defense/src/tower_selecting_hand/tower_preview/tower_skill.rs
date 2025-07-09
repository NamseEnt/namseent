use crate::{
    card::Rank,
    game_state::tower::{TowerSkillKind, TowerSkillTemplate},
    l10n::tower_skill::{TowerSkillText, TowerSkillTextLocale},
    l10n::upgrade::Locales,
    theme::{
        palette,
        typography::{FontSize, Headline, Paragraph, TextAlign},
    },
    tower_selecting_hand::PADDING,
};
use namui::*;
use namui_prebuilt::{simple_rect, table, typography};

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
        let symbol = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { .. } => "E",
            TowerSkillKind::NearbyTowerDamageAdd { .. } => "E",
            TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => "H",
            TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => "H",
            TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => "R",
            TowerSkillKind::NearbyMonsterSpeedMul { .. } => "D",
            TowerSkillKind::MoneyIncomeAdd { .. } => "B",
            TowerSkillKind::TopCardBonus { rank, .. } => match rank {
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            },
        };
        ctx.add(typography::body::center(wh, symbol, palette::ON_SURFACE));
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
    pub locale: &'a Locales,
}
impl Component for TowerEffectDescription<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { skill, locale } = self;

        let title = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { .. } => locale.tower_skill_text(&TowerSkillText::NearbyTowerDamageMulTitle),
            TowerSkillKind::NearbyTowerDamageAdd { .. } => locale.tower_skill_text(&TowerSkillText::NearbyTowerDamageAddTitle),
            TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => locale.tower_skill_text(&TowerSkillText::NearbyTowerAttackSpeedAddTitle),
            TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => locale.tower_skill_text(&TowerSkillText::NearbyTowerAttackSpeedMulTitle),
            TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => locale.tower_skill_text(&TowerSkillText::NearbyTowerAttackRangeAddTitle),
            TowerSkillKind::NearbyMonsterSpeedMul { .. } => locale.tower_skill_text(&TowerSkillText::NearbyMonsterSpeedMulTitle),
            TowerSkillKind::MoneyIncomeAdd { .. } => locale.tower_skill_text(&TowerSkillText::MoneyIncomeAddTitle),
            TowerSkillKind::TopCardBonus { .. } => locale.tower_skill_text(&TowerSkillText::TopCardBonusTitle),
        };
        let description = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { mul, range_radius } => locale.tower_skill_text(&TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius: range_radius as usize }),
            TowerSkillKind::NearbyTowerDamageAdd { add, range_radius } => locale.tower_skill_text(&TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius: range_radius as usize }),
            TowerSkillKind::NearbyTowerAttackSpeedAdd { add, range_radius } => locale.tower_skill_text(&TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius: range_radius as usize }),
            TowerSkillKind::NearbyTowerAttackSpeedMul { mul, range_radius } => locale.tower_skill_text(&TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius: range_radius as usize }),
            TowerSkillKind::NearbyTowerAttackRangeAdd { add, range_radius } => locale.tower_skill_text(&TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius: range_radius as usize }),
            TowerSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => locale.tower_skill_text(&TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius: range_radius as usize }),
            TowerSkillKind::MoneyIncomeAdd { add } => locale.tower_skill_text(&TowerSkillText::MoneyIncomeAddDesc { add }),
            TowerSkillKind::TopCardBonus { rank, bonus_damage } => locale.tower_skill_text(&TowerSkillText::TopCardBonusDesc { rank: rank.to_string(), bonus_damage }),
        };

        ctx.compose(|ctx| {
            let text_content = ctx.ghost_compose("TowerEffect description tooltip", |ctx| {
                table::vertical([
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(Headline {
                            text: title,
                            font_size: FontSize::Small,
                            text_align: TextAlign::LeftTop,
                            max_width: Some(TOWER_EFFECT_DESCRIPTION_MAXWIDTH),
                        });
                    }),
                    table::fixed(PADDING, |_, _| {}),
                    table::fit(table::FitAlign::LeftTop, |ctx| {
                        ctx.add(Paragraph {
                            text: description,
                            font_size: FontSize::Medium,
                            text_align: TextAlign::LeftTop,
                            max_width: Some(TOWER_EFFECT_DESCRIPTION_MAXWIDTH),
                        });
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
