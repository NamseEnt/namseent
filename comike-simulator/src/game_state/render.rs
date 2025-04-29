use super::*;
use crate::*;

impl Component for &'_ GameState {
    fn render(self, ctx: &RenderCtx) {
        self.render_physics_debug(ctx);

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
impl GameState {
    fn render_physics_debug(&self, ctx: &RenderCtx) {
        let paint = Paint::new(Color::from_f01(0., 1., 0., 0.1));

        for (_, collider) in self.physics_world.collider_set.iter() {
            let translation = collider.translation();
            let x = translation.x.px();
            let y = translation.y.px();
            let path = match collider.shape().as_typed_shape() {
                TypedShape::Ball(ball) => todo!(),
                TypedShape::Cuboid(cuboid) => Path::new().add_rect(Rect::from_xy_wh(
                    Xy::new(-cuboid.half_extents.x.px(), -cuboid.half_extents.y.px()),
                    Wh::new(
                        cuboid.half_extents.x.px() * 2,
                        cuboid.half_extents.y.px() * 2,
                    ),
                )),
                TypedShape::Capsule(capsule) => todo!(),
                TypedShape::Segment(segment) => todo!(),
                TypedShape::Triangle(triangle) => todo!(),
                TypedShape::TriMesh(tri_mesh) => todo!(),
                TypedShape::Polyline(polyline) => todo!(),
                TypedShape::HalfSpace(half_space) => todo!(),
                TypedShape::HeightField(height_field) => todo!(),
                TypedShape::Compound(compound) => todo!(),
                TypedShape::ConvexPolygon(convex_polygon) => todo!(),
                TypedShape::RoundCuboid(round_shape) => todo!(),
                TypedShape::RoundTriangle(round_shape) => todo!(),
                TypedShape::RoundConvexPolygon(round_shape) => todo!(),
                TypedShape::Custom(shape) => todo!(),
            };
            ctx.compose(|ctx| {
                ctx.translate((x, y)).add(namui::path(path, paint.clone()));
            });
        }
    }
}

fn render_hands(ctx: &RenderCtx, hands: &PhysicsHands) {
    let path = Path::new().add_rect(HANDS_RECT);
    let paint = Paint::new(Color::RED).set_style(PaintStyle::Stroke);
    ctx.add(namui::path(path, paint));
}

fn render_item(ctx: &RenderCtx, item: &PhysicsItem) {
    ctx.translate(item.center - item.item_kind.wh().as_xy() / 2)
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
