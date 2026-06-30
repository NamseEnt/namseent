use super::*;
use crate::l10n::{rich_text_helpers::RichTextHelpers, word::Word};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ShoppingBagUpgrade {
    pub damage_bonus_pct: f32,
    pub stacks: usize,
}

impl UpgradeBehavior for ShoppingBagUpgrade {
    fn key(&self) -> &'static str {
        "shopping_bag"
    }

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::SHOPPING_BAG,
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
            &format!("{:.0}%", self.stacks as f32 * self.damage_bonus_pct * 100.0),
            crate::theme::palette::RED,
        ))
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        if self.stacks > 0 {
            Some((
                TowerUpgradeTarget::Global,
                self.stacks as f32 * (self.damage_bonus_pct),
            ))
        } else {
            None
        }
    }

    fn on_item_bought(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        self.stacks += 1;
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn is_applicable(&self, _context: &SelectedTowerContext) -> bool {
        if self.stacks == 0 {
            return false;
        }
        true
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Shopping Bag",
            crate::l10n::locale::Language::Korean => "쇼핑백",
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
                    .static_text("Each purchased ")
                    .l10n(Word::Item.name(), locale)
                    .static_text("/")
                    .l10n(Word::Treasure.name(), locale)
                    .static_text(" increases all towers' ")
                    .with_damage_value(format!("damage +{:.0}%", self.damage_bonus_pct * 100.0));
            }
            crate::l10n::locale::Language::Korean => {
                builder
                    .l10n(Word::Item.name(), locale)
                    .static_text("/")
                    .l10n(Word::Treasure.name(), locale)
                    .static_text(" 을 구매할 때마다 모든 타워 ")
                    .with_damage_value(format!("데미지 +{:.0}%", self.damage_bonus_pct * 100.0));
            }
        }
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

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_legendary,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    ShoppingBagUpgrade::into_upgrade(0.5)
}
#[cfg(test)]
mod tests {

    use crate::game_state::{card::{Rank, Suit}, upgrade::*};

    #[test]
    fn shopping_bag_upgrade_activates_without_stacks() {
        let state = UpgradeState::with_upgrades(vec![
            crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(0.5),
        ]);

        assert!(state.upgrades.iter().any(|u| {
            matches!(u.upgrade, Upgrade::ShoppingBag(upgrade) if upgrade.stacks == 0)
        }));
    }

    #[test]
    fn shopping_bag_global_tower_damage_increases_with_stacks() {
        use crate::game_state::GameFlow;
        use crate::game_state::upgrade::tests::support;
        use crate::shop::ShopSlot;

        let mut gs = support::create_mock_game_state();
        gs.action(crate::game_state::GameStateAction::Upgrade(
            crate::game_state::upgrade::ShoppingBagUpgrade::into_upgrade(0.5),
            None,
        ));

        let slot_id = if let GameFlow::Shopping(flow) = &mut gs.flow {
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
                    let item = crate::game_state::item::LumpSugarItem::standard().into_item();
                    let cost = 0;
                    flow.shop.push(ShopSlot::Item { item, cost });
                    flow.shop.slots.last().unwrap().id
                }
            }
        } else {
            panic!("expected shopping flow");
        };

        gs.action(crate::game_state::GameStateAction::PurchaseShopItem(
            slot_id,
        ));

        let tower_template = crate::game_state::tower::TowerTemplate::new(
            crate::game_state::tower::TowerKind::High,
            Suit::Hearts,
            Rank::Two,
        );
        gs.action(crate::game_state::GameStateAction::StartPlacingTower(
            tower_template,
        ));

        let placed_template = support::first_hand_tower_template(&gs);
        let tower = crate::game_state::tower::Tower::new(
            &placed_template,
            crate::MapCoord::new(0, 0),
            gs.now(),
        );
        gs.action(crate::game_state::GameStateAction::PlaceTower(
            Box::new(tower),
            None,
        ));

        let placed_tower = gs.towers.iter().next().expect("expected tower placed");
        support::assert_tower_cached_damage_mul(placed_tower, 1.5);
    }
}
