use super::*;
use crate::l10n::word::Word;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DemolitionHammerUpgrade;

impl UpgradeBehavior for DemolitionHammerUpgrade {
    fn key(&self) -> &'static str {
        "demolition_hammer"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::DEMOLITION_HAMMER)
    }

    fn on_tower_removed(
        &mut self,
        game_state: &mut GameState,
        tower: &Tower,
    ) -> UpgradeUpdateFlags {
        game_state.action(crate::game_state::GameStateAction::GainRerolls(
            tower.rerolled_count(),
        ));
        UpgradeUpdateFlags::NONE
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Demolition Hammer",
            crate::l10n::locale::Language::Korean => "철거 망치",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Gain ")
                .l10n(Word::Dice.name(), locale)
                .static_text(" equal to the removed tower's rerolls"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("타워 철거 시 해당 타워에 사용된 리롤 횟수만큼 ")
                .l10n(Word::Dice.name(), locale)
                .static_text(" 획득"),
        };
    }
}

impl DemolitionHammerUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::DemolitionHammer(DemolitionHammerUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_legendary,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    DemolitionHammerUpgrade::into_upgrade()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn demolition_hammer_grants_rerolls_used_by_removed_tower() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.left_dice = 0;
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            DemolitionHammerUpgrade::into_upgrade(),
            None,
        ));

        let mut tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            Suit::Hearts,
            Rank::Two,
        );
        tower_template.rerolled_count = 3;
        let tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.action(crate::game_state::GameStateAction::PlaceTower(
            Box::new(tower),
            None,
        ));

        let tower_id = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower placed")
            .id();
        assert!(game_state.action(crate::game_state::GameStateAction::RemoveTower(tower_id)));

        assert_eq!(game_state.left_dice, 3);
    }

    #[test]
    fn demolition_hammer_does_not_increase_tower_damage() {
        let state = UpgradeState::with_upgrades(vec![DemolitionHammerUpgrade::into_upgrade()]);

        assert!(state.tower_upgrade_damage_bonuses().is_empty());
    }
}
