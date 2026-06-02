use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const PEA_HP_PLUS: f32 = 10.0;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PeaUpgrade;

impl UpgradeBehavior for PeaUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::PEA,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::HEAL_TO_FULL
    }

    fn max_hp_plus(&self) -> f32 {
        PEA_HP_PLUS
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Pea",
            crate::l10n::locale::Language::Korean => "완두콩",
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
                    .static_text("Increase max ")
                    .with_health_value("HP")
                    .static_text(" by ")
                    .with_health_value(format!("{:.0}", PEA_HP_PLUS))
                    .static_text(" and heal to full");
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .static_text("최대 ")
                    .with_health_value("체력")
                    .static_text("이 ")
                    .with_health_value(format!("{:.0}", PEA_HP_PLUS))
                    .static_text(" 증가하고 즉시 회복합니다");
            }
        };
    }
}

impl PeaUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Pea(PeaUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PeaUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn pea_increases_max_hp_and_fully_heals() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.hp = 1.0;

        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::PeaUpgrade::into_upgrade(),
            None,
        ));

        assert_eq!(game_state.upgrade_state.max_hp_plus(), 10);
        assert!(
            (game_state.max_hp() - (game_state.config.player.max_hp + 10.0)).abs() < f32::EPSILON
        );
        assert!((game_state.hp - game_state.max_hp()).abs() < f32::EPSILON);
    }
}
