use crate::game_state::{
    TILE_PX_SIZE,
    monster::{MONSTER_HP_BAR_HEIGHT, Monster, monster_hp_bar::MonsterHpBar},
};
use namui::*;

impl Component for &Monster {
    fn render(self, ctx: &RenderCtx) {
        // TODO: add monster image
        let monster_wh = TILE_PX_SIZE * 0.6;
        let path = Path::new().add_oval(Rect::from_xy_wh(monster_wh.to_xy() * -0.5, monster_wh));
        let paint = Paint::new(Color::RED);
        ctx.translate(TILE_PX_SIZE.to_xy() * 0.5)
            .add(namui::path(path, paint));

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
