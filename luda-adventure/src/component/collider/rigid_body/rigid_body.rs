use super::{collide_circle_to_polygon, collide_polygon_to_circle, Circle};
use crate::{app::game::Tile, component::CollisionInfo};
use geo::{coord, polygon, Polygon, Translate};
use namui::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum RigidBody {
    Polygon(Polygon),
    Circle(Circle),
}

impl RigidBody {
    pub fn from_polygon(polygon: Polygon) -> Self {
        Self::Polygon(polygon)
    }
    pub fn from_rect(rect: Rect<Tile>) -> Self {
        Self::from_polygon(polygon![
            (x: rect.left().as_f32() as f64, y: rect.top().as_f32() as f64),
            (x: rect.left().as_f32() as f64, y: rect.bottom().as_f32() as f64),
            (x: rect.right().as_f32() as f64, y: rect.bottom().as_f32() as f64),
            (x: rect.right().as_f32() as f64, y: rect.top().as_f32() as f64),
        ])
    }
    pub fn from_circle(center: Xy<Tile>, radius: Tile) -> Self {
        Self::Circle(Circle::new(
            coord!(x: center.x.as_f32() as f64, y: center.y.as_f32() as f64),
            radius.as_f32() as f64,
        ))
    }

    pub fn translate(&self, xy: Xy<Tile>) -> Self {
        let x = xy.x.as_f32() as f64;
        let y = xy.y.as_f32() as f64;
        match self {
            RigidBody::Polygon(polygon) => Self::Polygon(polygon.translate(x, y)),
            RigidBody::Circle(circle) => Self::Circle(circle.translate(x, y)),
        }
    }

    pub fn collide(&self, other: &Self) -> CollisionInfo {
        match (self, other) {
            (RigidBody::Polygon(_), RigidBody::Polygon(_)) => unimplemented!(),
            (RigidBody::Polygon(polygon), RigidBody::Circle(circle)) => {
                collide_polygon_to_circle(polygon, circle)
            }
            (RigidBody::Circle(circle), RigidBody::Polygon(polygon)) => {
                collide_circle_to_polygon(circle, polygon)
            }
            (RigidBody::Circle(_), RigidBody::Circle(_)) => unimplemented!(),
        }
    }
}
