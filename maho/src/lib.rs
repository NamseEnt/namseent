use namui::*;
use rapier2d::prelude::*;
use std::cell::RefCell;

pub fn main() {
    namui::start(|| Game)
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
                .translation(vector![0.0, 10.0])
                .build();
            let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
            let ball_body_handle = rigid_body_set.borrow_mut().insert(rigid_body);
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
        println!("Ball altitude: {}", ball_body.translation().y);

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
    }
}
