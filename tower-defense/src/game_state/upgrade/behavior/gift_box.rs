use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

const GIFT_BOX_GOLD_PER_ITEM: usize = 10;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct GiftBoxUpgrade;

impl UpgradeBehavior for GiftBoxUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::GIFT_BOX,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn on_stage_end(
        &mut self,
        game_state: &mut GameState,
        _perfect_clear: bool,
        _gold: usize,
        item_count: usize,
    ) -> UpgradeUpdateFlags {
        let bonus_gold = item_count * GIFT_BOX_GOLD_PER_ITEM;
        if bonus_gold > 0 {
            game_state.action(crate::game_state::GameStateAction::EarnGold(bonus_gold));
            UpgradeUpdateFlags::NONE
        } else {
            UpgradeUpdateFlags::NONE
        }
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
                .static_text("Earn ")
                .with_gold_value(format!("{}", GIFT_BOX_GOLD_PER_ITEM))
                .static_text(" gold per item at the end of each stage"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("스테이지 종료 시 보유한 아이템당 ")
                .with_gold_value(format!("{}골드", GIFT_BOX_GOLD_PER_ITEM))
                .static_text("를 얻습니다"),
        };
    }
}

impl GiftBoxUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::GiftBox(GiftBoxUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    GiftBoxUpgrade::into_upgrade()
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
            crate::game_state::item::LumpSugarItem::standard().into_item(),
            crate::game_state::item::LumpSugarItem::standard().into_item(),
        ];
        gs.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::GiftBoxUpgrade::into_upgrade(),
            None,
        ));

        crate::game_state::tick::defense_end::check_defense_end(&mut gs);

        assert_eq!(gs.gold, gs.config.player.starting_gold + 20);
    }
}
