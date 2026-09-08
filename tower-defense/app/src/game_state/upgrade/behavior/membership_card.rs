use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MembershipCardUpgrade {
    pub pending_free_shop: bool,
}

impl UpgradePresentation for MembershipCardUpgrade {
    fn key(&self) -> &'static str {
        "membership_card"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::MEMBERSHIP_CARD)
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Membership Card",
            crate::l10n::locale::Language::Korean => "멤버십 카드",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Get a free shop next stage",
            crate::l10n::locale::Language::Korean => {
                "다음 스테이지 상점의 모든 상품이 무료가 됩니다"
            }
        });
    }
}

impl MembershipCardUpgrade {
    #[cfg(any(test, feature = "debug-tools"))]
    pub fn into_upgrade() -> Upgrade {
        Upgrade::MembershipCard(MembershipCardUpgrade {
            pending_free_shop: true,
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn membership_card_grants_free_shop_next_stage() {
        use crate::game_state::GameFlow;
        use crate::game_state::item::{ItemDiscriminants, LumpSugarItem};
        use crate::game_state::upgrade::tests::support;
        use crate::shop::ShopSlot;

        let mut game_state = support::create_mock_game_state();
        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            crate::game_state::upgrade::MembershipCardUpgrade::into_upgrade(),
            None,
        ));

        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::StartStage {
            stage: 3,
        });
        assert!(game_state.stage_modifiers.is_free_shop_this_stage());
        let initial_gold = game_state.gold;

        let slot_id = if let GameFlow::Shopping(flow) = &mut game_state.flow {
            if !flow.shop.slots.iter().any(|slot_data| {
                matches!(slot_data.slot, ShopSlot::Item { .. }) && !slot_data.purchased
            }) {
                flow.shop.push(ShopSlot::Item {
                    item: LumpSugarItem::standard().into_item(),
                    cost: 0,
                });
            }
            flow.shop
                .slots
                .iter()
                .find_map(|slot_data| match &slot_data.slot {
                    ShopSlot::Item { .. } if !slot_data.purchased => Some(slot_data.id),
                    _ => None,
                })
                .expect("expected at least one item slot in shop")
        } else {
            panic!("expected shopping flow");
        };

        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::PurchaseShopItem(slot_id),
        );
        assert_eq!(game_state.gold, initial_gold);
        assert!(
            game_state
                .items
                .iter()
                .any(|item| item.discriminant() == ItemDiscriminants::LumpSugar)
        );
    }
}
