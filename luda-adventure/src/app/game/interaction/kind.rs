use crate::app::game::Tile;
use namui::Xy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionKind {
    Quest,
    MapTeleport {
        map_name: String,
        player_xy: Xy<Tile>,
    },
}
