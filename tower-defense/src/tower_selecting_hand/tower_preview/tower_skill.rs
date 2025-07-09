use crate::{
    card::Rank,
    game_state::tower::{TowerSkillKind, TowerSkillTemplate},
    l10n::tower_skill::TowerSkillText,
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
}
impl Component for TowerEffectDescription<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { skill } = self;

        let title = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { .. } => TowerSkillText::NearbyTowerDamageMulTitle.to_korean(),
            TowerSkillKind::NearbyTowerDamageAdd { .. } => TowerSkillText::NearbyTowerDamageAddTitle.to_korean(),
            TowerSkillKind::NearbyTowerAttackSpeedAdd { .. } => TowerSkillText::NearbyTowerAttackSpeedAddTitle.to_korean(),
            TowerSkillKind::NearbyTowerAttackSpeedMul { .. } => TowerSkillText::NearbyTowerAttackSpeedMulTitle.to_korean(),
            TowerSkillKind::NearbyTowerAttackRangeAdd { .. } => TowerSkillText::NearbyTowerAttackRangeAddTitle.to_korean(),
            TowerSkillKind::NearbyMonsterSpeedMul { .. } => TowerSkillText::NearbyMonsterSpeedMulTitle.to_korean(),
            TowerSkillKind::MoneyIncomeAdd { .. } => TowerSkillText::MoneyIncomeAddTitle.to_korean(),
            TowerSkillKind::TopCardBonus { .. } => TowerSkillText::TopCardBonusTitle.to_korean(),
        };
        let description = match skill.kind {
            TowerSkillKind::NearbyTowerDamageMul { mul, range_radius } => TowerSkillText::NearbyTowerDamageMulDesc { mul, range_radius: range_radius as usize }.to_korean(),
            TowerSkillKind::NearbyTowerDamageAdd { add, range_radius } => TowerSkillText::NearbyTowerDamageAddDesc { add, range_radius: range_radius as usize }.to_korean(),
            TowerSkillKind::NearbyTowerAttackSpeedAdd { add, range_radius } => TowerSkillText::NearbyTowerAttackSpeedAddDesc { add, range_radius: range_radius as usize }.to_korean(),
            TowerSkillKind::NearbyTowerAttackSpeedMul { mul, range_radius } => TowerSkillText::NearbyTowerAttackSpeedMulDesc { mul, range_radius: range_radius as usize }.to_korean(),
            TowerSkillKind::NearbyTowerAttackRangeAdd { add, range_radius } => TowerSkillText::NearbyTowerAttackRangeAddDesc { add, range_radius: range_radius as usize }.to_korean(),
            TowerSkillKind::NearbyMonsterSpeedMul { mul, range_radius } => TowerSkillText::NearbyMonsterSpeedMulDesc { mul, range_radius: range_radius as usize }.to_korean(),
            TowerSkillKind::MoneyIncomeAdd { add } => TowerSkillText::MoneyIncomeAddDesc { add }.to_korean(),
            TowerSkillKind::TopCardBonus { rank, bonus_damage } => TowerSkillText::TopCardBonusDesc { rank: rank.to_string(), bonus_damage }.to_korean(),
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
