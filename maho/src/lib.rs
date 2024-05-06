use namui::*;
use rapier2d::prelude::*;
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{mpsc::Sender, OnceLock},
};

pub fn main() {
    namui::start(|| Game)
}

pub struct SpawnFireball {
    xy: Xy<Px>,
    direction_vector: Xy<Px>,
}

#[component]
pub struct Fireball<'a> {
    rigid_body: &'a RigidBody,
}

impl Component for Fireball<'_> {
    fn render(self, ctx: &RenderCtx) {
        todo!()
    }
}

struct StartSet<'a> {
    collider_set: &'a mut ColliderSet,
    rigid_body_set: &'a mut RigidBodySet,
}

trait EntityLifeCycle {
    fn start(&self, start_set: &mut StartSet);
}

struct Entity {
    entity_id: EntityId,
    rigid_body_handle: RigidBodyHandle,
    render: fn(&RenderCtx, &RigidBody),
}

fn init_entity(
    start_set: &mut StartSet,
    start_xy: Xy<Px>,
    init_collider: impl FnOnce() -> Vec<Collider>,
    entity_id: EntityId,
    render: fn(&RenderCtx, &RigidBody),
) -> Entity {
    let rigid_body_handle = init_rigid_body(start_set.rigid_body_set, start_xy);
    let colliders = init_collider();
    for collider in colliders {
        start_set.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            start_set.rigid_body_set,
        );
    }

    Entity {
        entity_id,
        rigid_body_handle,
        render,
    }
}

struct StartEntity {
    xy: Xy<Px>,
    init_collider: fn() -> Vec<Collider>,
    render: fn(&RenderCtx, &RigidBody),
}

fn init_fireball_collider() -> Vec<Collider> {
    vec![ColliderBuilder::ball(0.5).restitution(0.9).build()]
}

fn render_fireball(ctx: &RenderCtx, rigid_body: &RigidBody) {
    let xy = rigid_body.translation();
    ctx.add(namui::path(
        Path::new().add_arc(
            Rect::from_xy_wh(
                Xy::new(xy.x.px() * 10.0, xy.y.px() * 10.0),
                Wh::new(20.px(), 20.px()),
            ),
            0.deg(),
            360.deg(),
        ),
        Paint::new(Color::RED),
    ));
}

fn init_rigid_body(rigid_body_set: &mut RigidBodySet, xy: Xy<Px>) -> RigidBodyHandle {
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![xy.x.as_f32(), xy.y.as_f32()])
        .build();
    rigid_body_set.insert(rigid_body)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct EntityId {}
impl EntityId {
    fn next() -> EntityId {
        todo!()
    }
}

struct KillEntity {
    entity_id: EntityId,
}

static START_ENTITY_TX: OnceLock<Sender<(StartEntity, EntityId)>> = OnceLock::new();
static KILL_ENTITY_TX: OnceLock<Sender<KillEntity>> = OnceLock::new();

fn send_start_entity(start_entity: StartEntity) -> EntityId {
    let tx = START_ENTITY_TX.get().unwrap();
    let entity_id = EntityId::next();
    tx.send((start_entity, entity_id)).unwrap();
    entity_id
}

fn run() {
    let (start_entity_tx, start_entity_rx) = std::sync::mpsc::channel();
    START_ENTITY_TX.set(start_entity_tx).unwrap();
    let (kill_entity_tx, kill_entity_rx) = std::sync::mpsc::channel();
    KILL_ENTITY_TX.set(kill_entity_tx).unwrap();

    let mut collider_set = ColliderSet::new();
    let mut rigid_body_set = RigidBodySet::new();

    send_start_entity(StartEntity {
        xy: Xy::new(0.px(), 50.px()),
        init_collider: init_fireball_collider,
        render: render_fireball,
    });

    let mut entities = HashMap::new();

    while let Ok((start_entity, entity_id)) = start_entity_rx.try_recv() {
        let entity = init_entity(
            &mut StartSet {
                collider_set: &mut collider_set,
                rigid_body_set: &mut rigid_body_set,
            },
            start_entity.xy,
            start_entity.init_collider,
            entity_id,
            start_entity.render,
        );

        entities.insert(entity_id, entity);
    }

    while let Ok(KillEntity { entity_id }) = kill_entity_rx.try_recv() {
        entities.remove(&entity_id);
    }

    let ctx = get_ctx();

    for entity in entities.values() {
        (entity.render)(ctx, &rigid_body_set[entity.rigid_body_handle]);
    }
}

fn get_ctx() -> &'static RenderCtx<'static, 'static> {
    todo!()
}

#[component]
pub struct Game;

impl Component for Game {
    fn render(self, ctx: &RenderCtx) {
        let collider_set = ctx.memo(|| RefCell::new(ColliderSet::new()));
        let rigid_body_set = ctx.memo(|| RefCell::new(RigidBodySet::new()));
        let ball_body_handle = ctx.memo(|| {
            /* Create the ground. */
            let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
            collider_set.borrow_mut().insert(collider);

            /* Create the bouncing ball. */
            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![0.0, 50.0])
                .build();

            let ball_body_handle = rigid_body_set.borrow_mut().insert(rigid_body);

            let collider = ColliderBuilder::ball(0.5).restitution(0.9).build();
            collider_set.borrow_mut().insert_with_parent(
                collider,
                ball_body_handle,
                &mut rigid_body_set.borrow_mut(),
            );

            ball_body_handle
        });

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = ctx.memo(|| RefCell::new(PhysicsPipeline::new()));
        let island_manager = ctx.memo(|| RefCell::new(IslandManager::new()));
        let broad_phase = ctx.memo(|| RefCell::new(BroadPhase::new()));
        let narrow_phase = ctx.memo(|| RefCell::new(NarrowPhase::new()));
        let impulse_joint_set = ctx.memo(|| RefCell::new(ImpulseJointSet::new()));
        let multibody_joint_set = ctx.memo(|| RefCell::new(MultibodyJointSet::new()));
        let ccd_solver = ctx.memo(|| RefCell::new(CCDSolver::new()));
        let query_pipeline = ctx.memo(|| RefCell::new(QueryPipeline::new()));
        let physics_hooks = ();
        let event_handler = ();

        physics_pipeline.borrow_mut().step(
            &gravity,
            &integration_parameters,
            &mut island_manager.borrow_mut(),
            &mut broad_phase.borrow_mut(),
            &mut narrow_phase.borrow_mut(),
            &mut rigid_body_set.borrow_mut(),
            &mut collider_set.borrow_mut(),
            &mut impulse_joint_set.borrow_mut(),
            &mut multibody_joint_set.borrow_mut(),
            &mut ccd_solver.borrow_mut(),
            Some(&mut query_pipeline.borrow_mut()),
            &physics_hooks,
            &event_handler,
        );

        let ball_body = &rigid_body_set.borrow_mut()[ball_body_handle.clone_inner()];

        ctx.add(namui::path(
            Path::new().add_arc(
                Rect::from_xy_wh(
                    Xy::new(
                        ball_body.translation().x.px() * 10.0,
                        ball_body.translation().y.px() * 10.0,
                    ),
                    Wh::new(20.px(), 20.px()),
                ),
                0.deg(),
                360.deg(),
            ),
            Paint::new(Color::RED),
        ));

        ctx.add(Fireball {
            rigid_body: ball_body,
        });
    }
}
