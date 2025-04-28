use super::*;
use crate::*;

impl Component for &'_ GameState {
    fn render(self, ctx: &RenderCtx) {
        match &self.view {
            GameView::BoothCustomer => todo!(),
            GameView::GridStorageBox { xy, hands, items } => {
                render_hands(ctx, hands);

                render_grid_storage_cell(ctx);

                // let cell = &self.grid_storage_box.cells[xy.y][xy.x];
                for item in items.values() {
                    render_item(ctx, item);
                }
            }
            GameView::CustomerBooth => todo!(),
            GameView::BoothStock => todo!(),
            GameView::BoothFloor => todo!(),
        }
    }
}

fn render_hands(ctx: &RenderCtx, hands: &PhysicsHands) {
    let path = Path::new().add_rect(HANDS_RECT);
    let paint = Paint::new(Color::RED).set_style(PaintStyle::Stroke);
    ctx.add(namui::path(path, paint));
}

fn render_item(ctx: &RenderCtx, item: &PhysicsItem) {
    ctx.translate(item.xy)
        .rotate(item.rotation)
        .add_with_key(
            item.id,
            match item.item_kind {
                ItemKind::Sticker => namui::path(
                    Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), Wh::new(50.px(), 100.px()))),
                    Paint::new(Color::RED),
                ),
            },
        )
        .attach_event(|event| {
            let Event::MouseDown { event } = event else {
                return;
            };
            if event.is_local_xy_in() {
                game_state::on_game_event(GameEvent::ItemMouseDown {
                    id: item.id,
                    mouse_global_xy: event.global_xy,
                });
            }
        });
}

fn render_grid_storage_cell(ctx: &RenderCtx) {
    let rect = GRID_STORAGE_CELL_RECT;
    let path = Path::new()
        .move_to(rect.right(), rect.top())
        .line_to(rect.left(), rect.top())
        .line_to(rect.left(), rect.bottom())
        .line_to(rect.right(), rect.bottom());
    let paint = Paint::new(Color::RED).set_style(PaintStyle::Stroke);
    ctx.add(namui::path(path, paint));
}
