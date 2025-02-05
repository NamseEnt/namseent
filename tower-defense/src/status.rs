use crate::{tower::TowerBlueprint, upgrade::Upgrade};
use namui::*;

pub static FLOW_ATOM: Atom<Flow> = Atom::uninitialized();
pub static UPGRADES_ATOM: Atom<Vec<Upgrade>> = Atom::uninitialized();

#[derive(Debug, Clone)]
pub enum Flow {
    SelectingTower,
    PlacingTower { tower: TowerBlueprint },
    SelectingUpgrade { upgrades: [Upgrade; 3] },
}
pub fn go_to_selecting_tower() {
    FLOW_ATOM.set(Flow::SelectingTower);
}
pub fn go_to_selecting_upgrade() {
    todo!("Generate upgrades");
    // FLOW_ATOM.set(Flow::SelectingUpgrade { upgrades });
}
