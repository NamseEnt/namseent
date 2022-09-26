use crate::app::game::{GameObject, Tile};
use namui::{Rect, Time};

pub trait Collider: GameObject {
    fn get_collision_box(&self, current_time: Time) -> CollisionBox;
}

pub type CollisionBox = Rect<Tile>;
