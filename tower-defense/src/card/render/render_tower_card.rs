use super::*;
use crate::{
    card::render::render_damage_bonus::{damage_bonus_halo_config, render_damage_bonus_overlay},
    game_state::tower::{
        AnimationKind, TowerTemplate,
        render::{TowerImage, TowerSpriteWithOverlay},
    },
    theme::card_halo_fx::CardHaloFx,
};
use namui::*;

pub struct RenderTowerCard<'a> {
    pub wh: Wh<Px>,
    pub tower_template: &'a TowerTemplate,
}

struct RenderTowerCardInner<'a> {
    wh: Wh<Px>,
    tower_template: &'a TowerTemplate,
}

impl Component for RenderTowerCard<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        let bonus_pct = tower_template.card_damage_bonus_pct();
        let on_enter = move || {
            if bonus_pct > 0.0 {
                Some(crate::tooltip::TooltipContent::Word(
                    crate::l10n::word::Word::DamageBonus(Some(bonus_pct)),
                ))
            } else {
                None
            }
        };

        ctx.add(crate::tooltip::WithHoverArea {
            component_key: "tower_card_tooltip",
            component: RenderTowerCardInner { wh, tower_template },
            placement: crate::tooltip::TooltipPlacement::Above,
            on_enter,
            on_exit: || {},
        });

        if let Some((color, strength)) = damage_bonus_halo_config(bonus_pct) {
            let seed = (tower_template.kind as u32 as f32 * 0.618034).fract();
            ctx.add(CardHaloFx {
                wh,
                radius: wh.width * 0.22,
                color,
                strength,
                seed,
            });
        }
    }
}

impl<'a> Component for RenderTowerCardInner<'a> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;
        render_damage_bonus_overlay(ctx, wh, tower_template.card_damage_bonus_pct(), 1.0);

        let tower_image = (tower_template.kind, AnimationKind::Idle1).image();

        ctx.add(TowerSpriteWithOverlay {
            image: tower_image,
            wh,
            suit: tower_template.suit,
            rank: tower_template.rank,
            alpha: 1.0,
        });

        render_background_rect(ctx, wh);
    }
}
