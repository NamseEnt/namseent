use super::*;
use crate::game_state::tower::{
    AnimationKind, TowerTemplate,
    render::{TowerImage, TowerSpriteWithOverlay},
};
use namui::*;

pub(super) struct RenderTower<'a> {
    pub wh: Wh<Px>,
    pub tower_template: &'a TowerTemplate,
}
impl Component for RenderTower<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        let tower_image = (tower_template.kind, AnimationKind::Idle1).image();

        ctx.add(TowerSpriteWithOverlay {
            image: tower_image,
            wh,
            suit: Some(tower_template.suit),
            rank: Some(tower_template.rank),
            alpha: 1.0,
        });

        render_background_rect(ctx, wh);
    }
}
