use super::*;
use crate::{
    card::render::render_damage_bonus::render_damage_bonus_overlay,
    game_state::tower::{
        AnimationKind, TowerTemplate,
        render::{TowerImage, TowerSpriteWithOverlay},
    },
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

        render_background_rect(ctx, wh);
    }
}
