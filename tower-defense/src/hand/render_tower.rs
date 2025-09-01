use super::*;
use crate::{
    asset_loader::get_tower_asset,
    game_state::tower::{AnimationKind, TowerKind, TowerTemplate},
};
use namui::*;

pub(super) struct RenderTower<'a> {
    pub wh: Wh<Px>,
    pub tower_template: &'a TowerTemplate,
}
impl Component for RenderTower<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, tower_template } = self;

        let tower_image = get_tower_asset((tower_template.kind, AnimationKind::Idle1));

        // 바리케이드가 아닐 때만 좌상단에 rank와 suit 수직 배치
        if tower_template.kind != TowerKind::Barricade {
            render_top_left_rank_and_suit(ctx, tower_template.rank, tower_template.suit);
        }

        // 타워 이미지 렌더링
        ctx.compose(|ctx| {
            let Some(tower_image) = tower_image else {
                return;
            };

            ctx.add(image(ImageParam {
                rect: wh.to_rect(),
                image: tower_image.clone(),
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }));
        });

        render_background_rect(ctx, wh);
    }
}
