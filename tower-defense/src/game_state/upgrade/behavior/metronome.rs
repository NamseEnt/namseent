use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MetronomeUpgrade {
    pub start_stage: Option<usize>,
}

impl UpgradeBehavior for MetronomeUpgrade {
    fn on_stage_start(
        &mut self,
        stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        let before = self.start_stage;
        self.apply_on_stage_start(stage, effects);
        if self.start_stage != before {
            UpgradeUpdateFlags::REVISION_REQUIRED
        } else {
            UpgradeUpdateFlags::NONE
        }
    }

    fn apply_on_stage_start(&mut self, stage: usize, effects: &mut StageStartEffects) {
        let start = self.start_stage.get_or_insert(stage);
        if stage >= *start && (stage - *start).is_multiple_of(2) {
            effects.extra_dice += 1;
        }
    }

    fn l10n_name<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Metronome",
            crate::l10n::locale::Language::Korean => "메트로놈",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Gain 1 extra dice every 2 stages",
            crate::l10n::locale::Language::Korean => "2스테이지마다 주사위 +1을 얻습니다",
        });
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
    use super::*;
    use crate::game_state::upgrade::{UpgradeBehavior, UpgradeUpdateFlags};

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

    #[test]
    fn metronome_returns_revision_required_only_on_first_stage_start() {
        let mut upgrade = MetronomeUpgrade { start_stage: None };
        let mut effects = StageStartEffects::new();

        let first_flags = upgrade.on_stage_start(1, &mut effects);
        let second_flags = upgrade.on_stage_start(2, &mut effects);

        assert_eq!(first_flags, UpgradeUpdateFlags::REVISION_REQUIRED);
        assert_eq!(second_flags, UpgradeUpdateFlags::NONE);
    }
}

