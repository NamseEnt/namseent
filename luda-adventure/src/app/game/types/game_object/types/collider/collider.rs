use super::RigidBody;
use crate::app::game::Tile;
use namui::prelude::*;

#[derive(ecs_macro::Component)]
pub struct Collider {
    rigid_body_at_origin: RigidBody,
}

impl Collider {
    pub fn from_rect(rect_at_origin: Rect<Tile>) -> Self {
        Self {
            rigid_body_at_origin: RigidBody::from_rect(rect_at_origin),
        }
    }
    pub fn get_rigid_body(&self, xy: Xy<Tile>) -> RigidBody {
        self.rigid_body_at_origin.translate(xy)
    }
}
