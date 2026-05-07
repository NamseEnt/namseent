use super::*;

const PIGGY_BANK_GOLD_STEP: usize = 500;
const PIGGY_BANK_GOLD_REWARD_PER_STEP: usize = 50;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PiggyBankUpgrade;

impl UpgradeBehavior for PiggyBankUpgrade {
    fn on_stage_end(
        &mut self,
        _perfect_clear: bool,
        gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        let bonus_gold = if gold >= PIGGY_BANK_GOLD_STEP {
            gold / PIGGY_BANK_GOLD_STEP * PIGGY_BANK_GOLD_REWARD_PER_STEP
        } else {
            0
        };
        (bonus_gold, UpgradeUpdateFlags::RESOURCE)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Piggy Bank",
            crate::l10n::locale::Language::Korean => "돼지저금통",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.text(match locale.language {
            crate::l10n::locale::Language::English => format!(
                "At stage end, gain {} gold for every {} gold you hold",
                PIGGY_BANK_GOLD_REWARD_PER_STEP, PIGGY_BANK_GOLD_STEP,
            ),
            crate::l10n::locale::Language::Korean => format!(
                "스테이지 종료 시 보유한 골드 {}골드당 {}골드를 획득합니다",
                PIGGY_BANK_GOLD_STEP, PIGGY_BANK_GOLD_REWARD_PER_STEP,
            ),
        });
    }
}

impl PiggyBankUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::PiggyBank(PiggyBankUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PiggyBankUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn piggy_bank_awards_gold_on_stage_end_with_enough_gold() {
        use crate::game_state::upgrade::tests::support;

        let mut gs = support::create_mock_game_state();
        gs.flow =
            crate::game_state::GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
        gs.gold = 500;
        gs.upgrade_state
            .upgrade(crate::game_state::upgrade::PiggyBankUpgrade::into_upgrade());

        crate::game_state::tick::defense_end::check_defense_end(&mut gs);

        assert_eq!(gs.gold, 550);
    }
}
