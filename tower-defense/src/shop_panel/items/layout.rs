use crate::game_state::item::{Item, ItemKind};
use crate::game_state::upgrade::Upgrade;

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
    pub name: ShopItemTitle<'a>,
    pub description: ShopItemDescription<'a>,
    pub cost: usize,
    pub available: bool,
    pub item_kind: Option<&'a ItemKind>,
    pub upgrade: Option<&'a Upgrade>,
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
        name: ShopItemTitle::Item {
            item_kind: item.kind.clone(),
            locale,
        },
        description: ShopItemDescription::Item { item, locale },
        cost,
        available,
        item_kind: Some(&item.kind),
        upgrade: None,
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
        name: ShopItemTitle::Upgrade { upgrade, locale },
        description: ShopItemDescription::Upgrade { upgrade, locale },
        cost,
        available,
        item_kind: None,
        upgrade: Some(upgrade),
    }
}

pub(crate) use render::ShopItemLayout;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::effect::Effect;
    use crate::game_state::item::Item;
    use crate::l10n::Locale;
    use namui::{Wh, px};

    #[test]
    fn make_item_and_upgrade_params() {
        let wh = Wh::new(px(1.0), px(1.0));
        let locale = Locale::default();

        let item = Item {
            kind: crate::game_state::item::ItemKind::RiceBall,
            effect: Effect::Heal { amount: 1.0 },
        };

        let params = make_item_params(wh, &item, 5, true, locale);
        assert!(params.available);
        assert_eq!(params.cost, 5);

        let up = crate::game_state::upgrade::CatUpgrade::into_upgrade(1);
        let params = make_upgrade_params(wh, &up, 3, false, locale);
        assert!(!params.available);
        assert_eq!(params.cost, 3);
    }
}
