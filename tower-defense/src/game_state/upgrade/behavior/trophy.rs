use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TrophyUpgrade {
    pub perfect_clear_stacks: usize,
}

impl UpgradeBehavior for TrophyUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::TROPHY,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
        }

    fn on_stage_end(
        &mut self,
        _game_state: &mut GameState,
        perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> UpgradeUpdateFlags {
        if perfect_clear {
            self.perfect_clear_stacks += 1;
        }
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        if self.perfect_clear_stacks > 0 {
            Some((
                TowerUpgradeTarget::Global,
                self.perfect_clear_stacks as f32 * (super::super::TROPHY_DAMAGE_MULTIPLIER - 1.0),
            ))
        } else {
            None
        }
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
        let current_bonus =
            self.perfect_clear_stacks as f32 * (super::super::TROPHY_DAMAGE_MULTIPLIER - 1.0);
        builder.text(match locale.language {
            crate::l10n::locale::Language::English => format!(
                "Perfect clears increase all towers' damage by {:.0}% each wave (currently +{:.0}%)",
                (super::super::TROPHY_DAMAGE_MULTIPLIER - 1.0) * 100.0,
                current_bonus * 100.0,
            ),
            crate::l10n::locale::Language::Korean => format!(
                "웨이브를 퍼펙트 클리어할 때마다 모든 타워의 공격력이 {:.0}% 증가합니다 (현재 +{:.0}%)",
                (super::super::TROPHY_DAMAGE_MULTIPLIER - 1.0) * 100.0,
                current_bonus * 100.0,
            ),
        });
    }
}

impl TrophyUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Trophy(TrophyUpgrade {
            perfect_clear_stacks: 0,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    TrophyUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn trophy_uses_perfect_clear_stacks_for_global_damage() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::TrophyUpgrade::into_upgrade(),
            None,
        ));

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Hearts,
            crate::card::Rank::Two,
        );
        let tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        let before_damage = tower.calculate_projectile_damage(&[], 1.0);

        game_state.action(crate::game_state::GameStateAction::StageEnd {
            perfect_clear: true,
            gold: 0,
            item_count: 0,
        });
        game_state.action(crate::game_state::GameStateAction::StageEnd {
            perfect_clear: true,
            gold: 0,
            item_count: 0,
        });

        let upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();
        let after_damage = tower.calculate_projectile_damage(&upgrade_bonuses, 1.0);

        assert!(after_damage > before_damage);
        assert!((after_damage / before_damage - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn trophy_perfect_clear_increments_perfect_clear_stacks() {
        use crate::game_state::upgrade::tests::support;

        let mut gs = support::create_mock_game_state();
        gs.flow =
            crate::game_state::GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
        gs.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::TrophyUpgrade::into_upgrade(),
            None,
        ));

        crate::game_state::tick::defense_end::check_defense_end(&mut gs);

        assert!(gs.upgrade_state.upgrades.iter().any(|upgrade| {
            matches!(upgrade.upgrade, crate::game_state::upgrade::Upgrade::Trophy(trophy) if trophy.perfect_clear_stacks == 1)
        }));
    }
}
