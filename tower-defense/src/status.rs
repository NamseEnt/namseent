use crate::{tower::TowerBlueprint, upgrade::Upgrade};
use namui::*;

pub static FLOW_ATOM: Atom<Flow> = Atom::uninitialized();
pub static UPGRADES_ATOM: Atom<Vec<Upgrade>> = Atom::uninitialized();

#[derive(Debug, Clone)]
pub enum Flow {
    SelectingTower,
    PlacingTower { tower: TowerBlueprint },
}
