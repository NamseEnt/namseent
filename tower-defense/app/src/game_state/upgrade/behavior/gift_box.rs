use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct GiftBoxUpgrade {
    pub(crate) add: usize,
}

impl UpgradePresentation for GiftBoxUpgrade {
    fn key(&self) -> &'static str {
        "gift_box"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::GIFT_BOX)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        vec![crate::thumbnail::ThumbnailOverlay::right_bottom(
            format!("{}", self.add),
            crate::theme::palette::YELLOW,
        )]
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Gift Box",
            crate::l10n::locale::Language::Korean => "선물 상자",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .l10n(Word::Gold.name(), locale)
                .with_bold(format!(" +{}", self.add))
                .static_text(" per ")
                .l10n(Word::Item.name(), locale)
                .static_text(" at the end of each stage"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("스테이지 종료 시 보유한 ")
                .l10n(Word::Item.name(), locale)
                .static_text(" 1개당 ")
                .l10n(Word::Gold.name(), locale)
                .with_bold(format!(" +{}", self.add)),
        };
    }
}

impl GiftBoxUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::GiftBox(GiftBoxUpgrade { add: 10 })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn gift_box_awards_gold_per_item_on_stage_end() {
        use crate::game_state::upgrade::tests::support;

        let mut gs = support::create_mock_game_state();
        gs.flow =
            crate::game_state::GameFlow::Defense(crate::game_state::flow::DefenseFlow::new(&gs));
        gs.items = vec![
            crate::game_state::item::LumpSugarItem::standard()
                .into_item()
                .with_unique_id(),
            crate::game_state::item::LumpSugarItem::standard()
                .into_item()
                .with_unique_id(),
        ];
        gs.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::GiftBoxUpgrade::into_upgrade(),
            None,
        ));

        support::check_defense_end_for_test(&mut gs);

        assert_eq!(gs.gold, gs.config.player.starting_gold + 20);
    }
}
