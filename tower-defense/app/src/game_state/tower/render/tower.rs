use namui::*;

use super::{TowerImage, TowerSpriteWithOverlay};
use crate::card::render::polish_halo_config;
use crate::game_state::tower::AnimationKind;
use crate::game_state::tower::Tower;
use crate::theme::card_halo_fx::CardHaloFx;

pub struct RenderTower<'a> {
    pub tower: &'a Tower,
    pub animation_kind: AnimationKind,
    pub y_ratio_offset: f32,
}

impl Component for RenderTower<'_> {
    fn render(self, ctx: &RenderCtx) {
        let RenderTower {
            tower,
            animation_kind,
            y_ratio_offset,
        } = self;

        render_tower_sprite(ctx, tower, animation_kind, y_ratio_offset);
    }
}

fn render_tower_sprite(
    ctx: &RenderCtx,
    tower: &Tower,
    animation_kind: AnimationKind,
    y_ratio_offset: f32,
) {
    let image = (tower.kind, animation_kind).image();
    let image_wh = image.info().wh();
    let scale = Xy::new(1.0 + y_ratio_offset * -0.5, 1.0 + y_ratio_offset);

    let center = (image_wh.width * 0.5, image_wh.height);

    ctx.translate(center)
        .scale(scale)
        .translate(Xy::new(-image_wh.width * 0.5, -image_wh.height))
        .add(TowerSpriteWithOverlay {
            image,
            wh: image_wh,
            suit: tower.suit,
            rank: tower.rank,
            alpha: 1.0,
        });

    let bonus_pct = tower.template.card_polish_pct();
    if let Some((color, strength)) = polish_halo_config(bonus_pct.as_f32()) {
        ctx.translate(center)
            .translate(Xy::new(-image_wh.width * 0.375, -image_wh.height * 0.875))
            .add(CardHaloFx {
                wh: image_wh * 0.75,
                radius: image_wh.width * 0.5,
                color,
                strength,
                seed: (tower.id().raw() as f32 * 0.618034).fract(),
            });
    }
}
