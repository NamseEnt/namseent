use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

const REROLLS_PER_BONUS: usize = 4;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrokenPotteryUpgrade;

impl UpgradeBehavior for BrokenPotteryUpgrade {
    fn key(&self) -> &'static str {
        "broken_pottery"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::BROKEN_POTTERY,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn on_card_reroll(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        if game_state.rerolled_count > 0
            && game_state.rerolled_count.is_multiple_of(REROLLS_PER_BONUS)
        {
            game_state.action(crate::game_state::GameStateAction::GainRerolls(1));
        }

        UpgradeUpdateFlags::NONE
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
                .with_dice_value(" +1")
                .static_text(" every 4 card rerolls"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("카드 리롤 4회마다 ")
                .l10n(Word::Dice.name(), locale)
                .with_dice_value(" +1"),
        };
    }
}

impl BrokenPotteryUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::BrokenPottery(BrokenPotteryUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_common,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    BrokenPotteryUpgrade::into_upgrade()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broken_pottery_grants_one_reroll_every_four_card_rerolls() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let mut upgrade = BrokenPotteryUpgrade;
        game_state.left_dice = 0;

        for rerolled_count in 0..4 {
            game_state.rerolled_count = rerolled_count;
            upgrade.on_card_reroll(&mut game_state);
        }
        assert_eq!(game_state.left_dice, 0);

        game_state.rerolled_count = 4;
        upgrade.on_card_reroll(&mut game_state);
        assert_eq!(game_state.left_dice, 1);

        game_state.rerolled_count = 5;
        upgrade.on_card_reroll(&mut game_state);
        assert_eq!(game_state.left_dice, 1);

        game_state.rerolled_count = 8;
        upgrade.on_card_reroll(&mut game_state);
        assert_eq!(game_state.left_dice, 2);
    }

    #[test]
    fn broken_pottery_does_not_increase_tower_damage() {
        let state = UpgradeState::with_upgrades(vec![BrokenPotteryUpgrade::into_upgrade()]);

        assert!(state.tower_upgrade_damage_bonuses().is_empty());
    }
}
