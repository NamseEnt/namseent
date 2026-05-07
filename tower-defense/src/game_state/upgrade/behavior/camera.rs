use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CameraUpgrade;

impl UpgradeBehavior for CameraUpgrade {
    fn on_tower_placed(&mut self, tower: &Tower) -> (TowerPlacementResult, UpgradeUpdateFlags) {
        (
            TowerPlacementResult {
                gold_earn: if tower.rank().is_face() { 50 } else { 0 },
            },
            UpgradeUpdateFlags::RESOURCE,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Camera",
            crate::l10n::locale::Language::Korean => "카메라",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Gain 50 gold when placing a face tower",
            crate::l10n::locale::Language::Korean => "페이스 타워를 배치하면 50골드를 얻습니다",
        });
    }
}

impl CameraUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Camera(CameraUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    CameraUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn camera_grants_gold_when_face_tower_is_placed() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let initial_gold = game_state.gold;

        game_state
            .upgrade_state
            .upgrade(crate::game_state::upgrade::CameraUpgrade::into_upgrade());

        let face_tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::King,
        );
        let face_tower = crate::game_state::tower::Tower::new(
            &face_tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.place_tower(face_tower);

        assert_eq!(game_state.gold, initial_gold + 50);
    }

    #[test]
    fn camera_does_not_grant_gold_for_number_tower() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let initial_gold = game_state.gold;

        game_state
            .upgrade_state
            .upgrade(crate::game_state::upgrade::CameraUpgrade::into_upgrade());

        let number_tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::Ten,
        );
        let number_tower = crate::game_state::tower::Tower::new(
            &number_tower_template,
            crate::MapCoord::new(2, 0),
            game_state.now(),
        );
        game_state.place_tower(number_tower);

        assert_eq!(game_state.gold, initial_gold);
    }
}

