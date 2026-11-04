//! Simulation event types for recording and analysis.

use anyhow::{Context, bail};
use serde::{Serialize, Serializer};
use td_core::{CardServiceKind, ItemKind, ShopContentCategory, UpgradeKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContentKind {
    Item(ItemKind),
    Upgrade(UpgradeKind),
    CardService(CardServiceKind),
}

impl ContentKind {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Item(kind) => kind.key(),
            Self::Upgrade(kind) => kind.key(),
            Self::CardService(kind) => kind.key(),
        }
    }

    pub fn from_shop_observation(
        category: ShopContentCategory,
        key_id: u16,
    ) -> anyhow::Result<Self> {
        let raw_id = catalog_raw_id(key_id)?;
        match category {
            ShopContentCategory::Item => ItemKind::from_raw(raw_id)
                .map(Self::Item)
                .with_context(|| format!("unknown item catalog id {key_id}")),
            ShopContentCategory::Upgrade => UpgradeKind::from_raw(raw_id)
                .map(Self::Upgrade)
                .with_context(|| format!("unknown upgrade catalog id {key_id}")),
            ShopContentCategory::CardService => CardServiceKind::from_raw(raw_id)
                .map(Self::CardService)
                .with_context(|| format!("unknown card service catalog id {key_id}")),
        }
    }

    pub fn from_event_key(key: &str) -> anyhow::Result<Self> {
        let item = find_catalog_kind(key, ItemKind::ALL, ItemKind::key).map(Self::Item);
        let upgrade = find_catalog_kind(key, UpgradeKind::ALL, UpgradeKind::key).map(Self::Upgrade);
        let card_service = find_catalog_kind(key, CardServiceKind::ALL, CardServiceKind::key)
            .map(Self::CardService);
        match (item, upgrade, card_service) {
            (Some(kind), None, None) | (None, Some(kind), None) | (None, None, Some(kind)) => {
                Ok(kind)
            }
            (None, None, None) => bail!("unknown content key {key:?}"),
            _ => bail!("ambiguous content key {key:?}"),
        }
    }
}

impl From<ItemKind> for ContentKind {
    fn from(kind: ItemKind) -> Self {
        Self::Item(kind)
    }
}

impl From<UpgradeKind> for ContentKind {
    fn from(kind: UpgradeKind) -> Self {
        Self::Upgrade(kind)
    }
}

impl From<CardServiceKind> for ContentKind {
    fn from(kind: CardServiceKind) -> Self {
        Self::CardService(kind)
    }
}

pub fn item_kind_from_key_id(key_id: u16) -> anyhow::Result<ItemKind> {
    ItemKind::from_raw(catalog_raw_id(key_id)?)
        .with_context(|| format!("unknown item catalog id {key_id}"))
}

pub fn upgrade_kind_from_event_key(key: &str) -> anyhow::Result<UpgradeKind> {
    find_catalog_kind(key, UpgradeKind::ALL, UpgradeKind::key)
        .with_context(|| format!("unknown upgrade key {key:?}"))
}

pub fn card_service_kind_from_key_id(key_id: u16) -> anyhow::Result<CardServiceKind> {
    CardServiceKind::from_raw(catalog_raw_id(key_id)?)
        .with_context(|| format!("unknown card service catalog id {key_id}"))
}

fn catalog_raw_id(key_id: u16) -> anyhow::Result<u8> {
    let raw_id = key_id
        .checked_sub(1)
        .with_context(|| format!("catalog id zero is reserved: {key_id}"))?;
    u8::try_from(raw_id).context("catalog id exceeds the supported range")
}

fn find_catalog_kind<K: Copy>(
    key: &str,
    kinds: &[K],
    catalog_key: impl Fn(K) -> &'static str,
) -> Option<K> {
    let key = kind_name_key(&crate::canonicalize_kind_name(key.to_string()));
    kinds
        .iter()
        .copied()
        .find(|kind| kind_name_key(catalog_key(*kind)) == key)
}

fn kind_name_key(name: &str) -> String {
    name.chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

fn serialize_content_kind<S>(kind: &ContentKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(kind.key())
}

fn serialize_item_kind<S>(kind: &ItemKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(kind.key())
}

fn serialize_upgrade_kind<S>(kind: &UpgradeKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(kind.key())
}

fn serialize_card_service_kind<S>(kind: &CardServiceKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(kind.key())
}

#[derive(Clone, Debug, Serialize)]
pub enum SimEvent {
    GameStart,
    StageStart {
        stage: usize,
    },
    ShopPurchase {
        stage: usize,
        cost: usize,
        #[serde(rename = "item_kind", serialize_with = "serialize_content_kind")]
        content_kind: ContentKind,
    },
    CardReroll {
        stage: usize,
        reroll_number: usize,
    },
    TowerSelected {
        stage: usize,
        tower_kind: String,
        rank: String,
        suit: String,
    },
    TowerPlaced {
        stage: usize,
        tower_kind: String,
        x: usize,
        y: usize,
    },
    TowerRemoved {
        stage: usize,
        x: usize,
        y: usize,
    },
    DefenseStart {
        stage: usize,
    },
    DefenseEnd {
        stage: usize,
        victory: bool,
    },
    DamageTaken {
        stage: usize,
        amount: f32,
        remaining_hp: f32,
    },
    MonsterKilled {
        stage: usize,
        monster_kind: String,
    },
    ItemUsed {
        stage: usize,
        #[serde(serialize_with = "serialize_item_kind")]
        item_kind: ItemKind,
    },
    CardServiceUsed {
        stage: usize,
        #[serde(serialize_with = "serialize_card_service_kind")]
        service_kind: CardServiceKind,
        cards_selected: usize,
    },
    TreasureSelected {
        stage: usize,
        #[serde(serialize_with = "serialize_upgrade_kind")]
        upgrade_kind: UpgradeKind,
    },
    GameEnd {
        final_stage: usize,
        victory: bool,
        clear_rate: f32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_keys_parse_to_catalog_types() {
        assert_eq!(
            ContentKind::from_event_key("bread").unwrap(),
            ContentKind::Item(ItemKind::Bread)
        );
        assert_eq!(
            ContentKind::from_event_key("black_white").unwrap(),
            ContentKind::Upgrade(UpgradeKind::BlackWhite)
        );
        assert_eq!(
            ContentKind::from_event_key("long_sword").unwrap(),
            ContentKind::CardService(CardServiceKind::LongSword)
        );
        assert_eq!(
            upgrade_kind_from_event_key("Cat(CatUpgrade)").unwrap(),
            UpgradeKind::Cat
        );
        assert!(ContentKind::from_event_key("not_a_content_kind").is_err());
        for &kind in ItemKind::ALL {
            assert_eq!(
                ContentKind::from_event_key(kind.key()).unwrap(),
                ContentKind::Item(kind)
            );
        }
        for &kind in UpgradeKind::ALL {
            assert_eq!(
                ContentKind::from_event_key(kind.key()).unwrap(),
                ContentKind::Upgrade(kind)
            );
        }
        for &kind in CardServiceKind::ALL {
            assert_eq!(
                ContentKind::from_event_key(kind.key()).unwrap(),
                ContentKind::CardService(kind)
            );
        }
    }

    #[test]
    fn shop_observation_ids_parse_to_catalog_types() {
        assert_eq!(
            ContentKind::from_shop_observation(ShopContentCategory::Item, 1).unwrap(),
            ContentKind::Item(ItemKind::Bread)
        );
        assert_eq!(
            ContentKind::from_shop_observation(ShopContentCategory::Upgrade, 11).unwrap(),
            ContentKind::Upgrade(UpgradeKind::BlackWhite)
        );
        assert_eq!(
            ContentKind::from_shop_observation(ShopContentCategory::CardService, 1).unwrap(),
            ContentKind::CardService(CardServiceKind::LongSword)
        );
        assert!(ContentKind::from_shop_observation(ShopContentCategory::Item, 0).is_err());
    }

    #[test]
    fn typed_events_keep_the_existing_database_json_shape() {
        let event = SimEvent::ShopPurchase {
            stage: 1,
            cost: 10,
            content_kind: ContentKind::Item(ItemKind::Bread),
        };
        assert_eq!(
            serde_json::to_value(event).unwrap(),
            serde_json::json!({
                "ShopPurchase": {
                    "stage": 1,
                    "cost": 10,
                    "item_kind": "Bread"
                }
            })
        );
    }
}
