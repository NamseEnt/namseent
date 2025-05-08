use super::*;
use crate::*;

#[derive(Debug)]
pub struct PhysicsItem {
    pub id: u128,
    pub item_kind: ItemKind,
    pub center: Xy<Px>,
    pub rotation: Angle,
    pub location: ItemLocation,
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}
impl PhysicsItem {
    pub fn new(
        physics_world: &mut PhysicsWorld,
        item_kind: ItemKind,
        center: Xy<Px>,
        location: ItemLocation,
    ) -> Self {
        let id = {
            static ID: AtomicU64 = AtomicU64::new(1024);
            ID.fetch_add(1, Ordering::Relaxed) as u128
        };

        let rigid_body = RigidBodyBuilder::dynamic()
            .user_data(id)
            .translation(
                vector![center.x.as_f32(), center.y.as_f32()] / PHYSICS_WORLD_MAGNIFICATION,
            )
            .ccd_enabled(true);
        let rigid_body_handle = physics_world.insert_rigid_body(rigid_body);

        let wh = item_kind
            .wh()
            .map(|v| v.as_f32() / PHYSICS_WORLD_MAGNIFICATION);
        let collider = ColliderBuilder::cuboid(wh.width / 2.0, wh.height / 2.0);
        let collider_handle = physics_world.insert_collider(collider, rigid_body_handle);

        Self {
            id,
            item_kind,
            center,
            rotation: 0.deg(),
            location,
            rigid_body_handle,
            collider_handle,
        }
    }
}

impl Component for &'_ PhysicsItem {
    fn render(self, ctx: &RenderCtx) {
        let game_state = use_game_state(ctx);

        if !game_state.item_enabled(self.id) {
            return;
        }

        let rt = namui::translate(
            self.center.x,
            self.center.y,
            namui::rotate(
                self.rotation,
                namui::translate(
                    -self.item_kind.wh().as_xy().x / 2,
                    -self.item_kind.wh().as_xy().y / 2,
                    match self.item_kind {
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

        ctx.add_with_key(self.id, rt).attach_event(|event| {
            let Event::MouseDown { event } = event else {
                return;
            };

            if event.is_local_xy_in() {
                game_state::on_game_event(GameEvent::ItemMouseDown {
                    id: self.id,
                    mouse_global_xy: event.global_xy,
                });
            }
        });
    }
}
