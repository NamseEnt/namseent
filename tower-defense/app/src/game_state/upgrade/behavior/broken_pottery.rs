use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

const BROKEN_POTTERY_REROLL_INTERVAL: usize = 4;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrokenPotteryUpgrade {
    pub(crate) rerolled_count: usize,
}

impl UpgradePresentation for BrokenPotteryUpgrade {
    fn key(&self) -> &'static str {
        "broken_pottery"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::BROKEN_POTTERY)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        vec![crate::thumbnail::ThumbnailOverlay::right_top(
            format!("{}/{}", self.rerolled_count, BROKEN_POTTERY_REROLL_INTERVAL),
            crate::theme::palette::WHITE,
        )]
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
        Upgrade::BrokenPottery(BrokenPotteryUpgrade { rerolled_count: 0 })
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

        let mut raw = game_state.raw_core_state().clone();
        for _ in 0..3 {
            raw.trigger_card_reroll_upgrades();
        }
        assert_eq!(raw.upgrades().entries()[0].scalar_value(0), Some(3));
        assert_eq!(raw.progress().left_dice, 0);
        game_state
            .restore_raw_core_projection(raw.clone())
            .expect("valid broken pottery projection");
        assert_eq!(
            game_state.presentation_upgrade_state_snapshot().upgrades[0]
                .upgrade
                .thumbnail_overlays(&game_state),
            vec![crate::thumbnail::ThumbnailOverlay::right_top(
                "3/4",
                crate::theme::palette::WHITE,
            )]
        );

        raw.trigger_card_reroll_upgrades();
        assert_eq!(raw.upgrades().entries()[0].scalar_value(0), Some(0));
        assert_eq!(raw.progress().left_dice, 1);

        game_state
            .restore_raw_core_projection(raw)
            .expect("valid broken pottery projection");
        assert_eq!(
            game_state.presentation_upgrade_state_snapshot().upgrades[0].upgrade,
            Upgrade::BrokenPottery(BrokenPotteryUpgrade { rerolled_count: 0 })
        );
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
