use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct IceCreamUpgrade {
    pub damage_bonus_pct: f32,
    pub waves_remaining: usize,
}

impl UpgradeBehavior for IceCreamUpgrade {
    fn on_stage_start(
        &mut self,
        _stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        if self.waves_remaining > 0 {
            effects.damage_multiplier += self.damage_bonus_pct;
        }
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn on_upgrade_acquired_mut(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.waves_remaining > 0 {
            Some((TowerUpgradeTarget::Global, self.damage_bonus_pct))
        } else {
            None
        }
    }

    fn on_stage_end(
        &mut self,
        _perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        if self.waves_remaining > 0 {
            self.waves_remaining -= 1;
            (0, UpgradeUpdateFlags::TOWER_STATS)
        } else {
            (0, UpgradeUpdateFlags::NONE)
        }
    }

    fn l10n_name<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Ice Cream",
            crate::l10n::locale::Language::Korean => "아이스크림",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                let waves_text =
                    Box::leak(format!("{} waves", self.waves_remaining).into_boxed_str());
                builder
                    .static_text("Damage ")
                    .with_icon_bold(crate::icon::IconKind::Damage, format!("X{:.1}", 1.0 + self.damage_bonus_pct))
                    .static_text(" for ")
                    .static_text(waves_text)
            }
            crate::l10n::locale::Language::Korean => {
                let waves_text =
                    Box::leak(format!("{}웨이브", self.waves_remaining).into_boxed_str());
                builder
                    .static_text("다음 ")
                    .static_text(waves_text)
                    .static_text(" 동안 피해 ")
                    .with_icon_bold(crate::icon::IconKind::Damage, format!("X{:.1}", 1.0 + self.damage_bonus_pct))
            }
        };
    }
}

impl IceCreamUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32, waves_remaining: usize) -> Upgrade {
        Upgrade::IceCream(IceCreamUpgrade {
            damage_bonus_pct,
            waves_remaining,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    IceCreamUpgrade::into_upgrade(2.0, 5)
}
#[cfg(test)]
mod tests {

    #[test]
    fn ice_cream_effect_applies_to_placed_tower_and_expires_after_waves() {
        use crate::game_state::upgrade::tests::support;
        use crate::game_state::flow::DefenseFlow;
        use crate::game_state::tower::TowerTemplate;

        let mut game_state = support::create_mock_game_state();
        game_state.flow = crate::game_state::GameFlow::Defense(DefenseFlow::new(&game_state));
        let upgrade = crate::game_state::upgrade::IceCreamUpgrade::into_upgrade(2.0, 2);
        game_state.upgrade(upgrade);

        let tower_template = TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Hearts,
            crate::card::Rank::Two,
        );
        let tower = crate::game_state::tower::Tower::new(
            &tower_template,
            crate::MapCoord::new(0, 0),
            game_state.now(),
        );
        game_state.place_tower(tower);

        let placed_tower = game_state
            .towers
            .iter()
            .next()
            .expect("expected tower placed");
        let base_damage = placed_tower.calculate_projectile_damage(&[], 1.0);
        let boosted_damage = placed_tower.cached_upgrade_damage();

        assert!(boosted_damage > base_damage);
        assert!((boosted_damage / base_damage - 3.0).abs() < f32::EPSILON);

        crate::game_state::tick::defense_end::check_defense_end(&mut game_state);
        let first_tower_after_second_wave = game_state
            .towers
            .iter()
            .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
            .expect("expected tower to still exist after first stage");
        let second_boosted_damage = first_tower_after_second_wave.cached_upgrade_damage();
        assert!((second_boosted_damage / base_damage - 3.0).abs() < f32::EPSILON);

        game_state.flow = crate::game_state::GameFlow::Defense(DefenseFlow::new(&game_state));
        crate::game_state::tick::defense_end::check_defense_end(&mut game_state);

        let expired_tower = game_state
            .towers
            .iter()
            .find(|tower| tower.left_top == crate::MapCoord::new(0, 0))
            .expect("expected tower to still exist after second stage");
        let expired_damage = expired_tower.cached_upgrade_damage();

        assert!((expired_damage / base_damage - 1.0).abs() < f32::EPSILON);
    }
}

