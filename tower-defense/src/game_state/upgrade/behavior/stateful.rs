use super::*;

// Stateful upgrades with stage-based effects
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MetronomeUpgrade {
    pub start_stage: Option<usize>,
}
impl UpgradeBehavior for MetronomeUpgrade {
    fn apply_on_stage_start(&mut self, stage: usize, effects: &mut StageStartEffects) {
        let start = self.start_stage.get_or_insert(stage);
        if stage >= *start && (stage - *start).is_multiple_of(2) {
            effects.extra_dice += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TapeUpgrade {
    pub acquired_stage: usize,
}
impl UpgradeBehavior for TapeUpgrade {
    fn apply_on_stage_start(&mut self, stage: usize, effects: &mut StageStartEffects) {
        if stage > self.acquired_stage && (stage - self.acquired_stage - 1).is_multiple_of(4) {
            effects.enemy_speed_multiplier = Some(0.75);
        }
    }

    fn on_upgrade_acquired_mut(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        self.acquired_stage = game_state.stage;
        self.on_upgrade_acquired(game_state)
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DemolitionHammerUpgrade {
    pub damage_multiplier: f32,
    pub removed_tower_count: usize,
    pub stored_damage_bonus: f32,
}
impl UpgradeBehavior for DemolitionHammerUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, _effects: &mut StageStartEffects) {}

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.stored_damage_bonus > 0.0 {
            Some((TowerUpgradeTarget::Global, self.stored_damage_bonus))
        } else {
            None
        }
    }

    fn on_tower_removed(&mut self) -> UpgradeUpdateFlags {
        self.removed_tower_count += 1;
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn on_stage_end(
        &mut self,
        _perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        if self.removed_tower_count == 0 {
            return (0, UpgradeUpdateFlags::NONE);
        }

        self.stored_damage_bonus += self.damage_multiplier * self.removed_tower_count as f32;
        self.removed_tower_count = 0;
        (0, UpgradeUpdateFlags::TOWER_STATS)
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TrophyUpgrade {
    pub perfect_clear_stacks: usize,
}
impl UpgradeBehavior for TrophyUpgrade {
    fn on_stage_end(
        &mut self,
        perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        if perfect_clear {
            self.perfect_clear_stacks += 1;
        }
        (0, UpgradeUpdateFlags::TOWER_STATS)
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.perfect_clear_stacks > 0 {
            Some((
                TowerUpgradeTarget::Global,
                self.perfect_clear_stacks as f32 * (TROPHY_DAMAGE_MULTIPLIER - 1.0),
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ShoppingBagUpgrade {
    pub damage_multiplier: f32,
    pub stacks: usize,
}
impl UpgradeBehavior for ShoppingBagUpgrade {
    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.stacks > 0 {
            Some((
                TowerUpgradeTarget::Global,
                self.stacks as f32 * (self.damage_multiplier - 1.0),
            ))
        } else {
            None
        }
    }

    fn on_item_bought(&mut self) -> UpgradeUpdateFlags {
        self.stacks += 1;
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct NameTagUpgrade {
    pub damage_multiplier: f32,
    pub target_tower_id: Option<usize>,
}
impl UpgradeBehavior for NameTagUpgrade {
    fn on_tower_placed(&mut self, tower: &Tower) -> (TowerPlacementResult, UpgradeUpdateFlags) {
        if self.target_tower_id.is_some() {
            return (TowerPlacementResult::default(), UpgradeUpdateFlags::NONE);
        }

        self.target_tower_id = Some(tower.id());
        (
            TowerPlacementResult::default(),
            UpgradeUpdateFlags::TOWER_STATS,
        )
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        self.target_tower_id.map(|tower_id| {
            (
                TowerUpgradeTarget::TowerId { tower_id },
                self.damage_multiplier - 1.0,
            )
        })
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ResolutionUpgrade {
    pub damage_multiplier_per_reroll: f32,
    pub stored_rerolls: usize,
}
impl UpgradeBehavior for ResolutionUpgrade {
    fn on_stage_end_with_state(
        &mut self,
        game_state: &GameState,
        _perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        self.stored_rerolls = game_state.left_dice;
        (0, UpgradeUpdateFlags::NONE)
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.stored_rerolls > 0 {
            Some((
                TowerUpgradeTarget::Global,
                self.stored_rerolls as f32 * self.damage_multiplier_per_reroll,
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MirrorUpgrade {
    pub pending: bool,
}
impl UpgradeBehavior for MirrorUpgrade {
    fn on_tower_placement(
        &mut self,
        _tower_template: &mut TowerTemplate,
        _left_dice: usize,
    ) -> usize {
        0
    }

    fn on_tower_placed_mut(
        &mut self,
        game_state: &mut GameState,
        tower: &Tower,
    ) -> UpgradeUpdateFlags {
        if !self.pending {
            return UpgradeUpdateFlags::NONE;
        }

        let tower_template = (**tower).clone();
        game_state
            .hand
            .push(crate::hand::HandItem::Tower(tower_template));
        self.pending = false;
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct IceCreamUpgrade {
    pub damage_multiplier: f32,
    pub waves_remaining: usize,
}
impl UpgradeBehavior for IceCreamUpgrade {
    fn on_stage_start(
        &mut self,
        _stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        if self.waves_remaining > 0 {
            effects.damage_multiplier += self.damage_multiplier - 1.0;
        }
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn on_upgrade_acquired_mut(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.waves_remaining > 0 {
            Some((TowerUpgradeTarget::Global, self.damage_multiplier - 1.0))
        } else {
            None
        }
    }

    fn on_stage_end(
        &mut self,
        _perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        if self.waves_remaining > 0 {
            self.waves_remaining -= 1;
            (0, UpgradeUpdateFlags::TOWER_STATS)
        } else {
            (0, UpgradeUpdateFlags::NONE)
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SlotMachineUpgrade {
    pub next_round_dice: usize,
}
impl UpgradeBehavior for SlotMachineUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.next_round_dice > 0 {
            effects.extra_dice += self.next_round_dice;
            self.next_round_dice = 0;
        }
    }
    fn on_stage_start(
        &mut self,
        stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        self.apply_on_stage_start(stage, effects);
        UpgradeUpdateFlags::RESOURCE
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PopcornUpgrade {
    pub max_multiplier: f32,
    pub duration: usize,
    pub waves_remaining: usize,
    pub active_stage_damage_bonus: f32,
}
impl UpgradeBehavior for PopcornUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        self.active_stage_damage_bonus = 0.0;
        if self.waves_remaining > 0 {
            let duration = self.duration.max(1);
            let elapsed = duration.saturating_sub(self.waves_remaining);
            let popcorn_multiplier = if duration <= 1 {
                self.max_multiplier
            } else {
                let step = (self.max_multiplier - 1.0) / (duration - 1) as f32;
                (self.max_multiplier - step * elapsed as f32).max(1.0)
            };

            self.active_stage_damage_bonus = popcorn_multiplier - 1.0;
            effects.damage_multiplier += self.active_stage_damage_bonus;
            self.waves_remaining -= 1;
        }
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.active_stage_damage_bonus > 0.0 {
            Some((TowerUpgradeTarget::Global, self.active_stage_damage_bonus))
        } else {
            None
        }
    }

    fn on_stage_start(
        &mut self,
        stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        self.apply_on_stage_start(stage, effects);
        UpgradeUpdateFlags::TOWER_STATS
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MembershipCardUpgrade {
    pub pending_free_shop: bool,
}
impl UpgradeBehavior for MembershipCardUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.pending_free_shop {
            effects.free_shop_this_stage = true;
            self.pending_free_shop = false;
        }
    }

    fn on_stage_start(
        &mut self,
        stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        self.apply_on_stage_start(stage, effects);
        UpgradeUpdateFlags::RESOURCE
    }
}
