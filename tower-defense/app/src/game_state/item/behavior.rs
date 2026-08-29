use crate::{Health, Shield};
use enum_dispatch::enum_dispatch;
use namui::*;
use std::sync::atomic::{AtomicU64, Ordering};

mod bread;
mod candy;
mod cannoli;
mod cookie;
mod donut;
mod gimbap;
mod lump_sugar;
mod lunch_box;
mod milk;
mod rice_ball;
mod rubber_cone;

pub use bread::*;
pub use candy::*;
pub use cannoli::*;
pub use cookie::*;
pub use donut::*;
pub use gimbap::*;
pub use lump_sugar::*;
pub use lunch_box::*;
pub use milk::*;
pub use rice_ball::*;
pub use rubber_cone::*;

#[enum_dispatch]
pub trait ItemBehavior {
    fn key(&self) -> &'static str;

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    );

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    );

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_>;

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![self.tooltip_section(locale)]
    }

    fn tooltip_section(&self, locale: crate::l10n::Locale) -> crate::tooltip::TooltipSection<'_> {
        crate::tooltip::TooltipSection {
            title: Some(crate::tooltip::SectionText {
                key: format!("item:{}:name", self.key()),
                apply: Box::new(move |builder| {
                    self.l10n_name(builder, &locale);
                }),
            }),
            body: crate::tooltip::SectionText {
                key: format!("item:{}:desc", self.key()),
                apply: Box::new(move |builder| {
                    self.l10n_description(builder, &locale);
                }),
            },
        }
    }
}

#[enum_dispatch(ItemBehavior)]
#[derive(Debug, Clone, PartialEq, State, strum_macros::EnumDiscriminants)]
#[strum_discriminants(derive(State, strum_macros::EnumIter, strum_macros::AsRefStr))]
pub enum Item {
    Bread(BreadItem),
    Candy(CandyItem),
    Cannoli(CannoliItem),
    Cookie(CookieItem),
    Donut(DonutItem),
    RiceBall(RiceBallItem),
    LunchBox(LunchBoxItem),
    LumpSugar(LumpSugarItem),
    Milk(MilkItem),
    RubberCone(RubberConeItem),
    Gimbap(GimbapItem),
}

#[derive(Debug, Clone, Copy, State, PartialEq, Eq, Hash)]
pub struct ItemId(pub u64);

#[derive(Debug, Clone, State, PartialEq)]
pub struct ItemWithId {
    pub id: ItemId,
    pub item: Item,
}

static NEXT_ITEM_ID: AtomicU64 = AtomicU64::new(1);

impl ItemWithId {
    pub fn new(item: Item) -> Self {
        Self {
            id: ItemId(NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed)),
            item,
        }
    }

    pub fn to_core_state(&self) -> td_core::ItemEntryState {
        let mut state = td_core::ItemEntryState {
            id: self.id.0,
            kind: self.item.discriminant().to_core_raw(),
            scalar_values: Vec::new(),
            signed_values: Vec::new(),
        };
        match &self.item {
            Item::Bread(item) => {
                state.signed_values.push(item.heal_amount.raw());
                state.signed_values.push(item.shield_amount.raw());
            }
            Item::Candy(item) => state.signed_values.push(item.heal_amount.raw()),
            Item::Cannoli(item) => state.signed_values.push(item.heal_amount.raw()),
            Item::Cookie(item) => state.signed_values.push(item.heal_amount.raw()),
            Item::Donut(item) => state.signed_values.push(item.heal_amount.raw()),
            Item::Gimbap(item) => {
                state.signed_values.push(item.heal_amount.raw());
                state.signed_values.push(item.shield_amount.raw());
            }
            Item::LunchBox(item) => {
                state.signed_values.push(item.heal_amount.raw());
                state.signed_values.push(item.shield_amount.raw());
            }
            Item::RiceBall(item) => {
                state.signed_values.push(item.heal_amount.raw());
                state.signed_values.push(item.shield_amount.raw());
            }
            Item::LumpSugar(item) => state.scalar_values.push(item.reroll_amount as u64),
            Item::Milk(item) => state.signed_values.push(item.shield_amount.raw()),
            Item::RubberCone(item) => state.scalar_values.push(item.count as u64),
        }
        state
    }

    pub fn from_core_state(state: td_core::ItemEntryState) -> Option<Self> {
        let discriminant = ItemDiscriminants::from_core_raw(state.kind)?;
        let scalar = |index: usize| usize::try_from(*state.scalar_values.get(index)?).ok();
        let signed = |index: usize| state.signed_values.get(index).copied();
        let item = match discriminant {
            ItemDiscriminants::Bread => Item::Bread(BreadItem {
                heal_amount: Health::from_raw(signed(0)?),
                shield_amount: Shield::from_raw(signed(1)?),
            }),
            ItemDiscriminants::Candy => Item::Candy(CandyItem {
                heal_amount: Health::from_raw(signed(0)?),
            }),
            ItemDiscriminants::Cannoli => Item::Cannoli(CannoliItem {
                heal_amount: Health::from_raw(signed(0)?),
            }),
            ItemDiscriminants::Cookie => Item::Cookie(CookieItem {
                heal_amount: Health::from_raw(signed(0)?),
            }),
            ItemDiscriminants::Donut => Item::Donut(DonutItem {
                heal_amount: Health::from_raw(signed(0)?),
            }),
            ItemDiscriminants::Gimbap => Item::Gimbap(GimbapItem {
                heal_amount: Health::from_raw(signed(0)?),
                shield_amount: Shield::from_raw(signed(1)?),
            }),
            ItemDiscriminants::LunchBox => Item::LunchBox(LunchBoxItem {
                heal_amount: Health::from_raw(signed(0)?),
                shield_amount: Shield::from_raw(signed(1)?),
            }),
            ItemDiscriminants::LumpSugar => Item::LumpSugar(LumpSugarItem {
                reroll_amount: scalar(0)?,
            }),
            ItemDiscriminants::Milk => Item::Milk(MilkItem {
                shield_amount: Shield::from_raw(signed(0)?),
            }),
            ItemDiscriminants::RiceBall => Item::RiceBall(RiceBallItem {
                heal_amount: Health::from_raw(signed(0)?),
                shield_amount: Shield::from_raw(signed(1)?),
            }),
            ItemDiscriminants::RubberCone => Item::RubberCone(RubberConeItem { count: scalar(0)? }),
        };
        let restored = Self {
            id: ItemId(state.id),
            item,
        };
        (restored.to_core_state() == state).then_some(restored)
    }
}

impl ItemDiscriminants {
    pub(crate) const fn to_core_kind(self) -> td_core::ItemKind {
        match self {
            Self::Bread => td_core::ItemKind::Bread,
            Self::Candy => td_core::ItemKind::Candy,
            Self::Cannoli => td_core::ItemKind::Cannoli,
            Self::Cookie => td_core::ItemKind::Cookie,
            Self::Donut => td_core::ItemKind::Donut,
            Self::RiceBall => td_core::ItemKind::RiceBall,
            Self::LunchBox => td_core::ItemKind::LunchBox,
            Self::LumpSugar => td_core::ItemKind::LumpSugar,
            Self::Milk => td_core::ItemKind::Milk,
            Self::RubberCone => td_core::ItemKind::RubberCone,
            Self::Gimbap => td_core::ItemKind::Gimbap,
        }
    }

    pub(crate) const fn to_core_raw(self) -> u8 {
        self.to_core_kind().raw()
    }

    pub(crate) const fn from_core_raw(value: u8) -> Option<Self> {
        match td_core::ItemKind::from_raw(value) {
            Some(kind) => Some(Self::from_core_kind(kind)),
            None => None,
        }
    }

    pub(crate) const fn from_core_kind(kind: td_core::ItemKind) -> Self {
        match kind {
            td_core::ItemKind::Bread => Self::Bread,
            td_core::ItemKind::Candy => Self::Candy,
            td_core::ItemKind::Cannoli => Self::Cannoli,
            td_core::ItemKind::Cookie => Self::Cookie,
            td_core::ItemKind::Donut => Self::Donut,
            td_core::ItemKind::RiceBall => Self::RiceBall,
            td_core::ItemKind::LunchBox => Self::LunchBox,
            td_core::ItemKind::LumpSugar => Self::LumpSugar,
            td_core::ItemKind::Milk => Self::Milk,
            td_core::ItemKind::RubberCone => Self::RubberCone,
            td_core::ItemKind::Gimbap => Self::Gimbap,
        }
    }
}

impl std::ops::Deref for ItemWithId {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl std::ops::DerefMut for ItemWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl PartialEq<Item> for ItemWithId {
    fn eq(&self, other: &Item) -> bool {
        self.item == *other
    }
}

impl Item {
    pub fn with_unique_id(self) -> ItemWithId {
        ItemWithId::new(self)
    }

    pub fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        ItemBehavior::l10n_name(self, builder, locale)
    }

    pub fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        ItemBehavior::l10n_description(self, builder, locale)
    }

    pub fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        ItemBehavior::thumbnail_source(self)
    }

    pub fn discriminant(&self) -> ItemDiscriminants {
        self.into()
    }
}

impl ItemDiscriminants {
    pub(crate) fn generate(self, rng: &mut dyn rand::RngCore) -> Item {
        let _ = rng;
        let raw = td_core::generated_item(self.to_core_kind())
            .and_then(ItemWithId::from_core_state)
            .expect("core item generation must produce a valid item");
        raw.item
    }

    pub(crate) fn rarity(self) -> crate::Rarity {
        match td_core::item_rarity(self.to_core_kind()).expect("valid item kind") {
            td_core::Rarity::Common => crate::Rarity::Common,
            td_core::Rarity::Rare => crate::Rarity::Rare,
            td_core::Rarity::Epic => crate::Rarity::Epic,
            td_core::Rarity::Legendary => crate::Rarity::Legendary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_payloads_round_trip_through_core_state() {
        let items = vec![
            ItemWithId::new(Item::Bread(BreadItem {
                heal_amount: Health::from_raw(100),
                shield_amount: Shield::from_raw(200),
            })),
            ItemWithId::new(Item::Candy(CandyItem {
                heal_amount: Health::from_raw(300),
            })),
            ItemWithId::new(Item::Gimbap(GimbapItem {
                heal_amount: Health::from_raw(400),
                shield_amount: Shield::from_raw(500),
            })),
            ItemWithId::new(Item::LumpSugar(LumpSugarItem { reroll_amount: 2 })),
            ItemWithId::new(Item::Milk(MilkItem {
                shield_amount: Shield::from_raw(600),
            })),
            ItemWithId::new(Item::RubberCone(RubberConeItem { count: 3 })),
        ];

        for item in items {
            let raw = item.to_core_state();
            let restored = ItemWithId::from_core_state(raw.clone()).expect("valid item state");
            assert_eq!(restored.to_core_state(), raw);
        }
    }

    #[test]
    fn item_raw_state_rejects_invalid_kind() {
        let mut raw = ItemWithId::new(Item::Bread(BreadItem {
            heal_amount: Health::from_raw(1),
            shield_amount: Shield::from_raw(1),
        }))
        .to_core_state();
        raw.kind = 11;

        assert!(ItemWithId::from_core_state(raw).is_none());
    }

    #[test]
    fn item_presentation_mapping_covers_every_core_kind() {
        use strum::IntoEnumIterator;

        for &kind in td_core::ItemKind::ALL {
            let discriminant = ItemDiscriminants::from_core_kind(kind);
            assert_eq!(discriminant.to_core_kind(), kind);
            assert_eq!(discriminant.to_core_kind().key(), kind.key());
        }
        assert_eq!(ItemDiscriminants::iter().count(), td_core::ItemKind::COUNT);
    }
}
