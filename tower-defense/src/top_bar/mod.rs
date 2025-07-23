mod hp_and_gold;
mod level;
mod level_up_details;
mod stage;

use crate::game_state::use_game_state;
use namui::*;
use namui_prebuilt::table;

const TOP_BAR_HEIGHT: Px = px(48.);
const ITEM_WIDTH: Px = px(256.);
const PADDING: Px = px(8.);

pub struct TopBar {
    pub screen_wh: Wh<Px>,
}
impl Component for TopBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;
        let game_state = use_game_state(ctx);
        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio(1, |_, _| {}),
                table::fixed(ITEM_WIDTH, |wh, ctx| {
                    ctx.add(crate::top_bar::hp_and_gold::HPAndGoldIndicator {
                        wh,
                        hp: (game_state.hp / 100.0).clamp(0.0, 1.0),
                        gold: game_state.gold,
                    });
                }),
                table::fixed(PADDING, |_, _| {}),
                table::fixed(ITEM_WIDTH, |wh, ctx| {
                    ctx.add(crate::top_bar::stage::StageIndicator {
                        wh,
                        stage: game_state.stage,
                    });
                }),
                table::fixed(PADDING, |_, _| {}),
                table::fixed(ITEM_WIDTH, |wh, ctx| {
                    ctx.add(crate::top_bar::level::LevelIndicator {
                        wh,
                        level: game_state.level.get(),
                        level_up_cost: game_state.level_up_cost(),
                        gold: game_state.gold,
                    });
                }),
                table::ratio(1, |_, _| {}),
            ])(Wh::new(screen_wh.width, TOP_BAR_HEIGHT), ctx);
        });
    }
}
