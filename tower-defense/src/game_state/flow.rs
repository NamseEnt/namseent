use crate::{tower::TowerBlueprint, upgrade::Upgrade};

#[derive(Debug, Clone)]
pub enum GameFlow {
    SelectingTower,
    PlacingTower { tower: TowerBlueprint },
    SelectingUpgrade { upgrades: [Upgrade; 3] },
}
