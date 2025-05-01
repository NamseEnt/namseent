mod physics_world;
mod render;

use crate::*;
use physics_world::*;
use rapier2d::{parry::query::ShapeCastOptions, prelude::*};
use std::sync::atomic::{AtomicU64, Ordering};

/*
고객이 와서 주문하면 그 주문한 아이템이 있는 박스 셀을 열고, 아이템을 손으로 꺼낸다.
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

fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(|game_state| {
        f(game_state);
    });
}

impl GameState {
    pub fn new() -> Self {
        let mut physics_world = PhysicsWorld::new(Xy::new(0.px(), 9.8.px()));
        Self {
            view: GameView::BoothCustomer(BoothCustomerView {
                grid_storage_cell_popup: None,
            }),
            hands: Hands::new(&mut physics_world),
            physics_world,
            grid_storage_box: GridStorageBox::new(),
            dragging: None,
        }
    }
    pub fn tick(&mut self) {
        self.handle_dragging();
        self.update_gravity_by_place();
        self.physics_world.tick();
        // self.update_physics_items();
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
        let original_linear_damping = rigid_body.linear_damping();

        rigid_body.set_vels(Default::default(), true);
        rigid_body.set_gravity_scale(0., true);
        rigid_body.set_linear_damping(10.);

        self.dragging = Some(Dragging {
            item_id,
            last_mouse_xy: mouse_xy,
            anchor: mouse_xy.to_vector() - rigid_body.translation(),
            rigid_body_handle,
            original_linear_damping,
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
        rigid_body.set_gravity_scale(1., false);
        rigid_body.set_linear_damping(dragging.original_linear_damping);
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

        let current_pos_vec = rigid_body.translation();
        let target_pos_vec = mouse_and_anchor;

        let delta_pos = target_pos_vec - current_pos_vec;
        let distance = delta_pos.magnitude();

        const MAX_DRAG_SPEED: f32 = 75.0;
        const SPEED_FACTOR: f32 = 8.0;
        let desired_speed = (distance * SPEED_FACTOR).min(MAX_DRAG_SPEED);

        const MIN_DISTANCE_THRESHOLD: f32 = 0.01;
        let target_velocity = if distance < MIN_DISTANCE_THRESHOLD {
            Vector::zeros()
        } else {
            let direction = delta_pos.normalize();
            direction * desired_speed
        };

        self.physics_world
            .rigid_body_mut(dragging.rigid_body_handle)
            .unwrap()
            .set_linvel(target_velocity, true);
    }

    fn update_physics_items(&mut self) {
        // 이거 좀 더 고민해보고 싶음. 뷰마다 하는것이 나을지, 아니면 하나의 큰 맵을 만들고 거기서 참조하도록 하는게 나을지.
        // match &mut self.view {
        //     GameView::BoothCustomer => todo!(),
        //     GameView::GridStorageBox {
        //         hands,
        //         xy,
        //         items,
        //         physics_cell,
        //     } => {
        //         for (_, rigid_body) in self.physics_world.rigid_body_set.iter() {
        //             let id = rigid_body.user_data;

        //             let Some(item) = items.get_mut(&id) else {
        //                 continue;
        //             };

        //             let translation = rigid_body.translation();
        //             item.center = Xy::new(translation.x.px(), translation.y.px())
        //                 * PHYSICS_WORLD_MAGNIFICATION;
        //             item.rotation = rigid_body.rotation().angle().rad();
        //         }
        //     }
        //     GameView::CustomerBooth => todo!(),
        //     GameView::BoothStock => todo!(),
        //     GameView::BoothFloor => todo!(),
        // }
    }

    fn update_gravity_by_place(&mut self) {
        for (rigid_body_handle, intersection) in self
            .physics_world
            .query_dynamic_rigid_body_intersection_mut(self.hands.collider_handle)
        {
            let rigid_body = self
                .physics_world
                .rigid_body_mut(rigid_body_handle)
                .unwrap();

            if self.dragging.as_ref().map(|d| d.item_id) == Some(rigid_body.user_data) {
                continue;
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

    fn open_grid_storage_cell_popup(&mut self, cell_xy: Xy<usize>) {
        let GameView::BoothCustomer(BoothCustomerView {
            grid_storage_cell_popup,
        }) = &mut self.view
        else {
            return;
        };
        *grid_storage_cell_popup = Some(PhysicsGridStorageCell::new(&mut self.physics_world));
    }

    fn spawn_item_on_hands(&mut self) {
        self.hands.items.push(PhysicsItem::new(
            &mut self.physics_world,
            ItemKind::Sticker,
            HANDS_RECT.center(),
        ));
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
    collider_handle: ColliderHandle,
    rigid_body_handle: RigidBodyHandle,
}

impl Hands {
    fn new(physics_world: &mut PhysicsWorld) -> Self {
        let rect = HANDS_RECT.map(|v| v.as_f32()) / PHYSICS_WORLD_MAGNIFICATION;

        let rigid_body =
            RigidBodyBuilder::fixed().translation(vector![rect.center().x, rect.center().y,]);
        let rigid_body_handle = physics_world.insert_rigid_body(rigid_body);

        let collider = ColliderBuilder::cuboid(rect.width() / 2., rect.height() / 2.).sensor(true);
        let collider_handle = physics_world.insert_collider(collider, rigid_body_handle);

        Self {
            items: vec![],
            collider_handle,
            rigid_body_handle,
        }
    }
}

struct BoothCustomerView {
    grid_storage_cell_popup: Option<PhysicsGridStorageCell>,
}

enum GameView {
    BoothCustomer(BoothCustomerView),
    // GridStorageBox {
    //     hands: PhysicsHands,
    //     xy: Xy<usize>,
    //     items: BTreeMap<u128, PhysicsItem>,
    //     physics_cell: PhysicsGridStorageCell,
    // },
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
        let rigid_body_handle = physics_world.insert_rigid_body(rigid_body);

        let rect = GRID_STORAGE_CELL_RECT.map(|v| v.as_f32()) / PHYSICS_WORLD_MAGNIFICATION;
        let thickness = GRID_STORAGE_CELL_THICKNESS.as_f32() / PHYSICS_WORLD_MAGNIFICATION;

        let top_center = vector![rect.center().x, rect.top()];
        let left_center = vector![rect.left(), rect.center().y];
        let bottom_center = vector![rect.center().x, rect.bottom()];

        let top_collider = ColliderBuilder::cuboid((rect.width() + thickness) / 2., thickness / 2.)
            .translation(top_center);
        physics_world.insert_collider(top_collider, rigid_body_handle);
        let left_collider =
            ColliderBuilder::cuboid(thickness / 2., (rect.height() + thickness) / 2.)
                .translation(left_center);
        physics_world.insert_collider(left_collider, rigid_body_handle);
        let bottom_collider =
            ColliderBuilder::cuboid((rect.width() + thickness) / 2., thickness / 2.)
                .translation(bottom_center);
        physics_world.insert_collider(bottom_collider, rigid_body_handle);

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
        let rigid_body_handle = physics_world.insert_rigid_body(rigid_body);

        let wh = item_kind
            .wh()
            .map(|v| v.as_f32() / PHYSICS_WORLD_MAGNIFICATION);
        let collider = ColliderBuilder::cuboid(wh.width / 2.0, wh.height / 2.0);
        physics_world.insert_collider(collider, rigid_body_handle);

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

#[derive(Debug)]
struct Dragging {
    item_id: u128,
    last_mouse_xy: Xy<Px>,
    anchor: Vector<f32>,
    rigid_body_handle: RigidBodyHandle,
    original_linear_damping: f32,
}
