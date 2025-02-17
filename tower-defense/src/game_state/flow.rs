use super::tower::TowerTemplate;
use crate::upgrade::Upgrade;

#[derive(Clone)]
pub enum GameFlow {
    SelectingTower,
    PlacingTower { tower: TowerTemplate },
    SelectingUpgrade { upgrades: [Upgrade; 3] },
}
