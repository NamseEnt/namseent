use super::RigidBody;
use crate::app::game::Tile;
use geo::Polygon;
use namui::prelude::*;

#[ecs_macro::component]
pub struct Collider {
    rigid_body_at_origin: RigidBody,
}

impl Collider {
    pub fn from_circle(center: Xy<Tile>, radius: Tile) -> Self {
        Self {
            rigid_body_at_origin: RigidBody::from_circle(center, radius),
        }
    }
    pub fn from_polygon(polygon_at_origin: Polygon) -> Self {
        Self {
            rigid_body_at_origin: RigidBody::from_polygon(polygon_at_origin),
        }
    }
    pub fn from_rect(rect_at_origin: Rect<Tile>) -> Self {
        Self {
            rigid_body_at_origin: RigidBody::from_rect(rect_at_origin),
        }
    }
    pub fn get_rigid_body(&self, xy: Xy<Tile>) -> RigidBody {
        self.rigid_body_at_origin.translate(xy)
    }
}
