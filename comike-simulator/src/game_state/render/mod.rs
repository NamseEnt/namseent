mod booth_customer_view;

use super::*;
use crate::*;

impl Component for &'_ GameState {
    fn render(self, ctx: &RenderCtx) {
        ctx.effect("spawn initial item on hands", || {
            mutate_game_state(|game_state| {
                game_state.spawn_item_on_hands();
            });
        });

        ctx.add(&self.physics_world);

        ctx.add(&self.hands);

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

fn render_item(ctx: ComposeCtx, item: &PhysicsItem) {
    let rt = namui::translate(
        item.center.x,
        item.center.y,
        namui::rotate(
            item.rotation,
            namui::translate(
                -item.item_kind.wh().as_xy().x / 2,
                -item.item_kind.wh().as_xy().y / 2,
                match item.item_kind {
                    ItemKind::Sticker => namui::path(
                        Path::new()
                            .add_rect(Rect::from_xy_wh(Xy::zero(), Wh::new(50.px(), 100.px()))),
                        Paint::new(Color::from_f01(0.5, 0., 0., 0.7)),
                    ),
                },
            ),
        ),
    );

    let bounding_box = namui::bounding_box(&rt).unwrap();
    ctx.add(namui::path(
        Path::new().add_rect(bounding_box),
        Paint::new(Color::from_f01(0., 0., 1., 0.7))
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(3.px()),
    ));

    ctx.add_with_key(item.id, rt).attach_event(|event| {
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
