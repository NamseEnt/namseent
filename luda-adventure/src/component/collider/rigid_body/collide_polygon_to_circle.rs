use super::{collide_circle_to_polygon, Circle};
use crate::component::CollisionInfo;
use geo::Polygon;

pub fn collide_polygon_to_circle(polygon: &Polygon, circle: &Circle) -> CollisionInfo {
    collide_circle_to_polygon(circle, polygon).reverse_collision_normal()
}
