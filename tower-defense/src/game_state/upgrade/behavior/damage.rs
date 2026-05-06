use super::*;

// Tower damage multipliers (suit-based and others)
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct StaffUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for StaffUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::Suit {
                suit: crate::card::Suit::Diamonds,
            },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct LongSwordUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for LongSwordUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::Suit {
                suit: crate::card::Suit::Spades,
            },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MaceUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for MaceUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::Suit {
                suit: crate::card::Suit::Hearts,
            },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ClubSwordUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for ClubSwordUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::Suit {
                suit: crate::card::Suit::Clubs,
            },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SingleChopstickUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for SingleChopstickUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::EvenOdd { even: false },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PairChopsticksUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for PairChopsticksUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::EvenOdd { even: true },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FountainPenUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for FountainPenUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::FaceNumber { face: false },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrushUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for BrushUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::FaceNumber { face: true },
            self.damage_multiplier - 1.0,
        ))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrokenPotteryUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for BrokenPotteryUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        Some((TowerUpgradeTarget::Global, self.damage_multiplier - 1.0))
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

// Tower select upgrades
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TricycleUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for TricycleUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PerfectPotteryUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for PerfectPotteryUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }
}
