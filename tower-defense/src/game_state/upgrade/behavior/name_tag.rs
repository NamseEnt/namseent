use super::*;

const NAME_TAG_DAMAGE_BONUS_PCT: f32 = 2.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct NameTagUpgrade {
    pub damage_bonus_pct: f32,
    pub target_tower_id: Option<usize>,
}

impl UpgradeBehavior for NameTagUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::NAME_TAG,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
        }

    fn is_applicable(&self, context: &SelectedTowerContext) -> bool {
        match (context.tower_id, self.target_tower_id) {
            (SelectedTowerId::Placed(selected_tower_id), Some(target_tower_id)) => {
                selected_tower_id == target_tower_id
            }
            (SelectedTowerId::ToBePlaced, None) => true,
            _ => false,
        }
    }

    fn on_tower_placed(
        &mut self,
        _game_state: &mut GameState,
        tower: &Tower,
    ) -> UpgradeUpdateFlags {
        if self.target_tower_id.is_some() {
            return UpgradeUpdateFlags::NONE;
        }

        self.target_tower_id = Some(tower.id());
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        self.target_tower_id.map(|tower_id| {
            (
                TowerUpgradeTarget::TowerId { tower_id },
                self.damage_bonus_pct,
            )
        })
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Name Tag",
            crate::l10n::locale::Language::Korean => "이름표",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.text(match locale.language {
            crate::l10n::locale::Language::English => format!(
                "The next tower you place gains +{:.0}% damage",
                self.damage_bonus_pct * 100.0,
            ),
            crate::l10n::locale::Language::Korean => format!(
                "다음 배치하는 타워가 +{:.0}% 피해를 얻습니다",
                self.damage_bonus_pct * 100.0,
            ),
        });
    }
}

impl NameTagUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32) -> Upgrade {
        Upgrade::NameTag(NameTagUpgrade {
            damage_bonus_pct,
            target_tower_id: None,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    NameTagUpgrade::into_upgrade(NAME_TAG_DAMAGE_BONUS_PCT)
}
#[cfg(test)]
mod tests {

    use crate::game_state::upgrade::Upgrade;

    #[test]
    fn name_tag_applies_to_next_tower_and_consumes_it() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::NameTagUpgrade::into_upgrade(2.0),
            None,
        ));
        game_state.left_dice = 0;

        let template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Spades,
            crate::card::Rank::Ace,
        );
        game_state.action(crate::game_state::GameStateAction::StartPlacingTower(
            template,
        ));

        assert!(game_state.upgrade_state.upgrades.iter().any(|upgrade| {
            if let Upgrade::NameTag(upgrade) = &upgrade.upgrade {
                (upgrade.damage_bonus_pct - 2.0).abs() < f32::EPSILON
            } else {
                false
            }
        }));

        let placing_slot_id = game_state
            .hand
            .get_slot_id_by_index(0)
            .expect("expected tower slot to be present");
        let placed_template = support::first_hand_tower_template(&game_state);
        let tower = crate::game_state::tower::Tower::new(
            &placed_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.action(crate::game_state::GameStateAction::PlaceTower(
            Box::new(tower),
            None,
        ));
        game_state.hand.delete_slots(&[placing_slot_id]);

        let placed_tower = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower placed");
        support::assert_tower_cached_damage_mul(placed_tower, 3.0);
    }
}
