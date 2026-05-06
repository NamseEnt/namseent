use super::*;

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

impl SlotMachineUpgrade {
    pub fn into_upgrade(next_round_dice: usize) -> Upgrade {
        Upgrade::SlotMachine(SlotMachineUpgrade { next_round_dice })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    SlotMachineUpgrade::into_upgrade(10)
}
#[cfg(test)]
mod tests {

    #[test]
    fn slot_machine_grants_extra_dice_on_stage_start_only_once() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.upgrade(crate::game_state::upgrade::SlotMachineUpgrade::into_upgrade(10));

        game_state.apply_stage_start(1);
        assert_eq!(
            game_state.left_dice,
            game_state.max_dice_chance() + 10,
            "slot machine should add extra dice on the first stage start",
        );

        game_state.apply_stage_start(2);
        assert_eq!(
            game_state.left_dice,
            game_state.max_dice_chance(),
            "slot machine should only apply extra dice once",
        );
    }
}
