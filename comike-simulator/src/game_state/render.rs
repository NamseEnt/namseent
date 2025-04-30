use super::*;
use crate::*;

impl Component for &'_ GameState {
    fn render(self, ctx: &RenderCtx) {
        self.render_physics_debug(ctx);

        match &self.view {
            GameView::BoothCustomer => todo!(),
            GameView::GridStorageBox {
                xy,
                hands,
                items,
                physics_cell,
            } => {
                ctx.compose(|ctx| render_hands(ctx, hands));

                ctx.compose(render_grid_storage_cell);

                for item in items.values() {
                    ctx.compose(|ctx| render_item(ctx, item));
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
        let color = Color::from_f01(0., 1., 0., 1.);
        let fill_paint = Paint::new(color).set_style(PaintStyle::Fill);
        let stroke_paint = Paint::new(color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(1.px());

        for (_joint_handle, joint) in self.physics_world.impulse_joint_set.iter() {
            let Some((rigid_body_1, rigid_body_2)) = self
                .physics_world
                .rigid_body_set
                .get(joint.body1)
                .zip(self.physics_world.rigid_body_set.get(joint.body2))
            else {
                continue;
            };
            let anchor_xy_1 = rigid_body_1
                .position()
                .transform_point(&joint.data.local_anchor1());
            let anchor_xy_2 = rigid_body_2
                .position()
                .transform_point(&joint.data.local_anchor2());
            let x1 = anchor_xy_1.x.px() * PHYSICS_WORLD_MAGNIFICATION;
            let y1 = anchor_xy_1.y.px() * PHYSICS_WORLD_MAGNIFICATION;
            let x2 = anchor_xy_2.x.px() * PHYSICS_WORLD_MAGNIFICATION;
            let y2 = anchor_xy_2.y.px() * PHYSICS_WORLD_MAGNIFICATION;
            ctx.compose(|ctx| {
                ctx.add(namui::path(
                    Path::new().move_to(x1, y1).line_to(x2, y2),
                    stroke_paint.clone(),
                ));
            });
        }

        for (_, rigid_body) in self.physics_world.rigid_body_set.iter() {
            let translation = rigid_body.translation();
            let x = translation.x.px() * PHYSICS_WORLD_MAGNIFICATION;
            let y = translation.y.px() * PHYSICS_WORLD_MAGNIFICATION;
            ctx.compose(|ctx| {
                ctx.translate((x, y))
                    .rotate(rigid_body.rotation().angle().rad())
                    .add(namui::path(
                        Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), Wh::new(1.px(), 1.px()))),
                        fill_paint.clone(),
                    ));
            });
        }

        for (_, collider) in self.physics_world.collider_set.iter() {
            let translation = collider.translation();
            let x = translation.x.px() * PHYSICS_WORLD_MAGNIFICATION;
            let y = translation.y.px() * PHYSICS_WORLD_MAGNIFICATION;
            let (path, paint) = match collider.shape().as_typed_shape() {
                TypedShape::Ball(ball) => todo!(),
                TypedShape::Cuboid(cuboid) => (
                    Path::new().add_rect(
                        Rect::from_xy_wh(
                            Xy::new(-cuboid.half_extents.x.px(), -cuboid.half_extents.y.px()),
                            Wh::new(
                                cuboid.half_extents.x.px() * 2,
                                cuboid.half_extents.y.px() * 2,
                            ),
                        ) * PHYSICS_WORLD_MAGNIFICATION,
                    ),
                    stroke_paint.clone(),
                ),
                TypedShape::Capsule(capsule) => todo!(),
                TypedShape::Segment(segment) => todo!(),
                TypedShape::Triangle(triangle) => todo!(),
                TypedShape::TriMesh(tri_mesh) => todo!(),
                TypedShape::Polyline(polyline) => {
                    let mut path = Path::new();
                    let mut first = true;
                    for vertex in polyline.vertices() {
                        let x = vertex.x.px() * PHYSICS_WORLD_MAGNIFICATION;
                        let y = vertex.y.px() * PHYSICS_WORLD_MAGNIFICATION;
                        path = if first {
                            first = false;
                            path.move_to(x, y)
                        } else {
                            path.line_to(x, y)
                        };
                    }
                    (path, stroke_paint.clone())
                }
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
                ctx.translate((x, y))
                    .rotate(collider.rotation().angle().rad())
                    .add(namui::path(path, paint.clone()));
            });
        }
    }
}

fn render_hands(ctx: ComposeCtx, hands: &PhysicsHands) {
    let path = Path::new().add_rect(HANDS_RECT);
    let paint = Paint::new(Color::RED).set_style(PaintStyle::Stroke);
    ctx.add(namui::path(path, paint));
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

        println!("item.rotation: {:?}", item.rotation.as_degrees());
        println!("event.global_xy: {:?}", event.global_xy);
        println!("event.local_xy: {:?}", event.local_xy());
        println!("event.is_local_xy_in: {:?}", event.is_local_xy_in());
        if event.is_local_xy_in() {
            game_state::on_game_event(GameEvent::ItemMouseDown {
                id: item.id,
                mouse_global_xy: event.global_xy,
            });
        }
    });
}

fn render_grid_storage_cell(ctx: ComposeCtx) {
    let rect = GRID_STORAGE_CELL_RECT;
    let path = Path::new()
        .move_to(rect.right(), rect.top())
        .line_to(rect.left(), rect.top())
        .line_to(rect.left(), rect.bottom())
        .line_to(rect.right(), rect.bottom());
    let paint = Paint::new(Color::RED).set_style(PaintStyle::Stroke);
    ctx.add(namui::path(path, paint));
}
