use crate::game_state::TILE_PX_SIZE;
use namui::*;

use super::{TowerImage, TowerSpriteWithOverlay};
use crate::game_state::tower::Tower;

pub struct RenderTower<'a> {
    pub tower: &'a Tower,
    pub now: Instant,
}

impl Component for RenderTower<'_> {
    fn render(self, ctx: &RenderCtx) {
        let RenderTower { tower, now } = self;

        if let Some(visual) = tower.royal_straight_flush_visual() {
            render_tower_sprite(ctx, tower, (0.0, 0.0), visual.original_alpha(now));

            let clone_alpha = visual.clone_alpha(now);
            let tower_left_top = tower.left_top.map(|t| t as f32);
            for clone_center_xy in visual.clone_positions(now) {
                let clone_left_top = Xy::new(clone_center_xy.0 - 1.0, clone_center_xy.1 - 1.0);
                let local_offset = (
                    clone_left_top.x - tower_left_top.x,
                    clone_left_top.y - tower_left_top.y,
                );
                render_tower_sprite(ctx, tower, local_offset, clone_alpha);
            }
            return;
        }

        render_tower_sprite(ctx, tower, (0.0, 0.0), 1.0);
    }
}

fn render_tower_sprite(ctx: &RenderCtx, tower: &Tower, local_left_top_xy: (f32, f32), alpha: f32) {
    if alpha <= 0.01 {
        return;
    }

    let image = (tower.kind, tower.animation.kind).image();
    let image_wh = image.info().wh();
    let scale = Xy::new(
        1.0 + tower.animation.y_ratio_offset * -0.5,
        1.0 + tower.animation.y_ratio_offset,
    );
    ctx.translate(TILE_PX_SIZE.to_xy() * Xy::new(local_left_top_xy.0, local_left_top_xy.1))
        .translate((image_wh.width * 0.5, image_wh.height))
        .scale(scale)
        .translate(Xy::new(-image_wh.width * 0.5, -image_wh.height))
        .add(TowerSpriteWithOverlay {
            image,
            wh: image_wh,
            suit: Some(tower.suit),
            rank: Some(tower.rank),
            alpha,
        });
}
