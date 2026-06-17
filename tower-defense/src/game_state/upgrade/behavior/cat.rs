use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CatUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for CatUpgrade {
    fn key(&self) -> &'static str {
        "cat"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::CAT,
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
            crate::theme::palette::YELLOW,
        ))
    }

    fn on_monster_death(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        game_state.gold += self.add;
        UpgradeUpdateFlags::NONE
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Cat",
            crate::l10n::locale::Language::Korean => "고양이",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Gain ")
                .l10n(Word::Gold.name(), locale)
                .with_gold_value(format!(" +{}", self.add))
                .static_text(" on monster kills"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("적 처치 시 ")
                .l10n(Word::Gold.name(), locale)
                .with_gold_value(format!(" +{}", self.add)),
        };
    }
}

impl CatUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Cat(CatUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_epic,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    CatUpgrade::into_upgrade(1)
}
