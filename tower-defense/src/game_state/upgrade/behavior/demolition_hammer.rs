use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DemolitionHammerUpgrade {
    pub damage_bonus_pct: f32,
    pub removed_tower_count: usize,
    pub stored_damage_bonus: f32,
}

impl UpgradeBehavior for DemolitionHammerUpgrade {
    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state.upgrade_state.upgrades.push(self.into());
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        if self.stored_damage_bonus > 0.0 {
            Some((TowerUpgradeTarget::Global, self.stored_damage_bonus))
        } else {
            None
        }
    }

    fn on_tower_removed(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        self.removed_tower_count += 1;
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn on_stage_end(
        &mut self,
        _game_state: &mut GameState,
        _perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> UpgradeUpdateFlags {
        if self.removed_tower_count == 0 {
            return UpgradeUpdateFlags::NONE;
        }

        self.stored_damage_bonus += self.damage_bonus_pct * self.removed_tower_count as f32;
        self.removed_tower_count = 0;
        UpgradeUpdateFlags::TOWER_STATS
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
        builder.text(match locale.language {
            crate::l10n::locale::Language::English => {
                let current = format!("+{:.0}%", self.stored_damage_bonus * 100.0);
                format!(
                    "Removing towers increases next stage damage (currently {})",
                    current
                )
            }
            crate::l10n::locale::Language::Korean => {
                let current = format!("+{:.0}%", self.stored_damage_bonus * 100.0);
                format!(
                    "타워를 제거하면 다음 스테이지 피해가 증가합니다 (현재 {})",
                    current
                )
            }
        });
    }
}

impl DemolitionHammerUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32) -> Upgrade {
        Upgrade::DemolitionHammer(DemolitionHammerUpgrade {
            damage_bonus_pct,
            removed_tower_count: 0,
            stored_damage_bonus: 0.0,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    DemolitionHammerUpgrade::into_upgrade(2.0)
}
#[cfg(test)]
mod tests {

    #[test]
    fn demolition_hammer_stage_end_stores_removed_tower_damage_bonus() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let upgrade = crate::game_state::upgrade::DemolitionHammerUpgrade::into_upgrade(2.0);
        game_state.action(crate::game_state::GameStateAction::Upgrade(upgrade, None));

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Hearts,
            crate::card::Rank::Two,
        );
        let first_tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        let second_tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(2, 0),
            game_state.now(),
        );

        game_state.action(crate::game_state::GameStateAction::PlaceTower(Box::new(
            first_tower,
        )));
        game_state.action(crate::game_state::GameStateAction::PlaceTower(Box::new(
            second_tower,
        )));

        let first_id = game_state
            .towers
            .iter()
            .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
            .expect("expected first tower placed")
            .id();
        let second_id = game_state
            .towers
            .iter()
            .find(|tower| tower.left_top == crate::MapCoord::new(2, 0))
            .expect("expected second tower placed")
            .id();
        assert!(game_state.action(crate::game_state::GameStateAction::RemoveTower(first_id)));
        assert!(game_state.action(crate::game_state::GameStateAction::RemoveTower(second_id)));

        game_state.action(crate::game_state::GameStateAction::StageEnd {
            perfect_clear: false,
            gold: game_state.gold,
            item_count: game_state.items.len(),
        });

        let upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();

        assert_eq!(upgrade_bonuses.len(), 1);
        assert!((upgrade_bonuses[0].bonus_pct - 4.0).abs() < f32::EPSILON);
        assert!(matches!(
            game_state.upgrade_state.upgrades[0],
            crate::game_state::upgrade::Upgrade::DemolitionHammer(..)
        ));
        if let crate::game_state::upgrade::Upgrade::DemolitionHammer(upgrade) =
            game_state.upgrade_state.upgrades[0]
        {
            assert_eq!(upgrade.removed_tower_count, 0);
        }
    }

    #[test]
    fn demolition_hammer_uses_configured_damage_multiplier() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let upgrade = crate::game_state::upgrade::DemolitionHammerUpgrade::into_upgrade(1.25);
        game_state.action(crate::game_state::GameStateAction::Upgrade(upgrade, None));

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Hearts,
            crate::card::Rank::Two,
        );
        let first_tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        let second_tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(2, 0),
            game_state.now(),
        );

        game_state.action(crate::game_state::GameStateAction::PlaceTower(Box::new(
            first_tower,
        )));
        game_state.action(crate::game_state::GameStateAction::PlaceTower(Box::new(
            second_tower,
        )));

        let first_id = game_state
            .towers
            .iter()
            .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
            .expect("expected first tower placed")
            .id();
        let second_id = game_state
            .towers
            .iter()
            .find(|tower| tower.left_top == crate::MapCoord::new(2, 0))
            .expect("expected second tower placed")
            .id();
        assert!(game_state.action(crate::game_state::GameStateAction::RemoveTower(first_id)));
        assert!(game_state.action(crate::game_state::GameStateAction::RemoveTower(second_id)));

        game_state.action(crate::game_state::GameStateAction::StageEnd {
            perfect_clear: false,
            gold: game_state.gold,
            item_count: game_state.items.len(),
        });

        let upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();

        assert_eq!(upgrade_bonuses.len(), 1);
        assert!((upgrade_bonuses[0].bonus_pct - 2.5).abs() < f32::EPSILON);
    }
}
