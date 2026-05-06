use super::*;

// Simple flags
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CrockUpgrade;
impl UpgradeBehavior for CrockUpgrade {
    fn tower_upgrade_damage_bonus(
        &self,
        game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if game_state.gold >= 1000 {
            Some((TowerUpgradeTarget::Global, (game_state.gold / 1000) as f32))
        } else {
            None
        }
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

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::PLAYER_STATS
    }

    fn on_upgrade_acquired_mut(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        let flags = self.on_upgrade_acquired(game_state);
        game_state.hp = game_state.max_hp();
        flags
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PiggyBankUpgrade;
impl UpgradeBehavior for PiggyBankUpgrade {
    fn on_stage_end(
        &mut self,
        _perfect_clear: bool,
        gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        let bonus_gold = if gold >= 500 { gold / 10 } else { 0 };
        (bonus_gold, UpgradeUpdateFlags::RESOURCE)
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CameraUpgrade;
impl UpgradeBehavior for CameraUpgrade {
    fn on_tower_placed(&mut self, tower: &Tower) -> (TowerPlacementResult, UpgradeUpdateFlags) {
        (
            TowerPlacementResult {
                gold_earn: if tower.rank().is_face() { 50 } else { 0 },
            },
            UpgradeUpdateFlags::RESOURCE,
        )
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct GiftBoxUpgrade;
impl UpgradeBehavior for GiftBoxUpgrade {
    fn on_stage_end(
        &mut self,
        _perfect_clear: bool,
        _gold: usize,
        item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        let bonus_gold = item_count * 10;
        (bonus_gold, UpgradeUpdateFlags::RESOURCE)
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FangUpgrade;
impl UpgradeBehavior for FangUpgrade {
    fn on_monster_death(&mut self) -> bool {
        true
    }
}
