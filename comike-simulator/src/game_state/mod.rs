mod render;

use crate::*;
use rapier2d::{parry::query::ShapeCastOptions, prelude::*};
use std::sync::atomic::{AtomicU64, Ordering};

const HANDS_RECT: Rect<Px> = Rect::Xywh {
    x: px(800.),
    y: px(0.),
    width: px(300.),
    height: px(300.),
};

const GRID_STORAGE_CELL_RECT: Rect<Px> = Rect::Xywh {
    x: px(400.),
    y: px(500.),
    width: px(300.),
    height: px(300.),
};
const GRID_STORAGE_CELL_THICKNESS: Px = px(10.);

const PHYSICS_WORLD_MAGNIFICATION: f32 = 100.;

pub static GAME_STATE_ATOM: Atom<GameState> = Atom::uninitialized();

pub enum GameEvent {
    ItemMouseDown { id: u128, mouse_global_xy: Xy<Px> },
}

pub struct GameState {
    view: GameView,
    physics_world: PhysicsWorld,
    grid_storage_box: GridStorageBox,
    hands: Hands,
    dragging: Option<Dragging>,
}

#[derive(Debug)]
struct Dragging {
    item_id: u128,
    last_mouse_xy: Xy<Px>,
    anchor: Vector<f32>,
    rigid_body_handle: RigidBodyHandle,
}

impl GameState {
    pub fn new() -> Self {
        let mut physics_world = PhysicsWorld::new(Xy::new(0.px(), 9.8.px()));
        let item = PhysicsItem::new(&mut physics_world, ItemKind::Sticker, HANDS_RECT.center());

        Self {
            view: GameView::GridStorageBox {
                xy: Xy::new(0, 0),
                hands: PhysicsHands::new(&mut physics_world),
                items: [(item.id, item)].into_iter().collect(),
                physics_cell: PhysicsGridStorageCell::new(&mut physics_world),
            },
            physics_world,
            grid_storage_box: GridStorageBox::new(),
            hands: Hands::new(),
            dragging: None,
        }
    }
    pub fn tick(&mut self) {
        self.handle_dragging();
        self.update_gravity_by_place();
        self.physics_world.tick();
        self.update_physics_items();
    }

    pub fn on_namui_event(&mut self, event: RawEvent) {
        match event {
            RawEvent::MouseUp { event: _ } => self.stop_drag_item(),
            RawEvent::MouseMove { event } => {
                if let Some(dragging) = &mut self.dragging {
                    dragging.last_mouse_xy = event.xy;
                }
            }
            _ => {}
        }
    }

    fn on_game_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::ItemMouseDown {
                id,
                mouse_global_xy,
            } => {
                self.start_drag_item(id, mouse_global_xy);
            }
        }
    }

    fn start_drag_item(&mut self, item_id: u128, mouse_xy: Xy<Px>) {
        self.stop_drag_item();
        let Some((rigid_body_handle, rigid_body)) = self.physics_world.find_rigid_body_mut(item_id)
        else {
            println!("cannot find rigid body for dragging item id: {}", item_id);
            return;
        };
        rigid_body.wake_up(true);
        rigid_body.set_vels(Default::default(), false);
        rigid_body.set_angvel(0., false);
        rigid_body.lock_rotations(true, false);
        rigid_body.set_gravity_scale(0., false);
        rigid_body.set_body_type(RigidBodyType::KinematicPositionBased, false);

        self.dragging = Some(Dragging {
            item_id,
            last_mouse_xy: mouse_xy,
            anchor: mouse_xy.to_vector() - rigid_body.translation(),
            rigid_body_handle,
        });
    }

    fn stop_drag_item(&mut self) {
        let Some(dragging) = self.dragging.take() else {
            return;
        };
        let Some((_rigid_body_handle, rigid_body)) =
            self.physics_world.find_rigid_body_mut(dragging.item_id)
        else {
            println!(
                "cannot find rigid body for dragging item id: {}",
                dragging.item_id
            );
            return;
        };
        rigid_body.set_vels(Default::default(), false);
        rigid_body.set_angvel(0., false);
        rigid_body.lock_rotations(false, false);
        rigid_body.set_gravity_scale(1., false);
        rigid_body.set_body_type(RigidBodyType::Dynamic, false);
    }
    fn handle_dragging(&mut self) {
        let Some(dragging) = &mut self.dragging else {
            return;
        };
        let Some(rigid_body) = self.physics_world.rigid_body(dragging.rigid_body_handle) else {
            println!(
                "cannot find rigid body for dragging item id: {}",
                dragging.item_id
            );
            return;
        };
        let mouse_vec = dragging.last_mouse_xy.to_vector();
        let mouse_and_anchor = mouse_vec - dragging.anchor;
        let movement_vec = mouse_and_anchor - rigid_body.position().translation.vector;
        let mut min_hit_info: Option<(ShapeCastHit, Isometry<Real>)> = None;

        for collider_handle in rigid_body.colliders() {
            let collider = self
                .physics_world
                .collider_set
                .get(*collider_handle)
                .unwrap();

            let collider_position = collider.position();

            let Some((_, hit)) = self.physics_world.cast_shape(
                collider_position,
                &movement_vec,
                collider.shape(),
                ShapeCastOptions::with_max_time_of_impact(1.),
                QueryFilter::new()
                    .exclude_sensors()
                    .exclude_rigid_body(dragging.rigid_body_handle),
            ) else {
                continue;
            };
            if hit.time_of_impact < 0. {
                continue;
            }
            if let Some((min_hit, _)) = &min_hit_info {
                if hit.time_of_impact < min_hit.time_of_impact {
                    min_hit_info = Some((hit, *collider_position));
                }
            } else {
                min_hit_info = Some((hit, *collider_position));
            }
        }

        const EPSILON: f32 = 1e-5;

        let next_translation = if let Some((hit, collider_position)) = min_hit_info {
            let world_normal1 = collider_position.rotation * hit.normal1;
            let dot_product = movement_vec.dot(world_normal1.as_ref());

            let is_away_from_collider = dot_product > 0.0;
            if hit.time_of_impact < EPSILON && is_away_from_collider {
                mouse_and_anchor
            } else {
                rigid_body.translation() + movement_vec * (hit.time_of_impact - EPSILON).max(0.)
            }
        } else {
            mouse_and_anchor
        };

        self.physics_world
            .rigid_body_mut(dragging.rigid_body_handle)
            .unwrap()
            .set_next_kinematic_translation(next_translation);
    }

    fn update_physics_items(&mut self) {
        match &mut self.view {
            GameView::BoothCustomer => todo!(),
            GameView::GridStorageBox {
                hands,
                xy,
                items,
                physics_cell,
            } => {
                for (_, rigid_body) in self.physics_world.rigid_body_set.iter() {
                    let id = rigid_body.user_data;

                    let Some(item) = items.get_mut(&id) else {
                        continue;
                    };

                    let translation = rigid_body.translation();
                    item.center = Xy::new(translation.x.px(), translation.y.px())
                        * PHYSICS_WORLD_MAGNIFICATION;
                    item.rotation = rigid_body.rotation().angle().rad();
                }
            }
            GameView::CustomerBooth => todo!(),
            GameView::BoothStock => todo!(),
            GameView::BoothFloor => todo!(),
        }
    }

    fn update_gravity_by_place(&mut self) {
        let GameView::GridStorageBox { hands, .. } = &self.view else {
            return;
        };
        for (rigid_body_handle, intersection) in
            self.physics_world.query_intersection(hands.collider_handle)
        {
            let Some(rigid_body) = self.physics_world.rigid_body_mut(rigid_body_handle) else {
                continue;
            };
            if let Some(dragging) = &self.dragging {
                if dragging.item_id == rigid_body.user_data {
                    continue;
                }
            }

            if intersection && rigid_body.gravity_scale() >= 1.0 {
                rigid_body.set_gravity_scale(0., false);
                rigid_body.set_vels(Default::default(), false);
            } else if !intersection && rigid_body.gravity_scale() == 0.0 {
                rigid_body.set_gravity_scale(1., false);
                rigid_body.set_vels(Default::default(), true);
            }
        }
    }
}

pub fn on_game_event(event: GameEvent) {
    GAME_STATE_ATOM.mutate(|game_state| {
        game_state.on_game_event(event);
    });
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

struct Hands {
    items: Vec<PhysicsItem>,
}

struct PhysicsHands {
    collider_handle: ColliderHandle,
    rigid_body_handle: RigidBodyHandle,
}

impl PhysicsHands {
    fn new(physics_world: &mut PhysicsWorld) -> Self {
        let rect = HANDS_RECT.map(|v| v.as_f32()) / PHYSICS_WORLD_MAGNIFICATION;

        let rigid_body =
            RigidBodyBuilder::fixed().translation(vector![rect.center().x, rect.center().y,]);
        let rigid_body_handle = physics_world.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::cuboid(rect.width() / 2., rect.height() / 2.).sensor(true);
        let collider_handle = physics_world.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut physics_world.rigid_body_set,
        );

        Self {
            collider_handle,
            rigid_body_handle,
        }
    }
}

impl Hands {
    fn new() -> Self {
        Self { items: vec![] }
    }
}

enum GameView {
    BoothCustomer,
    GridStorageBox {
        hands: PhysicsHands,
        xy: Xy<usize>,
        items: BTreeMap<u128, PhysicsItem>,
        physics_cell: PhysicsGridStorageCell,
    },
    CustomerBooth,
    BoothStock,
    BoothFloor,
}

struct GridStorageBox {
    cells: [[GridStorageCell; 3]; 5],
}
impl GridStorageBox {
    fn new() -> Self {
        Self {
            cells: [
                [
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                ],
                [
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                ],
                [
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                ],
                [
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                ],
                [
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                    GridStorageCell::new(),
                ],
            ],
        }
    }
}

struct GridStorageCell {
    items: Vec<PhysicsItem>,
}
impl GridStorageCell {
    fn new() -> Self {
        Self { items: vec![] }
    }
}

struct PhysicsGridStorageCell {
    rigid_body_handle: RigidBodyHandle,
}
impl PhysicsGridStorageCell {
    fn new(physics_world: &mut PhysicsWorld) -> Self {
        let rigid_body = RigidBodyBuilder::fixed();
        let rigid_body_handle = physics_world.rigid_body_set.insert(rigid_body);

        let rect = GRID_STORAGE_CELL_RECT.map(|v| v.as_f32()) / PHYSICS_WORLD_MAGNIFICATION;
        let thickness = GRID_STORAGE_CELL_THICKNESS.as_f32() / PHYSICS_WORLD_MAGNIFICATION;

        let top_center = vector![rect.center().x, rect.top()];
        let left_center = vector![rect.left(), rect.center().y];
        let bottom_center = vector![rect.center().x, rect.bottom()];

        let top_collider = ColliderBuilder::cuboid((rect.width() + thickness) / 2., thickness / 2.)
            .translation(top_center);
        physics_world.collider_set.insert_with_parent(
            top_collider,
            rigid_body_handle,
            &mut physics_world.rigid_body_set,
        );
        let left_collider =
            ColliderBuilder::cuboid(thickness / 2., (rect.height() + thickness) / 2.)
                .translation(left_center);
        physics_world.collider_set.insert_with_parent(
            left_collider,
            rigid_body_handle,
            &mut physics_world.rigid_body_set,
        );
        let bottom_collider =
            ColliderBuilder::cuboid((rect.width() + thickness) / 2., thickness / 2.)
                .translation(bottom_center);
        physics_world.collider_set.insert_with_parent(
            bottom_collider,
            rigid_body_handle,
            &mut physics_world.rigid_body_set,
        );

        Self { rigid_body_handle }
    }
}

struct PhysicsItem {
    id: u128,
    item_kind: ItemKind,
    center: Xy<Px>,
    rotation: Angle,
}
impl PhysicsItem {
    fn new(physics_world: &mut PhysicsWorld, item_kind: ItemKind, center: Xy<Px>) -> Self {
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
        let rigid_body_handle = physics_world.rigid_body_set.insert(rigid_body);

        let wh = item_kind
            .wh()
            .map(|v| v.as_f32() / PHYSICS_WORLD_MAGNIFICATION);
        let collider = ColliderBuilder::cuboid(wh.width / 2.0, wh.height / 2.0);
        physics_world.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut physics_world.rigid_body_set,
        );

        Self {
            id,
            item_kind,
            center,
            rotation: 0.deg(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ItemKind {
    Sticker,
}

impl ItemKind {
    pub fn wh(&self) -> Wh<Px> {
        match self {
            ItemKind::Sticker => Wh::new(50.px(), 100.px()),
        }
    }
}

struct Position {
    xy: Xy<Px>,
    rotation: Angle,
}

struct PhysicsWorld {
    gravity: Xy<Px>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    positions: BTreeMap<u128, Position>,
}

impl PhysicsWorld {
    fn new(gravity: Xy<Px>) -> Self {
        Self {
            gravity,
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            positions: BTreeMap::new(),
        }
    }

    fn tick(&mut self) {
        self.physics_pipeline.step(
            &vector![self.gravity.x.as_f32(), self.gravity.y.as_f32()],
            &IntegrationParameters::default(),
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );

        for (_handle, rigid_body) in self.rigid_body_set.iter() {
            let position = rigid_body.position();
            self.positions.insert(
                rigid_body.user_data,
                Position {
                    xy: Xy::new(position.translation.x.px(), position.translation.y.px()),
                    rotation: position.rotation.re.rad(),
                },
            );
        }
    }

    pub fn insert_with_parent(
        &mut self,
        collider: impl Into<Collider>,
        rigid_body_handle: RigidBodyHandle,
    ) -> ColliderHandle {
        self.collider_set
            .insert_with_parent(collider, rigid_body_handle, &mut self.rigid_body_set)
    }

    /// last `bool` is indicating if the colliders are actually intersecting or not.
    pub fn intersection_pairs_with(
        &self,
        collider: ColliderHandle,
    ) -> impl Iterator<Item = (ColliderHandle, bool)> {
        self.narrow_phase
            .intersection_pairs_with(collider)
            .map(move |(a, b, intersecting)| (if a == collider { b } else { a }, intersecting))
    }

    pub fn intersection_pairs_with_exact(
        &self,
        collider: ColliderHandle,
    ) -> impl Iterator<Item = ColliderHandle> {
        self.intersection_pairs_with(collider)
            .filter_map(|(collider_handle, intersecting)| {
                if intersecting {
                    Some(collider_handle)
                } else {
                    None
                }
            })
    }

    pub fn intersection_exact_collider(
        &self,
        collider: ColliderHandle,
    ) -> impl Iterator<Item = &Collider> {
        self.intersection_pairs_with(collider)
            .filter_map(|(collider_handle, intersecting)| {
                if intersecting {
                    Some(self.collider_set.get(collider_handle).unwrap())
                } else {
                    None
                }
            })
    }

    pub fn intersect_exact_rigid_body_handles(
        &self,
        collider: ColliderHandle,
    ) -> Vec<RigidBodyHandle> {
        self.intersection_pairs_with(collider)
            .filter_map(|(collider_handle, intersecting)| {
                if intersecting {
                    let collider = self.collider_set.get(collider_handle).unwrap();
                    collider.parent()
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_rigid_body_mut(
        &mut self,
        item_id: u128,
    ) -> Option<(RigidBodyHandle, &mut RigidBody)> {
        self.rigid_body_set
            .iter_mut()
            .find_map(|(handle, rigid_body)| {
                (rigid_body.user_data == item_id).then_some((handle, rigid_body))
            })
    }

    pub fn rigid_body(&self, rigid_body_handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.rigid_body_set.get(rigid_body_handle)
    }

    pub fn rigid_body_mut(&mut self, rigid_body_handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        self.rigid_body_set.get_mut(rigid_body_handle)
    }

    pub fn query_intersection(
        &self,
        collider_handle: ColliderHandle,
    ) -> Vec<(RigidBodyHandle, bool)> {
        self.collider_set
            .iter()
            .filter(|(handle, collider)| handle != &collider_handle && collider.parent().is_some())
            .map(|(handle, collider)| {
                (
                    collider.parent().unwrap(),
                    self.narrow_phase
                        .intersection_pair(collider_handle, handle)
                        .unwrap_or_default(),
                )
            })
            .collect()
    }

    fn find_rigid_body(&self, item_id: u128) -> Option<(RigidBodyHandle, &RigidBody)> {
        self.rigid_body_set.iter().find_map(|(handle, rigid_body)| {
            (rigid_body.user_data == item_id).then_some((handle, rigid_body))
        })
    }

    fn cast_shape(
        &self,
        shape_position: &Isometry<Real>,
        desired_movement_vec: &Vector<f32>,
        shape: &dyn Shape,
        cast_options: ShapeCastOptions,
        query_filter: QueryFilter,
    ) -> Option<(ColliderHandle, ShapeCastHit)> {
        self.query_pipeline.cast_shape(
            &self.rigid_body_set,
            &self.collider_set,
            shape_position,
            desired_movement_vec,
            shape,
            cast_options,
            query_filter,
        )
    }
}

trait NaHelper {
    fn to_vector(&self) -> Vector<f32>;
    fn to_point(&self) -> Point<f32>;
}

impl NaHelper for Xy<Px> {
    fn to_vector(&self) -> Vector<f32> {
        vector![self.x.as_f32(), self.y.as_f32()] / PHYSICS_WORLD_MAGNIFICATION
    }
    fn to_point(&self) -> Point<f32> {
        point![self.x.as_f32(), self.y.as_f32()] / PHYSICS_WORLD_MAGNIFICATION
    }
}
