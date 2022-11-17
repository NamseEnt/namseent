mod app;
mod entity;

pub use app::*;
pub use entity::*;
use namui::Uuid;

pub trait Component {
    fn insert(self, app_id: Uuid, entity_id: Uuid);
    fn drop(app_id: Uuid, entity_id: Uuid);
}

pub trait ComponentCombination {
    fn filter(app_id: Uuid, entity: &Entity) -> Option<Self>
    where
        Self: Sized;
}

pub trait ComponentCombinationMut {
    fn filter(app_id: Uuid, entity: &mut Entity) -> Option<Self>
    where
        Self: Sized;
}
