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
impl Component for RenderTowerCard<'_> {
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

        if let Some((color, strength)) =
            damage_bonus_halo_config(tower_template.card_damage_bonus_pct())
        {
            let seed = (tower_template.kind as u32 as f32 * 0.618034).fract();
            ctx.add(CardHaloFx {
                wh,
                radius: wh.width * 0.22,
                color,
                strength,
                seed,
            });
        }

        render_background_rect(ctx, wh);
    }
}
