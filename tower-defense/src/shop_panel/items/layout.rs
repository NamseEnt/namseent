use crate::game_state::upgrade::{Upgrade, UpgradeKind};
use crate::game_state::item::{Effect, Item};

use namui::{Px, Wh};

use super::description::ShopItemDescription;
use super::title::ShopItemTitle;

mod body;
mod imp;
pub(crate) mod render;

pub(crate) enum ShopSlotVariant<'a> {
    Item { item: &'a Item, cost: usize },
    Upgrade { upgrade: &'a Upgrade, cost: usize },
}

pub(crate) struct ShopItemLayoutParams<'a> {
    pub wh: Wh<Px>,
    pub name: ShopItemTitle,
    pub description: ShopItemDescription<'a>,
    pub cost: usize,
    pub available: bool,
    pub item_kind: Option<&'a Effect>,
    pub upgrade_kind: Option<&'a UpgradeKind>,
    pub rarity: crate::rarity::Rarity,
}

#[inline]
pub(crate) fn layout_params_for_slot<'a>(
    wh: Wh<Px>,
    variant: ShopSlotVariant<'a>,
    purchased: bool,
    disabled: bool,
    game_state: &crate::game_state::GameState,
) -> ShopItemLayoutParams<'a> {
    let available = !purchased && !disabled;
    let locale = game_state.text().locale();

    match variant {
        ShopSlotVariant::Item { item, cost } => make_item_params(wh, item, cost, available, locale),
        ShopSlotVariant::Upgrade { upgrade, cost } => {
            make_upgrade_params(wh, upgrade, cost, available, locale)
        }
    }
}

#[inline]
fn make_item_params<'a>(
    wh: Wh<Px>,
    item: &'a Item,
    cost: usize,
    available: bool,
    locale: crate::l10n::Locale,
) -> ShopItemLayoutParams<'a> {
    ShopItemLayoutParams {
        wh,
        name: ShopItemTitle::Effect {
            effect: item.effect.clone(),
            locale,
        },
        description: ShopItemDescription::Effect {
            effect: item.effect.clone(),
            locale,
        },
        cost,
        available,
        item_kind: Some(&item.effect),
        upgrade_kind: None,
        rarity: item.rarity,
    }
}

#[inline]
fn make_upgrade_params<'a>(
    wh: Wh<Px>,
    upgrade: &'a Upgrade,
    cost: usize,
    available: bool,
    locale: crate::l10n::Locale,
) -> ShopItemLayoutParams<'a> {
    ShopItemLayoutParams {
        wh,
        name: ShopItemTitle::Upgrade {
            upgrade_kind: upgrade.kind,
            locale,
        },
        description: ShopItemDescription::Upgrade {
            upgrade_kind: &upgrade.kind,
            locale,
        },
        cost,
        available,
        item_kind: None,
        upgrade_kind: Some(&upgrade.kind),
        rarity: upgrade.rarity,
    }
}

pub(crate) use render::ShopItemLayout;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::effect::Effect;
    use crate::game_state::item::Item;
    use crate::game_state::upgrade::{Upgrade, UpgradeKind};
    use crate::l10n::Locale;
    use namui::{OneZero, Wh, px};

    #[test]
    fn make_item_and_upgrade_params() {
        let wh = Wh::new(px(1.0), px(1.0));
        let locale = Locale::default();

        let item = Item {
            effect: Effect::Heal { amount: 1.0 },
            rarity: crate::rarity::Rarity::Common,
            value: OneZero::default(),
        };

        let params = make_item_params(wh, &item, 5, true, locale);
        assert!(params.available);
        assert_eq!(params.cost, 5);
        assert_eq!(params.rarity, item.rarity);

        let up = Upgrade {
            kind: UpgradeKind::GoldEarnPlus,
            rarity: crate::rarity::Rarity::Rare,
            value: OneZero::default(),
        };
        let params = make_upgrade_params(wh, &up, 3, false, locale);
        assert!(!params.available);
        assert_eq!(params.cost, 3);
        assert_eq!(params.rarity, up.rarity);
    }
}
