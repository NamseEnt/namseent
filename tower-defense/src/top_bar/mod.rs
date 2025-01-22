use namui::*;
use namui_prebuilt::table;

const TOP_BAR_HEIGHT: Px = px(36.0);
const UPGRADE_BOARD_BUTTON_WIDTH: Px = px(36.0);

pub struct TopBar {
    screen_wh: Wh<Px>,
}
impl Component for TopBar {
    fn render(self, ctx: &RenderCtx) {
        let Self { screen_wh } = self;

        ctx.compose(|ctx| {
            table::horizontal([
                table::ratio(1, |_, _| {}),
                table::fixed_no_clip(UPGRADE_BOARD_BUTTON_WIDTH, |wh, ctx| {
                    ctx.add(UpgradeBoard { wh });
                }),
                table::ratio(1, |_, _| {}),
            ])(
                Wh {
                    width: screen_wh.width,
                    height: TOP_BAR_HEIGHT,
                },
                ctx,
            );
        });
    }
}
