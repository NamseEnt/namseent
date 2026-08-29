use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrokenPotteryUpgrade;

impl UpgradePresentation for BrokenPotteryUpgrade {
    fn key(&self) -> &'static str {
        "broken_pottery"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::BROKEN_POTTERY)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Broken Pottery",
            crate::l10n::locale::Language::Korean => "깨진 도자기",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .l10n(Word::Dice.name(), locale)
                .with_bold(" +1")
                .static_text(" every 4 card rerolls"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("카드 리롤 4회마다 ")
                .l10n(Word::Dice.name(), locale)
                .with_bold(" +1"),
        };
    }
}

impl BrokenPotteryUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::BrokenPottery(BrokenPotteryUpgrade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broken_pottery_grants_one_reroll_every_four_card_rerolls() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            BrokenPotteryUpgrade::into_upgrade(),
            None,
        ));
        let mut raw = game_state.raw_core_state().clone();
        raw.edit_snapshot(|parts| parts.progress.left_dice = 0)
            .expect("valid dice count");
        game_state
            .restore_raw_core_projection(raw)
            .expect("valid broken pottery projection");

        for rerolled_count in 0..4 {
            let mut raw = game_state.raw_core_state().clone();
            raw.edit_snapshot(|parts| parts.progress.rerolled_count = rerolled_count)
                .expect("valid reroll count");
            raw.trigger_card_reroll_upgrades();
            game_state
                .restore_raw_core_projection(raw)
                .expect("valid broken pottery projection");
        }
        assert_eq!(game_state.raw_core_state().progress().left_dice, 0);

        let mut raw = game_state.raw_core_state().clone();
        raw.edit_snapshot(|parts| parts.progress.rerolled_count = 4)
            .expect("valid reroll count");
        raw.trigger_card_reroll_upgrades();
        game_state
            .restore_raw_core_projection(raw)
            .expect("valid broken pottery projection");
        assert_eq!(game_state.raw_core_state().progress().left_dice, 1);

        let mut raw = game_state.raw_core_state().clone();
        raw.edit_snapshot(|parts| parts.progress.rerolled_count = 5)
            .expect("valid reroll count");
        raw.trigger_card_reroll_upgrades();
        game_state
            .restore_raw_core_projection(raw)
            .expect("valid broken pottery projection");
        assert_eq!(game_state.raw_core_state().progress().left_dice, 1);

        let mut raw = game_state.raw_core_state().clone();
        raw.edit_snapshot(|parts| parts.progress.rerolled_count = 8)
            .expect("valid reroll count");
        raw.trigger_card_reroll_upgrades();
        game_state
            .restore_raw_core_projection(raw)
            .expect("valid broken pottery projection");
        assert_eq!(game_state.raw_core_state().progress().left_dice, 2);
    }

    #[test]
    fn broken_pottery_does_not_increase_tower_damage() {
        let state = UpgradeState::with_upgrades(vec![BrokenPotteryUpgrade::into_upgrade()]);

        assert_eq!(
            state.to_core_state().tower_damage_bonus_raw_for_template(
                &crate::game_state::tower::TowerTemplate::rubber_cone().to_core_state()
            ),
            0
        );
    }
}
