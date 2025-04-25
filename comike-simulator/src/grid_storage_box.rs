use crate::*;
use rapier2d::{na::Point2, prelude::*};

/*
우측 상단 = 손
드래그가 아닌 상태로 바닥에 떨어지면 booth floor에 아이템이 떨어지는 것으로.
grid storage box와 hands 사이에 아이템을 주고 받는 것을 먼저 해보자.
*/

const HANDS_RECT: Rect<f32> = Rect::Xywh {
    x: 800.,
    y: 0.,
    width: 300.,
    height: 300.,
};

pub enum GameEvent {
    ItemMouseDown { id: u128 },
}

pub struct GameState {
    view: GameView,
    physics_world: PhysicsWorld,
    grid_storage_box: GridStorageBox,
    hands: Hands,
    dragging_item_id: Option<u128>,
}

impl GameState {
    pub fn new() -> Self {
        todo!()
    }
    pub fn tick(&mut self) {
        self.physics_world.tick();
        self.move_item_to_hands();

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

    pub fn on_namui_event(&mut self, event: Event) {
        match event {
            Event::MouseUp { event: _ } => self.dragging_item_id = None,
            Event::MouseMove { event } => {
                let Some(item_id) = self.dragging_item_id else {
                    return;
                };

                let Some(rigid_body) = self.find_rigid_body_mut(item_id) else {
                    unreachable!("cannot find rigid body for dragging item id: {}", item_id)
                };
                rigid_body.set_next_kinematic_translation(vector![
                    event.global_xy.x.as_f32(),
                    event.global_xy.y.as_f32(),
                ]);
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn on_game_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::ItemMouseDown { id } => self.dragging_item_id = Some(id),
        }
    }

    fn switch_view(&mut self, view: GameView) {
        let prev_view = &self.view;
        match prev_view {
            GameView::BoothCustomer => todo!(),
            GameView::GridStorageBox { xy } => todo!(),
            GameView::CustomerBooth => todo!(),
            GameView::BoothStock => todo!(),
            GameView::BoothFloor => todo!(),
        }

        self.view = view;
    }

    fn move_item_to_hands(&mut self) {
        for collider_handle in self
            .physics_world
            .intersection_exact_collider(self.hands.collider_handle)
        {
            let item_id = collider_handle.user_data;
            if self.dragging_item_id == Some(item_id) {
                continue;
            }

            if let GameView::GridStorageBox { xy } = self.view {
                let cell = &mut self.grid_storage_box.cells[xy.x][xy.y];
                let Some(index) = cell
                    .items
                    .iter()
                    .enumerate()
                    .find_map(|(index, item)| (item.id == item_id).then_some(index))
                else {
                    continue;
                };
                let item = cell.items.swap_remove(index);
                self.hands.items.push(item);
            }
        }
    }

    fn find_rigid_body_mut(&mut self, item_id: u128) -> Option<&mut RigidBody> {
        self.physics_world
            .rigid_body_set
            .iter_mut()
            .find_map(|(_handle, rigid_body)| {
                (rigid_body.user_data == item_id).then_some(rigid_body)
            })
    }
}

struct Hands {
    items: Vec<PhysicsItem>,
    collider_handle: ColliderHandle,
}

enum GameView {
    BoothCustomer,
    GridStorageBox { xy: Xy<usize> },
    CustomerBooth,
    BoothStock,
    BoothFloor,
}

struct GridStorageBox {
    cells: [[GridStorageCell; 3]; 5],
}

struct GridStorageCell {
    items: Vec<PhysicsItem>,
}

struct PhysicsItem {
    id: u128,
    xy: Xy<Px>,
    rotation: Angle,
    item_kind: ItemKind,
}

enum ItemKind {}

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
}
