use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

const PIGGY_BANK_GOLD_STEP: usize = 100;
const PIGGY_BANK_GOLD_REWARD_PER_STEP: usize = 10;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PiggyBankUpgrade;

impl UpgradePresentation for PiggyBankUpgrade {
    fn key(&self) -> &'static str {
        "piggy_bank"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::PIGGY_BANK)
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
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("At stage end, gain ")
                .l10n(Word::Gold.name(), locale)
                .with_bold(format!(" +{}", PIGGY_BANK_GOLD_REWARD_PER_STEP))
                .static_text(" for every ")
                .l10n(Word::Gold.name(), locale)
                .with_bold(format!(" {}", PIGGY_BANK_GOLD_STEP))
                .static_text(" you hold"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("스테이지 종료 시 보유한 ")
                .l10n(Word::Gold.name(), locale)
                .with_bold(format!(" {}", PIGGY_BANK_GOLD_STEP))
                .static_text("당 ")
                .l10n(Word::Gold.name(), locale)
                .with_bold(format!(" +{}", PIGGY_BANK_GOLD_REWARD_PER_STEP)),
        };
    }
}

impl PiggyBankUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::PiggyBank(PiggyBankUpgrade)
    }
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
        gs.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::PiggyBankUpgrade::into_upgrade(),
            None,
        ));

        support::check_defense_end_for_test(&mut gs);

        assert_eq!(gs.gold, 550);
    }
}
