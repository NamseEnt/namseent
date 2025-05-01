use std::collections::HashMap;

use super::*;
use crate::*;

pub struct PhysicsWorld {
    gravity: Vector<f32>,
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
    pub fn new(gravity: Vector<f32>) -> Self {
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

    pub fn tick(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
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

    pub fn query_dynamic_rigid_body_intersection_mut(
        &mut self,
        collider_handle: ColliderHandle,
    ) -> impl Iterator<Item = (RigidBodyHandle, bool)> + use<> {
        let mut intersection_map = HashMap::new();

        for (handle, collider) in self.collider_set.iter_enabled() {
            if handle == collider_handle {
                continue;
            }
            let Some(parent) = collider.parent() else {
                continue;
            };

            let intersection = self
                .narrow_phase
                .intersection_pair(collider_handle, handle)
                .unwrap_or_default();
            let entry = intersection_map.entry(parent).or_insert(false);
            *entry = *entry || intersection;
        }

        intersection_map.into_iter()
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

    pub(crate) fn insert_rigid_body(&mut self, rigid_body: RigidBodyBuilder) -> RigidBodyHandle {
        self.rigid_body_set.insert(rigid_body)
    }

    pub(crate) fn insert_collider(
        &mut self,
        collider: impl Into<Collider>,
        rigid_body_handle: RigidBodyHandle,
    ) -> ColliderHandle {
        self.collider_set
            .insert_with_parent(collider, rigid_body_handle, &mut self.rigid_body_set)
    }

    pub(crate) fn rigid_body_iter(&self) -> impl Iterator<Item = (RigidBodyHandle, &RigidBody)> {
        self.rigid_body_set.iter()
    }
}

impl Component for &'_ PhysicsWorld {
    fn render(self, ctx: &RenderCtx) {
        let color = Color::from_f01(0., 1., 0., 1.);
        let fill_paint = Paint::new(color).set_style(PaintStyle::Fill);
        let stroke_paint = Paint::new(color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(1.px());

        for (_joint_handle, joint) in self.impulse_joint_set.iter() {
            let Some((rigid_body_1, rigid_body_2)) = self
                .rigid_body_set
                .get(joint.body1)
                .zip(self.rigid_body_set.get(joint.body2))
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

        for (_, rigid_body) in self.rigid_body_set.iter() {
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

        for (_, collider) in self.collider_set.iter() {
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
