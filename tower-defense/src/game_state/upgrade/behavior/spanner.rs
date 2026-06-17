use crate::l10n::word::Word;

use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SpannerUpgrade;

impl UpgradeBehavior for SpannerUpgrade {
    fn key(&self) -> &'static str {
        "spanner"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::SPANNER,
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
        UpgradeUpdateFlags::CACHE | UpgradeUpdateFlags::REVISION
    }

    fn clear_shield_on_stage_start(&self) -> bool {
        false
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Spanner",
            crate::l10n::locale::Language::Korean => "스패너",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::Language::Korean => builder
                .static_text("스테이지 종료 시 ")
                .l10n(Word::Shield.name(), locale)
                .static_text("이 사라지지 않습니다"),
            crate::l10n::Language::English => builder
                .static_text("Keep ")
                .l10n(Word::Shield.name(), locale)
                .static_text(" on stage ends"),
        };
    }
}

impl SpannerUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Spanner(SpannerUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_epic,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    SpannerUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn spanner_keeps_shield_across_stage_transition() {
        use crate::game_state::upgrade::tests::support;

        let mut gs = support::create_mock_game_state();
        gs.shield = 50.0;
        gs.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::SpannerUpgrade::into_upgrade(),
            None,
        ));

        gs.action(crate::game_state::GameStateAction::StartStage { stage: gs.stage });

        assert_eq!(gs.shield, 50.0);
    }
}
