use crate::app::game::Tile;
use namui::prelude::*;

pub type Velocity = Xy<Per<Tile, Time>>;
