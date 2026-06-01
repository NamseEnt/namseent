use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CrockUpgrade {
    pub current_step: usize,
}

const CROCK_GOLD_PER_DAMAGE: usize = 100;
const CROCK_DAMAGE_PER_STEP: f32 = 0.25;

impl CrockUpgrade {
    fn current_damage_bonus(&self) -> f32 {
        self.current_step as f32 * CROCK_DAMAGE_PER_STEP
    }

    fn update_step_from_gold(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        let next_step = game_state.gold / CROCK_GOLD_PER_DAMAGE;
        if next_step == self.current_step {
            return UpgradeUpdateFlags::NONE;
        }

        self.current_step = next_step;
        UpgradeUpdateFlags::TOWER_STATS
    }
}

impl UpgradeBehavior for CrockUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::CROCK,
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
            &format!("+{:.0}%", self.current_damage_bonus() * 100.0),
            crate::theme::palette::RED,
        ))
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        if self.current_step > 0 {
            Some((TowerUpgradeTarget::Global, self.current_damage_bonus()))
        } else {
            None
        }
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        let mut upgrade = self;
        let flags = upgrade.update_step_from_gold(game_state);
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(upgrade).with_unique_id());
        flags
    }

    fn on_gold_earned(&mut self, game_state: &mut GameState, _earned: usize) -> UpgradeUpdateFlags {
        self.update_step_from_gold(game_state)
    }

    fn on_gold_spent(&mut self, game_state: &mut GameState, _spent: usize) -> UpgradeUpdateFlags {
        self.update_step_from_gold(game_state)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Crock",
            crate::l10n::locale::Language::Korean => "항아리",
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
                    .with_damage_value(format!("+{:.0}%", CROCK_DAMAGE_PER_STEP * 100.0))
                    .static_text(" ")
                    .with_damage_text("damage")
                    .static_text(" for every ")
                    .text(CROCK_GOLD_PER_DAMAGE.to_string())
                    .static_text(" gold (currently ")
                    .with_damage_value(format!("+{:.0}%", self.current_damage_bonus() * 100.0))
                    .static_text(")");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .static_text("현재 보유 골드 ")
                    .text(CROCK_GOLD_PER_DAMAGE.to_string())
                    .static_text("당 모든 타워 ")
                    .with_damage_text("피해 ")
                    .with_damage_value(format!("+{:.0}%", CROCK_DAMAGE_PER_STEP * 100.0))
                    .static_text(" (현재 ")
                    .with_damage_value(format!("+{:.0}%", self.current_damage_bonus() * 100.0))
                    .static_text(")");
            }
        }
    }
}

impl CrockUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Crock(CrockUpgrade { current_step: 0 })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    CrockUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn crock_increases_tower_damage_for_existing_towers() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::FullHouse,
            crate::card::Suit::Hearts,
            crate::card::Rank::Queen,
        );
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
            .expect("expected tower to be placed")
            .id();
        let before_damage = game_state
            .towers
            .iter()
            .find(|tower| tower.id() == tower_id)
            .expect("expected placed tower")
            .calculate_projectile_damage(&[], 1.0);

        game_state.gold = 2500;
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::CrockUpgrade::into_upgrade(),
            None,
        ));

        let after_damage = {
            let tower = game_state
                .towers
                .iter()
                .find(|tower| tower.id() == tower_id)
                .expect("expected placed tower");
            let upgrade_bonuses = game_state.upgrade_state.tower_upgrade_damage_bonuses();
            tower.calculate_projectile_damage(&upgrade_bonuses, 1.0)
        };

        assert!(before_damage > 0.0);
        assert!(after_damage > before_damage);
    }
}
