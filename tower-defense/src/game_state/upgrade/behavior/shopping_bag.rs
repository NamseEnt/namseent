use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ShoppingBagUpgrade {
    pub damage_bonus_pct: f32,
    pub stacks: usize,
}

impl UpgradeBehavior for ShoppingBagUpgrade {
    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        if self.stacks > 0 {
            Some((
                TowerUpgradeTarget::Global,
                self.stacks as f32 * (self.damage_bonus_pct),
            ))
        } else {
            None
        }
    }

    fn on_item_bought(&mut self) -> UpgradeUpdateFlags {
        self.stacks += 1;
        UpgradeUpdateFlags::TOWER_STATS
    }
}

impl ShoppingBagUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32) -> Upgrade {
        Upgrade::ShoppingBag(ShoppingBagUpgrade {
            damage_bonus_pct,
            stacks: 0,
        })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    ShoppingBagUpgrade::into_upgrade(0.5)
}
#[cfg(test)]
mod tests {

    use crate::game_state::upgrade::*;

    #[test]
    fn shopping_bag_upgrade_activates_without_stacks() {
        let mut state = UpgradeState::default();
        state.upgrade(crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(0.5));

        assert!(
            state
                .upgrades
                .iter()
                .any(|u| { matches!(u, Upgrade::ShoppingBag(upgrade) if upgrade.stacks == 0) })
        );
    }

    #[test]
    fn shopping_bag_global_tower_damage_increases_with_stacks() {
        use crate::game_state::upgrade::tests::support;
        use crate::game_state::GameFlow;
        use crate::shop::ShopSlot;

        let mut gs = support::create_mock_game_state();
        gs.upgrade_state
            .upgrade(crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(0.5));

        let slot_id = if let GameFlow::SelectingTower(flow) = &mut gs.flow {
            match flow
                .shop
                .slots
                .iter()
                .find_map(|slot_data| match &slot_data.slot {
                    ShopSlot::Item { .. } if !slot_data.purchased => Some(slot_data.id),
                    _ => None,
                }) {
                Some(id) => id,
                None => {
                    let item = crate::game_state::item::Item {
                        kind: crate::game_state::item::ItemKind::LumpSugar,
                        effect: crate::game_state::effect::Effect::ExtraDice,
                    };
                    let cost = 0;
                    flow.shop.push(ShopSlot::Item { item, cost });
                    flow.shop.slots.last().unwrap().id
                }
            }
        } else {
            panic!("expected selecting tower flow");
        };

        gs.purchase_shop_item(slot_id);

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            crate::card::Suit::Hearts,
            crate::card::Rank::Two,
        );
        gs.goto_placing_tower(tower_template);

        let placed_template = support::first_hand_tower_template(&gs);
        let tower = crate::game_state::tower::Tower::new(
            &placed_template,
            crate::MapCoord::new(0, 0),
            gs.now(),
        );
        gs.place_tower(tower);

        let placed_tower = gs.towers.iter().next().expect("expected tower placed");
        support::assert_tower_cached_damage_mul(placed_tower, 1.5);
    }
}
