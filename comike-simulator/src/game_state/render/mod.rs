mod booth_customer_view;

use super::*;
use crate::*;

impl Component for &'_ GameState {
    fn render(self, ctx: &RenderCtx) {
        ctx.effect("spawn initial item on hands", || {
            mutate_game_state(|game_state| {
                game_state.spawn_initial_storage_cell_items();
            });
        });

        ctx.add(&self.physics_world);

        ctx.add(&self.hands);

        for item in self.items.values() {
            ctx.add(item);
        }

        match &self.view {
            GameView::BoothCustomer(view) => {
                ctx.add(view);
            }
            // GameView::GridStorageBox {
            //     xy,
            //     hands,
            //     items,
            //     physics_cell,
            // } => {
            //     ctx.compose(|ctx| render_hands(ctx, hands));

            //     ctx.compose(render_grid_storage_cell);

            //     for item in items.values() {
            //         ctx.compose(|ctx| render_item(ctx, item));
            //     }
            // }
            GameView::CustomerBooth => todo!(),
            GameView::BoothStock => todo!(),
            GameView::BoothFloor => todo!(),
        }
    }
}

impl Component for &'_ Hands {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(namui::path(
            Path::new().add_rect(HANDS_RECT),
            Paint::new(Color::RED).set_style(PaintStyle::Stroke),
        ));
    }
}
