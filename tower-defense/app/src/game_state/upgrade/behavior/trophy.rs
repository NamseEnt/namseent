use super::*;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TrophyUpgrade;

impl UpgradePresentation for TrophyUpgrade {
    fn key(&self) -> &'static str {
        "trophy"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::TROPHY)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Trophy",
            crate::l10n::locale::Language::Korean => "트로피",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .l10n(Word::PerfectClear.name(), locale)
                .static_text(" adds one free ")
                .l10n(Word::CardService.name(), locale)
                .static_text(" to the next shop"),
            crate::l10n::locale::Language::Korean => builder
                .l10n(Word::PerfectClear.name(), locale)
                .static_text(" 시 다음 상점에 무료 ")
                .l10n(Word::CardService.name(), locale)
                .static_text(" 1개 추가"),
        };
    }

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![
            self.tooltip_section(locale),
            Word::PerfectClear.tooltip_section(locale),
        ]
    }
}

impl TrophyUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Trophy(TrophyUpgrade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{GameFlow, flow::DefenseFlow};
    use crate::shop::ShopSlot;

    #[test]
    fn trophy_adds_one_free_card_service_to_the_next_shop_after_a_perfect_clear() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            TrophyUpgrade::into_upgrade(),
            None,
        ));
        let base_shop_slot_count = game_state.max_shop_slot();
        game_state.flow = GameFlow::Defense(DefenseFlow::new(&game_state));

        support::check_defense_end_for_test(&mut game_state);

        let GameFlow::Shopping(flow) = &game_state.flow else {
            panic!("expected shopping flow");
        };
        assert_eq!(flow.shop.slots.len(), base_shop_slot_count + 1);
        assert_eq!(
            flow.shop
                .slots
                .iter()
                .filter(|slot_data| matches!(
                    &slot_data.slot,
                    ShopSlot::CardService { cost: 0, .. }
                ))
                .count(),
            1
        );
    }

    #[test]
    fn trophy_does_not_add_a_card_service_after_taking_damage() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            TrophyUpgrade::into_upgrade(),
            None,
        ));
        let base_shop_slot_count = game_state.max_shop_slot();
        let mut defense_flow = DefenseFlow::new(&game_state);
        defense_flow.took_damage = true;
        game_state.flow = GameFlow::Defense(defense_flow);

        support::check_defense_end_for_test(&mut game_state);

        let GameFlow::Shopping(flow) = &game_state.flow else {
            panic!("expected shopping flow");
        };
        assert_eq!(flow.shop.slots.len(), base_shop_slot_count);
        assert!(
            !flow
                .shop
                .slots
                .iter()
                .any(|slot_data| matches!(&slot_data.slot, ShopSlot::CardService { cost: 0, .. }))
        );
    }
}
