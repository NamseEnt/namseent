//! Stable ML vocabulary mappings backed by `td-core` raw kind ids.

pub const SUIT_VOCABULARY: [&str; 4] = ["spades", "hearts", "diamonds", "clubs"];
pub const RANK_VOCABULARY: [&str; 13] = [
    "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen",
    "king", "ace",
];

pub const ENGRAVING_VOCABULARY: usize = 4;
pub const TOWER_KIND_VOCABULARY: usize = td_core::TOWER_KIND_KEYS.len();
pub const MONSTER_KIND_VOCABULARY: usize = td_core::MONSTER_KIND_KEYS.len();
pub const ITEM_VOCABULARY: usize = td_core::ItemKind::COUNT;
pub const UPGRADE_VOCABULARY: usize = td_core::UpgradeKind::COUNT;
pub const CARD_SERVICE_VOCABULARY: usize = td_core::CardServiceKind::COUNT;
pub const SHOP_KIND_VOCABULARY: [&str; 3] = ["item", "upgrade", "card_service"];

const ENGRAVING_KEYS: [&str; 4] = ["magnet", "overcharge", "cactus", "spinning_top"];
const UPGRADE_SNAKE_KEYS: [&str; 37] = [
    "apple",
    "banana",
    "carrot",
    "cat",
    "backpack",
    "dice_bundle",
    "energy_drink",
    "perfect_pottery",
    "four_leaf_clover",
    "rabbit",
    "black_white",
    "trophy",
    "crock",
    "cup_noodles",
    "french_fries",
    "hamburger",
    "pizza",
    "demolition_hammer",
    "metronome",
    "tape",
    "name_tag",
    "shopping_bag",
    "resolution",
    "mirror",
    "ice_cream",
    "spanner",
    "pea",
    "slot_machine",
    "piggy_bank",
    "camera",
    "gift_box",
    "fang",
    "popcorn",
    "membership_card",
    "broken_pottery",
    "strawberry",
    "watermelon",
];

/// 0 is reserved for padding; valid ids start at 1.
pub fn suit_id(value: &str) -> u16 {
    SUIT_VOCABULARY
        .iter()
        .position(|candidate| *candidate == value)
        .map(|index| index as u16 + 1)
        .unwrap_or_default()
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn rank_id(value: &str) -> u16 {
    RANK_VOCABULARY
        .iter()
        .position(|candidate| *candidate == value)
        .map(|index| index as u16 + 1)
        .unwrap_or_default()
}

/// Converts the raw core engraving id (0..=3) to the one-based ML id.
pub fn engraving_id(raw: u8) -> u16 {
    (raw as usize)
        .checked_add(1)
        .filter(|index| *index <= ENGRAVING_VOCABULARY)
        .map(|index| index as u16)
        .unwrap_or_default()
}

/// Converts a raw core kind id (0-based) to a one-based ML id.
pub fn tower_kind_id(raw: u8) -> u16 {
    td_core::tower_kind_id(raw)
}

/// Converts a raw core kind id (0-based) to a one-based ML id.
pub fn monster_kind_id(raw: u8) -> u16 {
    td_core::monster_kind_id(raw)
}

/// Converts a validated core item kind to its one-based ML id.
pub fn item_id(kind: td_core::ItemKind) -> u16 {
    kind.id()
}

/// Converts a raw core item kind id at the ML wire boundary.
#[deprecated(
    note = "use item_id(ItemKind::from_raw(raw).ok_or(...)?), or item_id_raw at a wire boundary"
)]
pub fn item_id_raw(raw: u8) -> u16 {
    td_core::item_kind_id(raw)
}

/// Converts a validated core upgrade kind to its one-based ML id.
pub fn upgrade_id(kind: td_core::UpgradeKind) -> u16 {
    kind.id()
}

/// Converts a raw core upgrade kind id at the ML wire boundary.
pub fn upgrade_id_raw(raw: u8) -> u16 {
    td_core::UpgradeKind::from_raw(raw)
        .map(upgrade_id)
        .unwrap_or_default()
}

/// Compatibility name for callers that previously passed a headed
/// `UpgradeDiscriminants` value at the raw ML boundary.
#[deprecated(note = "use upgrade_id_raw at the ML wire boundary")]
pub fn upgrade_discriminant_id(raw: u8) -> u16 {
    upgrade_id_raw(raw)
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn card_service_key_id(key: &str) -> u16 {
    td_core::CardServiceKind::ALL
        .iter()
        .find(|kind| kind.key() == key)
        .map(|kind| kind.id())
        .unwrap_or_default()
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn shop_kind_id(value: &str) -> u16 {
    SHOP_KIND_VOCABULARY
        .iter()
        .position(|candidate| *candidate == value)
        .map(|index| index as u16 + 1)
        .unwrap_or_default()
}

/// String-keyed exhaustive mapping for engraving observations.
pub fn engraving_key_id(value: &str) -> u16 {
    ENGRAVING_KEYS
        .iter()
        .position(|candidate| *candidate == value)
        .map(|index| index as u16 + 1)
        .unwrap_or_default()
}

/// String-keyed exhaustive mapping for upgrade key observations.
pub fn upgrade_key_id(value: &str) -> u16 {
    UPGRADE_SNAKE_KEYS
        .iter()
        .position(|candidate| *candidate == value)
        .map(|index| index as u16 + 1)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_vocabularies_are_exhaustive_and_nonzero() {
        assert_eq!(suit_id("spades"), 1);
        assert_eq!(rank_id("ace"), 13);
        assert_eq!(engraving_key_id("spinning_top"), 4);
        assert_eq!(shop_kind_id("card_service"), 3);
        assert_eq!(upgrade_key_id("watermelon"), UPGRADE_VOCABULARY as u16);
        assert_eq!(card_service_key_id("copier"), 12);
        assert_eq!(shop_kind_id("future_content"), 0);
    }

    #[test]
    fn raw_core_catalogs_match_ml_one_based_ids() {
        assert_eq!(tower_kind_id(0), 1);
        assert_eq!(tower_kind_id(10), TOWER_KIND_VOCABULARY as u16);
        assert_eq!(monster_kind_id(63), MONSTER_KIND_VOCABULARY as u16);
        assert_eq!(item_id(td_core::ItemKind::Gimbap), ITEM_VOCABULARY as u16);
        assert_eq!(
            upgrade_id(td_core::UpgradeKind::Watermelon),
            UPGRADE_VOCABULARY as u16
        );
        assert_eq!(engraving_id(3), ENGRAVING_VOCABULARY as u16);
    }

    #[test]
    #[allow(deprecated)]
    fn typed_core_catalogs_are_exhaustive_in_ml_ids() {
        for &kind in td_core::ItemKind::ALL {
            assert_eq!(item_id(kind), kind.id());
            assert_eq!(item_id_raw(kind.raw()), kind.id());
        }
        for &kind in td_core::UpgradeKind::ALL {
            assert_eq!(upgrade_id(kind), kind.id());
            assert_eq!(upgrade_id_raw(kind.raw()), kind.id());
            assert_eq!(
                upgrade_key_id(UPGRADE_SNAKE_KEYS[kind.raw() as usize]),
                kind.id()
            );
        }
        for &kind in td_core::CardServiceKind::ALL {
            assert_eq!(card_service_key_id(kind.key()), kind.id());
        }
        assert_eq!(item_id_raw(u8::MAX), 0);
        assert_eq!(upgrade_id_raw(u8::MAX), 0);
    }
}
