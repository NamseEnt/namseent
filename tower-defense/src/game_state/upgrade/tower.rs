use crate::{Suit, game_state::tower::Tower};
use namui::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord, State)]
pub enum TowerUpgradeTarget {
    Global,
    Suit { suit: Suit },
    EvenOdd { even: bool },
    FaceNumber { face: bool },
    LowCardTower,
    NoRerollTower,
    RerolledTower,
    TowerId { tower_id: usize },
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TowerUpgradeDamageBonus {
    pub target: TowerUpgradeTarget,
    pub bonus_pct: f32,
}

impl TowerUpgradeDamageBonus {
    pub fn applies_to_tower(&self, tower: &Tower) -> bool {
        self.target.applies_to_tower(tower)
    }

    pub fn effective_bonus_pct_for_tower(&self, tower: &Tower) -> f32 {
        if !self.applies_to_tower(tower) {
            return 0.0;
        }

        match self.target {
            TowerUpgradeTarget::RerolledTower => {
                let rerolled_count = tower.rerolled_count() as f32;
                self.bonus_pct * rerolled_count
            }
            _ => self.bonus_pct,
        }
    }
}

impl TowerUpgradeTarget {
    fn applies_to_tower(&self, tower: &Tower) -> bool {
        match self {
            TowerUpgradeTarget::TowerId { tower_id } => tower.id() == *tower_id,
            TowerUpgradeTarget::NoRerollTower => tower.rerolled_count() == 0,
            _ => self.applies_to_tower_template(&tower.template),
        }
    }

    pub fn applies_to_tower_template(
        &self,
        tower_template: &crate::game_state::tower::TowerTemplate,
    ) -> bool {
        match self {
            TowerUpgradeTarget::Global => true,
            TowerUpgradeTarget::Suit { suit } => *suit == tower_template.suit,
            TowerUpgradeTarget::EvenOdd { even } => *even == tower_template.rank.is_even(),
            TowerUpgradeTarget::FaceNumber { face } => *face == tower_template.rank.is_face(),
            TowerUpgradeTarget::LowCardTower => tower_template.kind.is_low_card_tower(),
            TowerUpgradeTarget::NoRerollTower => true, // NoRerollTower target is only relevant for placed towers, so it applies to all templates.
            TowerUpgradeTarget::RerolledTower => true, // RerolledTower target is only relevant for placed towers, so it applies to all templates.
            TowerUpgradeTarget::TowerId { .. } => false, // TowerId target does not apply to templates.
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TowerUpgradeState {
    pub damage_multiplier: f32,
}
impl Default for TowerUpgradeState {
    fn default() -> Self {
        TowerUpgradeState {
            damage_multiplier: 1.0,
        }
    }
}
