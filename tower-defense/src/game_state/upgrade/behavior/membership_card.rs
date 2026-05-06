use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MembershipCardUpgrade {
    pub pending_free_shop: bool,
}

impl UpgradeBehavior for MembershipCardUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.pending_free_shop {
            effects.free_shop_this_stage = true;
            self.pending_free_shop = false;
        }
    }

    fn on_stage_start(
        &mut self,
        stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        self.apply_on_stage_start(stage, effects);
        UpgradeUpdateFlags::RESOURCE
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
        use crate::game_state::upgrade::tests::support;
        use crate::game_state::GameFlow;
        use crate::game_state::effect::Effect;
        use crate::game_state::item::ItemKind;
        use crate::shop::ShopSlot;

        let mut game_state = support::create_mock_game_state();
        game_state.upgrade(crate::game_state::upgrade::MembershipCardUpgrade::into_upgrade());

        game_state.apply_stage_start(3);
        assert!(game_state.stage_modifiers.is_free_shop_this_stage());

        game_state.goto_selecting_tower();
        let initial_gold = game_state.gold;

        let slot_id = if let GameFlow::SelectingTower(flow) = &mut game_state.flow {
            if !flow
                .shop
                .slots
                .iter()
                .any(|slot_data| matches!(slot_data.slot, ShopSlot::Item { .. }) && !slot_data.purchased)
            {
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

        game_state.purchase_shop_item(slot_id);
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
