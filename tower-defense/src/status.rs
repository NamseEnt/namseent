use crate::tower::TowerBlueprint;
use namui::*;

pub static FLOW_ATOM: Atom<Flow> = Atom::uninitialized();

#[derive(Debug, Clone)]
pub enum Flow {
    SelectingTower,
    PlacingTower { tower: TowerBlueprint },
}
