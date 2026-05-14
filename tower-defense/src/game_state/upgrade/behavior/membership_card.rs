use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MembershipCardUpgrade {
    pub pending_free_shop: bool,
}

impl UpgradeBehavior for MembershipCardUpgrade {
    fn on_stage_start(&mut self, game_state: &mut GameState, _stage: usize) -> UpgradeUpdateFlags {
        if self.pending_free_shop {
            game_state.stage_modifiers.set_free_shop_this_stage(true);
            self.pending_free_shop = false;
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
            crate::l10n::locale::Language::Korean => "다음 스테이지 상점이 무료가 됩니다",
        });
    }
}

impl MembershipCardUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::MembershipCard(MembershipCardUpgrade {
            pending_free_shop: true,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    MembershipCardUpgrade::into_upgrade()
}
#[cfg(test)]
mod tests {

    #[test]
    fn membership_card_grants_free_shop_next_stage() {
        use crate::game_state::GameFlow;
        use crate::game_state::effect::Effect;
        use crate::game_state::item::ItemKind;
        use crate::game_state::upgrade::tests::support;
        use crate::shop::ShopSlot;

        let mut game_state = support::create_mock_game_state();
        game_state.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::MembershipCardUpgrade::into_upgrade(),
            None,
        ));

        game_state.action(crate::game_state::GameStateAction::StartStage { stage: 3 });
        assert!(game_state.stage_modifiers.is_free_shop_this_stage());
        let initial_gold = game_state.gold;

        let slot_id = if let GameFlow::SelectingTower(flow) = &mut game_state.flow {
            if !flow.shop.slots.iter().any(|slot_data| {
                matches!(slot_data.slot, ShopSlot::Item { .. }) && !slot_data.purchased
            }) {
                flow.shop.push(ShopSlot::Item {
                    item: crate::game_state::item::Item {
                        kind: ItemKind::LumpSugar,
                        effect: Effect::ExtraDice,
                    },
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
            panic!("expected selecting tower flow");
        };

        game_state.action(crate::game_state::GameStateAction::PurchaseShopItem(
            slot_id,
        ));
        assert_eq!(game_state.gold, initial_gold);
        assert!(
            game_state
                .items
                .iter()
                .any(|item| item.kind == ItemKind::LumpSugar)
                || game_state
                    .items
                    .iter()
                    .any(|item| item.effect == Effect::ExtraDice)
        );
    }
}
