use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CameraUpgrade;

const CAMERA_GOLD_REWARD: usize = 50;

impl UpgradeBehavior for CameraUpgrade {
    fn key(&self) -> &'static str {
        "camera"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::CAMERA,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn thumbnail_overlay(
        &self,
        width_height: Wh<Px>,
        _game_state: &GameState,
    ) -> Option<RenderingTree> {
        Some(crate::thumbnail::render_right_bottom_overlay(
            width_height,
            &format!("{}", CAMERA_GOLD_REWARD),
            crate::theme::palette::YELLOW,
        ))
    }

    fn on_tower_placed(&mut self, game_state: &mut GameState, tower: &Tower) -> UpgradeUpdateFlags {
        if tower.rank().is_some_and(|rank| rank.is_face()) {
            game_state.action(crate::game_state::GameStateAction::EarnGold(
                CAMERA_GOLD_REWARD,
            ));
            UpgradeUpdateFlags::NONE
        } else {
            UpgradeUpdateFlags::NONE
        }
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Camera",
            crate::l10n::locale::Language::Korean => "카메라",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder
                    .static_text("Gain ")
                    .l10n(Word::Gold.name(), locale)
                    .with_gold_value(format!(" +{}", CAMERA_GOLD_REWARD))
                    .static_text(" when placing a face tower");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .static_text("그림 카드 타워를 배치 시 ")
                    .l10n(Word::Gold.name(), locale)
                    .with_gold_value(format!(" +{}", CAMERA_GOLD_REWARD));
            }
        }
    }
}

impl CameraUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Camera(CameraUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_legendary,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    CameraUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {
    use crate::game_state::{
        card::{Rank, Suit},
        upgrade::behavior::camera::CAMERA_GOLD_REWARD,
    };

    #[test]
    fn camera_grants_gold_when_face_tower_is_placed() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let initial_gold = game_state.gold;

        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::CameraUpgrade::into_upgrade(),
            None,
        ));

        let face_tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            Suit::Spades,
            Rank::King,
        );
        let face_tower = crate::game_state::tower::Tower::new(
            &face_tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.action(crate::game_state::GameStateAction::PlaceTower(
            Box::new(face_tower),
            None,
        ));

        assert_eq!(game_state.gold, initial_gold + CAMERA_GOLD_REWARD);
    }

    #[test]
    fn camera_does_not_grant_gold_for_number_tower() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let initial_gold = game_state.gold;

        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::CameraUpgrade::into_upgrade(),
            None,
        ));

        let number_tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            Suit::Spades,
            Rank::Ten,
        );
        let number_tower = crate::game_state::tower::Tower::new(
            &number_tower_template,
            crate::MapCoord::new(2, 0),
            game_state.now(),
        );
        game_state.action(crate::game_state::GameStateAction::PlaceTower(
            Box::new(number_tower),
            None,
        ));

        assert_eq!(game_state.gold, initial_gold);
    }
}
