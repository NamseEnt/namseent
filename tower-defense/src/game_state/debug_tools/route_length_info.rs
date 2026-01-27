use crate::game_state::use_game_state;
use crate::theme::typography;
use namui::*;
use namui_prebuilt::table;

pub struct RouteLengthInfoTool {
    pub width: Px,
}

impl Component for RouteLengthInfoTool {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);

        let route_length = game_state.route.iter_coords().len();
        let active_monsters = game_state.monsters.len();

        ctx.compose(|ctx| {
            table::vertical([
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(
                        typography::paragraph()
                            .text(&format!("Route Length: {}", route_length))
                            .build(),
                    );
                }),
                table::fit(table::FitAlign::LeftTop, |ctx| {
                    ctx.add(
                        typography::paragraph()
                            .text(&format!("Active Monsters: {}", active_monsters))
                            .build(),
                    );
                }),
            ])(Wh::new(self.width, f32::MAX.px()), ctx);
        });
    }
}
