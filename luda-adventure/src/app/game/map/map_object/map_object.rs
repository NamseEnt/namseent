use super::Portal;
use namui::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapObject {
    Portal(Portal),
}

impl MapObject {
    pub fn create_entity(&self, app: &mut crate::ecs::App) {
        match self {
            MapObject::Portal(portal) => portal.create_entity(app),
        }
    }
}
