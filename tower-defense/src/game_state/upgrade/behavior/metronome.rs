use super::*;

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

impl MetronomeUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Metronome(MetronomeUpgrade { start_stage: None })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    MetronomeUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn metronome_grants_extra_dice_every_two_waves() {
        let mut state = crate::game_state::upgrade::UpgradeState::default();
        state.upgrade(crate::game_state::upgrade::MetronomeUpgrade::into_upgrade());

        let (effects_stage_1, _) = state.stage_start_effects(1);
        assert_eq!(effects_stage_1.extra_dice, 1);

        let (effects_stage_2, _) = state.stage_start_effects(2);
        assert_eq!(effects_stage_2.extra_dice, 0);

        let (effects_stage_3, _) = state.stage_start_effects(3);
        assert_eq!(effects_stage_3.extra_dice, 1);
    }
}
