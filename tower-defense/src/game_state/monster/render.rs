use crate::{
    asset_loader::MONSTER_ASSET_LOADER_ATOM,
    game_state::{
        MonsterKind, TILE_PX_SIZE,
        monster::{MONSTER_HP_BAR_HEIGHT, Monster, monster_hp_bar::MonsterHpBar},
    },
};
use namui::*;

impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {
        let (monster_asset_loader, _) = ctx.atom(&MONSTER_ASSET_LOADER_ATOM);
        let image = monster_asset_loader.get(self.kind);

        let monster_wh = monster_wh(self.kind);

        if let Some(image) = image {
            ctx.translate(Xy::new(TILE_PX_SIZE.width * 0.5, TILE_PX_SIZE.height * 0.5))
                .add(namui::image(ImageParam {
                    rect: Rect::from_xy_wh(monster_wh.to_xy() * -0.5, monster_wh),
                    image,
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint: None,
                    },
                }));
        }

        let hp_bar_wh = Wh::new(monster_wh.width, MONSTER_HP_BAR_HEIGHT);
        ctx.translate(Xy::new(
            TILE_PX_SIZE.width * 0.5,
            TILE_PX_SIZE.width * 0.5 + monster_wh.height * 0.6,
        ))
        .add(MonsterHpBar {
            wh: hp_bar_wh,
            progress: self.hp / self.max_hp,
        });
    }
}

fn monster_wh(kind: MonsterKind) -> Wh<Px> {
    match kind {
        MonsterKind::Boss01
        | MonsterKind::Boss02
        | MonsterKind::Boss03
        | MonsterKind::Boss04
        | MonsterKind::Boss05
        | MonsterKind::Boss06
        | MonsterKind::Boss07
        | MonsterKind::Boss08
        | MonsterKind::Boss09
        | MonsterKind::Boss10
        | MonsterKind::Boss11 => TILE_PX_SIZE * 0.8,
        MonsterKind::Named01
        | MonsterKind::Named02
        | MonsterKind::Named03
        | MonsterKind::Named04
        | MonsterKind::Named05
        | MonsterKind::Named06
        | MonsterKind::Named07
        | MonsterKind::Named08
        | MonsterKind::Named09
        | MonsterKind::Named10
        | MonsterKind::Named11
        | MonsterKind::Named12
        | MonsterKind::Named13
        | MonsterKind::Named14
        | MonsterKind::Named15
        | MonsterKind::Named16 => TILE_PX_SIZE * 0.7,
        _ => TILE_PX_SIZE * 0.5,
    }
}
