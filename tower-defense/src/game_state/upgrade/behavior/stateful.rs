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
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DemolitionHammerUpgrade {
    pub damage_multiplier: f32,
    pub removed_tower_count: usize,
}
impl UpgradeBehavior for DemolitionHammerUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.removed_tower_count > 0 {
            effects.damage_multiplier *=
                1.0 + self.damage_multiplier * self.removed_tower_count as f32;
            self.removed_tower_count = 0;
        }
    }

    fn record_tower_removed(&mut self) {
        self.removed_tower_count += 1;
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TrophyUpgrade {
    pub perfect_clear_stacks: usize,
}
impl UpgradeBehavior for TrophyUpgrade {
    fn record_perfect_clear(&mut self) {
        self.perfect_clear_stacks += 1;
    }

    fn get_global_damage_multiplier(&self, _game_state: &GameState) -> Option<f32> {
        if self.perfect_clear_stacks > 0 {
            Some(self.perfect_clear_stacks as f32 * (TROPHY_DAMAGE_MULTIPLIER - 1.0))
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
    fn get_global_damage_multiplier(&self, _game_state: &GameState) -> Option<f32> {
        if self.stacks > 0 {
            Some(self.stacks as f32 * (self.damage_multiplier - 1.0))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct NameTagUpgrade {
    pub damage_multiplier: f32,
    pub pending: bool,
}
impl UpgradeBehavior for NameTagUpgrade {
    fn apply_pending_placement_bonuses(
        &mut self,
        tower_template: &mut TowerTemplate,
        _left_dice: usize,
    ) {
        if self.pending {
            tower_template
                .default_status_effects
                .push(TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageMul {
                        mul: self.damage_multiplier,
                    },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                });
            self.pending = false;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ResolutionUpgrade {
    pub damage_multiplier_per_reroll: f32,
    pub pending: bool,
}
impl UpgradeBehavior for ResolutionUpgrade {
    fn apply_pending_placement_bonuses(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) {
        if self.pending {
            let multiplier = 1.0 + left_dice as f32 * self.damage_multiplier_per_reroll;
            tower_template
                .default_status_effects
                .push(TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageMul { mul: multiplier },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                });
            self.pending = false;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MirrorUpgrade {
    pub pending: bool,
}
impl UpgradeBehavior for MirrorUpgrade {
    fn consume_pending_mirror_count(&mut self) -> usize {
        if self.pending {
            self.pending = false;
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct IceCreamUpgrade {
    pub damage_multiplier: f32,
    pub waves_remaining: usize,
}
impl UpgradeBehavior for IceCreamUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.waves_remaining > 0 {
            effects.damage_multiplier *= self.damage_multiplier;
            self.waves_remaining -= 1;
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
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PopcornUpgrade {
    pub max_multiplier: f32,
    pub duration: usize,
    pub waves_remaining: usize,
}
impl UpgradeBehavior for PopcornUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.waves_remaining > 0 {
            let duration = self.duration.max(1);
            let elapsed = duration.saturating_sub(self.waves_remaining);
            let popcorn_multiplier = if duration <= 1 {
                self.max_multiplier
            } else {
                let step = (self.max_multiplier - 1.0) / (duration - 1) as f32;
                (self.max_multiplier - step * elapsed as f32).max(1.0)
            };

            effects.damage_multiplier *= popcorn_multiplier;
            self.waves_remaining -= 1;
        }
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
}
