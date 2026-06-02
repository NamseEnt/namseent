use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FangUpgrade {
    add: usize,
}

impl UpgradeBehavior for FangUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::FANG,
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
            &format!("{}", self.add),
            crate::theme::palette::RED,
        ))
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        for upgrade in game_state.upgrade_state.upgrades.iter_mut() {
            if let Upgrade::Fang(upgrade) = &mut upgrade.upgrade {
                upgrade.add += self.add;
                return UpgradeUpdateFlags::NONE;
            }
        }

        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::NONE
    }

    fn on_monster_death(&mut self, _game_state: &mut GameState) -> bool {
        true
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Fang",
            crate::l10n::locale::Language::Korean => "송곳니",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .with_health_value(format!("HP +{}", self.add))
                .static_text(" when a monster dies"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("적 처시 시 ")
                .with_health_value(format!("체력 +{}", self.add)),
        };
    }
}

impl FangUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Fang(FangUpgrade { add: 1 })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_rare,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    FangUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    use crate::game_state::{Monster, monster_spawn, tick};

    #[test]
    fn fang_recovers_hp_when_monster_dies() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.hp = 10.0;

        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::FangUpgrade::into_upgrade(),
            None,
        ));

        let (template_queue, _) =
            monster_spawn::monster_template_queue_table(1, &game_state.config);
        let template = template_queue
            .front()
            .expect("expected at least one monster template in stage 1")
            .clone();
        let target = Monster::new(&template, game_state.route.clone(), game_state.now(), 1.0);
        let target_xy = target.center_xy_tile();
        let now = game_state.now();

        game_state.monsters.push(target);
        tick::monster_death::handle_monster_death(&mut game_state, 0, target_xy, now);

        assert!((game_state.hp - 11.0).abs() < f32::EPSILON);
    }

    #[test]
    fn fang_recovery_respects_current_max_hp() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.hp = game_state.max_hp();

        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::FangUpgrade::into_upgrade(),
            None,
        ));

        let (template_queue, _) =
            monster_spawn::monster_template_queue_table(1, &game_state.config);
        let template = template_queue
            .front()
            .expect("expected at least one monster template in stage 1")
            .clone();
        let target = Monster::new(&template, game_state.route.clone(), game_state.now(), 1.0);
        let target_xy = target.center_xy_tile();
        let now = game_state.now();

        game_state.monsters.push(target);
        tick::monster_death::handle_monster_death(&mut game_state, 0, target_xy, now);

        assert!((game_state.hp - game_state.max_hp()).abs() < f32::EPSILON);
    }
}
