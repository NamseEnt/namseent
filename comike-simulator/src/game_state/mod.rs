mod render;

use crate::*;
use rapier2d::{na::Point2, prelude::*};
use std::sync::atomic::{AtomicU64, Ordering};

/*
우측 상단 = 손
드래그가 아닌 상태로 바닥에 떨어지면 booth floor에 아이템이 떨어지는 것으로.
grid storage box와 hands 사이에 아이템을 주고 받는 것을 먼저 해보자.
*/

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

#[derive(Debug, Clone, Copy)]
struct Dragging {
    item_id: u128,
    mouse_anchor_xy: Xy<Px>,
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
            },
            physics_world,
            grid_storage_box: GridStorageBox::new(),
            hands: Hands::new(),
            dragging: None,
        }
    }
    pub fn tick(&mut self) {
        self.physics_world.tick();
        self.update_physics_items();
        // self.move_item_to_hands();

        // match self.view {
        //     GameView::BoothCustomer => todo!(),
        //     GameView::GridStorageBox { xy } => {
        //         let cell = &mut self.grid_storage_box.cells[xy.x][xy.y];
        //     }
        //     GameView::CustomerBooth => todo!(),
        //     GameView::BoothStock => todo!(),
        //     GameView::BoothFloor => todo!(),
        // }
    }

    pub fn on_namui_event(&mut self, event: RawEvent) {
        match event {
            RawEvent::MouseUp { event: _ } => self.stop_drag_item(),
            RawEvent::MouseMove { event } => {
                let Some(dragging) = self.dragging else {
                    return;
                };

                let Some(rigid_body) = self.physics_world.find_rigid_body_mut(dragging.item_id)
                else {
                    println!(
                        "cannot find rigid body for dragging item id: {}",
                        dragging.item_id
                    );
                    return;
                };
                let mouse_diff_xy = event.xy - dragging.mouse_anchor_xy;
                let translation = rigid_body.translation();

                rigid_body.set_next_kinematic_translation(
                    translation + vector![mouse_diff_xy.x.as_f32(), mouse_diff_xy.y.as_f32(),],
                );
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

    pub fn put_item_in_grid_storage_box(&mut self, item: ItemKind, box_xy: Xy<usize>) {
        let cell = &mut self.grid_storage_box.cells[box_xy.y][box_xy.x];
    }

    fn start_drag_item(&mut self, item_id: u128, mouse_anchor_xy: Xy<Px>) {
        self.stop_drag_item();
        self.dragging = Some(Dragging {
            item_id,
            mouse_anchor_xy,
        });
        let Some(rigid_body) = self.physics_world.find_rigid_body_mut(item_id) else {
            println!("cannot find rigid body for dragging item id: {}", item_id);
            return;
        };

        rigid_body.set_body_type(RigidBodyType::KinematicPositionBased, true);

        println!(
            "start drag item: {} at mouse anchor: {:?}",
            item_id, mouse_anchor_xy
        );
    }

    fn stop_drag_item(&mut self) {
        let Some(dragging) = self.dragging.take() else {
            return;
        };
        let Some(rigid_body) = self.physics_world.find_rigid_body_mut(dragging.item_id) else {
            println!(
                "cannot find rigid body for dragging item id: {}",
                dragging.item_id
            );
            return;
        };

        rigid_body.set_body_type(RigidBodyType::Dynamic, true);

        println!("stop drag item: {}", dragging.item_id);
    }

    fn update_physics_items(&mut self) {
        match &mut self.view {
            GameView::BoothCustomer => todo!(),
            GameView::GridStorageBox { hands, xy, items } => {
                for (_, rigid_body) in self.physics_world.rigid_body_set.iter() {
                    let id = rigid_body.user_data;
                    let Some(item) = items.get_mut(&id) else {
                        continue;
                    };

                    let translation = rigid_body.translation();
                    item.xy = Xy::new(translation.x.px(), translation.y.px());
                }
            }
            GameView::CustomerBooth => todo!(),
            GameView::BoothStock => todo!(),
            GameView::BoothFloor => todo!(),
        }
    }
    fn switch_view(&mut self, view: GameView) {
        // let prev_view = &self.view;
        // match prev_view {
        //     GameView::BoothCustomer => todo!(),
        //     GameView::GridStorageBox { xy, hands } => todo!(),
        //     GameView::CustomerBooth => todo!(),
        //     GameView::BoothStock => todo!(),
        //     GameView::BoothFloor => todo!(),
        // }

        // self.view = view;
    }

    // fn move_item_to_hands(&mut self) {
    //     for collider_handle in self
    //         .physics_world
    //         .intersection_exact_collider(self.hands.collider_handle)
    //     {
    //         let item_id = collider_handle.user_data;
    //         if self.dragging.is_some_and(|d| d.item_id == item_id) {
    //             continue;
    //         }

    //         if let GameView::GridStorageBox { xy } = self.view {
    //             let cell = &mut self.grid_storage_box.cells[xy.x][xy.y];
    //             let Some(index) = cell
    //                 .items
    //                 .iter()
    //                 .enumerate()
    //                 .find_map(|(index, item)| (item.id == item_id).then_some(index))
    //             else {
    //                 continue;
    //             };
    //             let item = cell.items.swap_remove(index);
    //             self.hands.items.push(item);
    //         }
    //     }
    // }
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
        let rigid_body = RigidBodyBuilder::fixed();
        let rigid_body_handle = physics_world.rigid_body_set.insert(rigid_body);

        let vertices = vec![
            Point2::new(HANDS_RECT.right().as_f32(), HANDS_RECT.top().as_f32()),
            Point2::new(HANDS_RECT.left().as_f32(), HANDS_RECT.top().as_f32()),
            Point2::new(HANDS_RECT.left().as_f32(), HANDS_RECT.bottom().as_f32()),
            Point2::new(HANDS_RECT.right().as_f32(), HANDS_RECT.bottom().as_f32()),
        ];
        let collider = ColliderBuilder::polyline(vertices, None);
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

struct PhysicsItem {
    id: u128,
    item_kind: ItemKind,
    xy: Xy<Px>,
    rotation: Angle,
}
impl PhysicsItem {
    fn new(physics_world: &mut PhysicsWorld, item_kind: ItemKind, xy: Xy<Px>) -> Self {
        let id = {
            static ID: AtomicU64 = AtomicU64::new(1024);
            ID.fetch_add(1, Ordering::Relaxed) as u128
        };

        {
            let rigid_body = RigidBodyBuilder::dynamic()
                .user_data(id)
                .translation(vector![xy.x.as_f32(), xy.y.as_f32()]);
            let rigid_body_handle = physics_world.rigid_body_set.insert(rigid_body);

            let wh = item_kind.wh().map(|v| v.as_f32());
            let collider = ColliderBuilder::cuboid(wh.width / 2.0, wh.height / 2.0);
            let collider_handle = physics_world.collider_set.insert_with_parent(
                collider,
                rigid_body_handle,
                &mut physics_world.rigid_body_set,
            );
        }

        Self {
            id,
            item_kind,
            xy,
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

fn test(rect: Rect<Px>) {
    let mut rigid_body_set = RigidBodySet::new();

    let rigid_body = RigidBodyBuilder::fixed();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let mut collider_set = ColliderSet::new();

    let vertices = vec![
        Point2::new(rect.right().as_f32(), rect.top().as_f32()),
        Point2::new(rect.left().as_f32(), rect.top().as_f32()),
        Point2::new(rect.left().as_f32(), rect.bottom().as_f32()),
        Point2::new(rect.right().as_f32(), rect.bottom().as_f32()),
    ];
    let collider = ColliderBuilder::polyline(vertices, None);
    let collider_handle =
        collider_set.insert_with_parent(collider, rigid_body_handle, &mut rigid_body_set);
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
            .map(|(_, collider_handle, intersecting)| (collider_handle, intersecting))
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

    pub fn find_rigid_body_mut(&mut self, item_id: u128) -> Option<&mut RigidBody> {
        self.rigid_body_set
            .iter_mut()
            .find_map(|(_handle, rigid_body)| {
                (rigid_body.user_data == item_id).then_some(rigid_body)
            })
    }
}
