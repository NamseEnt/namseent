use super::*;

// Simple flags
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CrockUpgrade;
impl UpgradeBehavior for CrockUpgrade {
    fn get_global_damage_multiplier(&self, game_state: &GameState) -> Option<f32> {
        Some((game_state.gold / 1000) as f32)
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SpannerUpgrade;
impl UpgradeBehavior for SpannerUpgrade {
    fn clear_shield_on_stage_start(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PeaUpgrade;
impl UpgradeBehavior for PeaUpgrade {
    fn max_hp_plus(&self) -> f32 {
        10.0
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PiggyBankUpgrade;
impl UpgradeBehavior for PiggyBankUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CameraUpgrade;
impl UpgradeBehavior for CameraUpgrade {
    fn on_tower_placed(&mut self, tower: &Tower) -> TowerPlacementResult {
        TowerPlacementResult {
            gold_earn: if tower.rank().is_face() { 50 } else { 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct GiftBoxUpgrade;
impl UpgradeBehavior for GiftBoxUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FangUpgrade;
impl UpgradeBehavior for FangUpgrade {
    fn on_monster_death(&mut self) -> bool {
        true
    }
}
