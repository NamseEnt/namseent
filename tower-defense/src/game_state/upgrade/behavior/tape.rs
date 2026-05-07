use super::*;

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
        let before = self.acquired_stage;
        self.acquired_stage = game_state.stage;
        let mut flags = self.on_upgrade_acquired(game_state);
        if self.acquired_stage != before {
            flags |= UpgradeUpdateFlags::REVISION_REQUIRED;
        }
        flags
    }
}

impl TapeUpgrade {
    pub fn into_upgrade(acquired_stage: usize) -> Upgrade {
        Upgrade::Tape(TapeUpgrade { acquired_stage })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    TapeUpgrade::into_upgrade(0)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::upgrade::{UpgradeBehavior, UpgradeUpdateFlags};

    #[test]
    fn tape_applies_enemy_speed_reduction_every_four_waves() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.stage = 3;

        game_state.upgrade(crate::game_state::upgrade::TapeUpgrade::into_upgrade(0));

        let (effects_stage_3, _) = game_state.upgrade_state.stage_start_effects(3);
        assert_eq!(effects_stage_3.enemy_speed_multiplier, None);

        let (effects_stage_4, _) = game_state.upgrade_state.stage_start_effects(4);
        assert_eq!(effects_stage_4.enemy_speed_multiplier, Some(0.75));

        let (effects_stage_5, _) = game_state.upgrade_state.stage_start_effects(5);
        assert_eq!(effects_stage_5.enemy_speed_multiplier, None);

        let (effects_stage_8, _) = game_state.upgrade_state.stage_start_effects(8);
        assert_eq!(effects_stage_8.enemy_speed_multiplier, Some(0.75));
    }

    #[test]
    fn tape_returns_revision_required_when_acquired_stage_changes() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.stage = 5;
        let mut upgrade = TapeUpgrade { acquired_stage: 0 };

        let flags = upgrade.on_upgrade_acquired_mut(&mut game_state);

        assert_eq!(upgrade.acquired_stage, 5);
        assert!(flags.contains(UpgradeUpdateFlags::REVISION_REQUIRED));
    }
}
