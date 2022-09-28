use crate::app::game::{Position, Tile};
use namui::Rect;

pub type CollisionBox = Rect<Tile>;

#[derive(ecs_macro::Component)]
pub struct Collider {
    collision_offset_rect: Rect<Tile>,
}

impl Collider {
    pub fn new(collision_offset_rect: Rect<Tile>) -> Self {
        Self {
            collision_offset_rect,
        }
    }
    pub fn get_collision_box(&self, position: Position) -> CollisionBox {
        Rect::Xywh {
            x: position.x + self.collision_offset_rect.x(),
            y: position.y + self.collision_offset_rect.y(),
            width: self.collision_offset_rect.width(),
            height: self.collision_offset_rect.height(),
        }
    }
}
