use crate::card::Engraving;
use crate::game_state::item::Item;
use crate::game_state::monster::MonsterKind;
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::{Upgrade, UpgradeDiscriminants};

pub const SUIT_VOCABULARY: [&str; 4] = ["spades", "hearts", "diamonds", "clubs"];

pub const RANK_VOCABULARY: [&str; 13] = [
    "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen",
    "king", "ace",
];

pub const ENGRAVING_VOCABULARY: usize = 4;

pub const TOWER_KIND_VOCABULARY: usize = 11;

pub const MONSTER_KIND_VOCABULARY: usize = 64;

pub const ITEM_VOCABULARY: usize = 11;

pub const UPGRADE_VOCABULARY: usize = 37;

pub const CARD_SERVICE_VOCABULARY: usize = 16;

pub const SHOP_KIND_VOCABULARY: [&str; 3] = ["item", "upgrade", "card_service"];

/// 0 is reserved for padding; valid ids start at 1.
pub fn suit_id(value: &str) -> u16 {
    match value {
        "spades" => 1,
        "hearts" => 2,
        "diamonds" => 3,
        "clubs" => 4,
        _ => 0,
    }
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn rank_id(value: &str) -> u16 {
    match value {
        "two" => 1,
        "three" => 2,
        "four" => 3,
        "five" => 4,
        "six" => 5,
        "seven" => 6,
        "eight" => 7,
        "nine" => 8,
        "ten" => 9,
        "jack" => 10,
        "queen" => 11,
        "king" => 12,
        "ace" => 13,
        _ => 0,
    }
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn engraving_id(engraving: Engraving) -> u16 {
    match engraving {
        Engraving::Magnet => 1,
        Engraving::Overcharge => 2,
        Engraving::Cactus => 3,
        Engraving::SpinningTop => 4,
    }
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn tower_kind_id(kind: TowerKind) -> u16 {
    match kind {
        TowerKind::RubberCone => 1,
        TowerKind::High => 2,
        TowerKind::OnePair => 3,
        TowerKind::TwoPair => 4,
        TowerKind::ThreeOfAKind => 5,
        TowerKind::Straight => 6,
        TowerKind::Flush => 7,
        TowerKind::FullHouse => 8,
        TowerKind::FourOfAKind => 9,
        TowerKind::StraightFlush => 10,
        TowerKind::RoyalFlush => 11,
    }
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn monster_kind_id(kind: MonsterKind) -> u16 {
    match kind {
        MonsterKind::Mob01 => 1,
        MonsterKind::Mob02 => 2,
        MonsterKind::Mob03 => 3,
        MonsterKind::Mob04 => 4,
        MonsterKind::Mob05 => 5,
        MonsterKind::Mob06 => 6,
        MonsterKind::Mob07 => 7,
        MonsterKind::Mob08 => 8,
        MonsterKind::Mob09 => 9,
        MonsterKind::Mob10 => 10,
        MonsterKind::Mob11 => 11,
        MonsterKind::Mob12 => 12,
        MonsterKind::Mob13 => 13,
        MonsterKind::Mob14 => 14,
        MonsterKind::Mob15 => 15,
        MonsterKind::Mob16 => 16,
        MonsterKind::Mob17 => 17,
        MonsterKind::Mob18 => 18,
        MonsterKind::Mob19 => 19,
        MonsterKind::Mob20 => 20,
        MonsterKind::Mob21 => 21,
        MonsterKind::Mob22 => 22,
        MonsterKind::Mob23 => 23,
        MonsterKind::Mob24 => 24,
        MonsterKind::Mob25 => 25,
        MonsterKind::Mob26 => 26,
        MonsterKind::Mob27 => 27,
        MonsterKind::Mob28 => 28,
        MonsterKind::Mob29 => 29,
        MonsterKind::Mob30 => 30,
        MonsterKind::Mob31 => 31,
        MonsterKind::Mob32 => 32,
        MonsterKind::Mob33 => 33,
        MonsterKind::Mob34 => 34,
        MonsterKind::Mob35 => 35,
        MonsterKind::Mob36 => 36,
        MonsterKind::Mob37 => 37,
        MonsterKind::Mob38 => 38,
        MonsterKind::Mob39 => 39,
        MonsterKind::Mob40 => 40,
        MonsterKind::Mob41 => 41,
        MonsterKind::Mob42 => 42,
        MonsterKind::Mob43 => 43,
        MonsterKind::Mob44 => 44,
        MonsterKind::Mob45 => 45,
        MonsterKind::Mob46 => 46,
        MonsterKind::Mob47 => 47,
        MonsterKind::Mob48 => 48,
        MonsterKind::Mob49 => 49,
        MonsterKind::Mob50 => 50,
        MonsterKind::Boss01 => 51,
        MonsterKind::Boss02 => 52,
        MonsterKind::Boss03 => 53,
        MonsterKind::Boss04 => 54,
        MonsterKind::Boss05 => 55,
        MonsterKind::Boss06 => 56,
        MonsterKind::Boss07 => 57,
        MonsterKind::Boss08 => 58,
        MonsterKind::Boss09 => 59,
        MonsterKind::Boss10 => 60,
        MonsterKind::Boss11 => 61,
        MonsterKind::Boss12 => 62,
        MonsterKind::Boss13 => 63,
        MonsterKind::Boss14 => 64,
    }
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn item_id(item: &Item) -> u16 {
    match item {
        Item::Bread(_) => 1,
        Item::Candy(_) => 2,
        Item::Cannoli(_) => 3,
        Item::Cookie(_) => 4,
        Item::Donut(_) => 5,
        Item::RiceBall(_) => 6,
        Item::LunchBox(_) => 7,
        Item::LumpSugar(_) => 8,
        Item::Milk(_) => 9,
        Item::RubberCone(_) => 10,
        Item::Gimbap(_) => 11,
    }
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn upgrade_id(upgrade: &Upgrade) -> u16 {
    upgrade_discriminant_id(upgrade.discriminant())
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn upgrade_discriminant_id(discriminant: UpgradeDiscriminants) -> u16 {
    match discriminant {
        UpgradeDiscriminants::Apple => 1,
        UpgradeDiscriminants::Banana => 2,
        UpgradeDiscriminants::Carrot => 3,
        UpgradeDiscriminants::Cat => 4,
        UpgradeDiscriminants::Backpack => 5,
        UpgradeDiscriminants::DiceBundle => 6,
        UpgradeDiscriminants::EnergyDrink => 7,
        UpgradeDiscriminants::PerfectPottery => 8,
        UpgradeDiscriminants::FourLeafClover => 9,
        UpgradeDiscriminants::Rabbit => 10,
        UpgradeDiscriminants::BlackWhite => 11,
        UpgradeDiscriminants::Trophy => 12,
        UpgradeDiscriminants::Crock => 13,
        UpgradeDiscriminants::CupNoodles => 14,
        UpgradeDiscriminants::FrenchFries => 15,
        UpgradeDiscriminants::Hamburger => 16,
        UpgradeDiscriminants::Pizza => 17,
        UpgradeDiscriminants::DemolitionHammer => 18,
        UpgradeDiscriminants::Metronome => 19,
        UpgradeDiscriminants::Tape => 20,
        UpgradeDiscriminants::NameTag => 21,
        UpgradeDiscriminants::ShoppingBag => 22,
        UpgradeDiscriminants::Resolution => 23,
        UpgradeDiscriminants::Mirror => 24,
        UpgradeDiscriminants::IceCream => 25,
        UpgradeDiscriminants::Spanner => 26,
        UpgradeDiscriminants::Pea => 27,
        UpgradeDiscriminants::SlotMachine => 28,
        UpgradeDiscriminants::PiggyBank => 29,
        UpgradeDiscriminants::Camera => 30,
        UpgradeDiscriminants::GiftBox => 31,
        UpgradeDiscriminants::Fang => 32,
        UpgradeDiscriminants::Popcorn => 33,
        UpgradeDiscriminants::MembershipCard => 34,
        UpgradeDiscriminants::BrokenPottery => 35,
        UpgradeDiscriminants::Strawberry => 36,
        UpgradeDiscriminants::Watermelon => 37,
    }
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn card_service_key_id(key: &str) -> u16 {
    match key {
        "long_sword" => 1,
        "staff" => 2,
        "mace" => 3,
        "club_sword" => 4,
        "brush" => 5,
        "fountain_pen" => 6,
        "tricycle" => 7,
        "eraser" => 8,
        "magic_wand" => 9,
        "pliers" => 10,
        "screwdriver" => 11,
        "copier" => 12,
        "magnet" => 13,
        "battery" => 14,
        "cactus" => 15,
        "spinning_top" => 16,
        _ => 0,
    }
}

/// 0 is reserved for padding; valid ids start at 1.
pub fn shop_kind_id(value: &str) -> u16 {
    match value {
        "item" => 1,
        "upgrade" => 2,
        "card_service" => 3,
        _ => 0,
    }
}

/// String-keyed exhaustive mapping for engraving observations.
/// 0 is reserved for padding; valid ids start at 1.
pub fn engraving_key_id(value: &str) -> u16 {
    match value {
        "magnet" => 1,
        "overcharge" => 2,
        "cactus" => 3,
        "spinning_top" => 4,
        _ => 0,
    }
}

/// String-keyed exhaustive mapping for upgrade key observations.
/// 0 is reserved for padding; valid ids start at 1.
pub fn upgrade_key_id(value: &str) -> u16 {
    match value {
        "apple" => 1,
        "banana" => 2,
        "carrot" => 3,
        "cat" => 4,
        "backpack" => 5,
        "dice_bundle" => 6,
        "energy_drink" => 7,
        "perfect_pottery" => 8,
        "four_leaf_clover" => 9,
        "rabbit" => 10,
        "black_white" => 11,
        "trophy" => 12,
        "crock" => 13,
        "cup_noodles" => 14,
        "french_fries" => 15,
        "hamburger" => 16,
        "pizza" => 17,
        "demolition_hammer" => 18,
        "metronome" => 19,
        "tape" => 20,
        "name_tag" => 21,
        "shopping_bag" => 22,
        "resolution" => 23,
        "mirror" => 24,
        "ice_cream" => 25,
        "spanner" => 26,
        "pea" => 27,
        "slot_machine" => 28,
        "piggy_bank" => 29,
        "camera" => 30,
        "gift_box" => 31,
        "fang" => 32,
        "popcorn" => 33,
        "membership_card" => 34,
        "broken_pottery" => 35,
        "strawberry" => 36,
        "watermelon" => 37,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_categories_are_explicit_and_distinct() {
        assert_eq!(suit_id("spades"), 1);
        assert_eq!(suit_id("clubs"), 4);
        assert_ne!(rank_id("ace"), rank_id("two"));
        assert_ne!(shop_kind_id("item"), shop_kind_id("upgrade"));
        assert_eq!(shop_kind_id("future_content"), 0);
    }

    #[test]
    fn card_vocabularies_are_exhaustive_and_nonzero() {
        assert_eq!(SUIT_VOCABULARY.len(), 4);
        assert_eq!(RANK_VOCABULARY.len(), 13);
        assert!(SUIT_VOCABULARY.iter().all(|value| suit_id(value) != 0));
        assert!(RANK_VOCABULARY.iter().all(|value| rank_id(value) != 0));
        assert_eq!(
            SUIT_VOCABULARY
                .iter()
                .map(|value| suit_id(value))
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            SUIT_VOCABULARY.len()
        );
        assert_eq!(
            RANK_VOCABULARY
                .iter()
                .map(|value| rank_id(value))
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            RANK_VOCABULARY.len()
        );
    }

    #[test]
    fn shop_kinds_are_exhaustive_and_nonzero() {
        assert!(
            SHOP_KIND_VOCABULARY
                .iter()
                .all(|value| shop_kind_id(value) != 0)
        );
        assert_eq!(
            SHOP_KIND_VOCABULARY
                .iter()
                .map(|value| shop_kind_id(value))
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            SHOP_KIND_VOCABULARY.len()
        );
    }

    #[test]
    fn enum_ids_are_nonzero_and_within_vocabulary() {
        use crate::game_state::item::*;
        assert_eq!(tower_kind_id(TowerKind::RubberCone), 1);
        assert_eq!(
            tower_kind_id(TowerKind::RoyalFlush),
            TOWER_KIND_VOCABULARY as u16
        );
        assert_eq!(monster_kind_id(MonsterKind::Mob01), 1);
        assert_eq!(
            monster_kind_id(MonsterKind::Boss14),
            MONSTER_KIND_VOCABULARY as u16
        );
        assert_eq!(engraving_id(Engraving::Magnet), 1);
        assert_eq!(
            engraving_id(Engraving::SpinningTop),
            ENGRAVING_VOCABULARY as u16
        );

        let items = [
            Item::Bread(BreadItem::standard()),
            Item::Candy(CandyItem::standard()),
            Item::Cannoli(CannoliItem::standard()),
            Item::Cookie(CookieItem::standard()),
            Item::Donut(DonutItem::standard()),
            Item::RiceBall(RiceBallItem::standard()),
            Item::LunchBox(LunchBoxItem::standard()),
            Item::LumpSugar(LumpSugarItem::standard()),
            Item::Milk(MilkItem::standard()),
            Item::RubberCone(RubberConeItem::standard()),
            Item::Gimbap(GimbapItem::standard()),
        ];
        for (index, item) in items.iter().enumerate() {
            assert_eq!(item_id(item), index as u16 + 1);
        }
        assert_eq!(items.len(), ITEM_VOCABULARY);

        let card_service_keys = [
            "long_sword",
            "staff",
            "mace",
            "club_sword",
            "brush",
            "fountain_pen",
            "tricycle",
            "eraser",
            "magic_wand",
            "pliers",
            "screwdriver",
            "copier",
            "magnet",
            "battery",
            "cactus",
            "spinning_top",
        ];
        for (index, key) in card_service_keys.iter().enumerate() {
            assert_eq!(
                card_service_key_id(key),
                index as u16 + 1,
                "card service key {key}"
            );
        }
        assert_eq!(card_service_keys.len(), CARD_SERVICE_VOCABULARY);
    }

    #[test]
    fn upgrade_key_ids_cover_all_keys_and_match_enum_ids() {
        let keys = [
            ("apple", UpgradeDiscriminants::Apple),
            ("banana", UpgradeDiscriminants::Banana),
            ("carrot", UpgradeDiscriminants::Carrot),
            ("cat", UpgradeDiscriminants::Cat),
            ("backpack", UpgradeDiscriminants::Backpack),
            ("dice_bundle", UpgradeDiscriminants::DiceBundle),
            ("energy_drink", UpgradeDiscriminants::EnergyDrink),
            ("perfect_pottery", UpgradeDiscriminants::PerfectPottery),
            ("four_leaf_clover", UpgradeDiscriminants::FourLeafClover),
            ("rabbit", UpgradeDiscriminants::Rabbit),
            ("black_white", UpgradeDiscriminants::BlackWhite),
            ("trophy", UpgradeDiscriminants::Trophy),
            ("crock", UpgradeDiscriminants::Crock),
            ("cup_noodles", UpgradeDiscriminants::CupNoodles),
            ("french_fries", UpgradeDiscriminants::FrenchFries),
            ("hamburger", UpgradeDiscriminants::Hamburger),
            ("pizza", UpgradeDiscriminants::Pizza),
            ("demolition_hammer", UpgradeDiscriminants::DemolitionHammer),
            ("metronome", UpgradeDiscriminants::Metronome),
            ("tape", UpgradeDiscriminants::Tape),
            ("name_tag", UpgradeDiscriminants::NameTag),
            ("shopping_bag", UpgradeDiscriminants::ShoppingBag),
            ("resolution", UpgradeDiscriminants::Resolution),
            ("mirror", UpgradeDiscriminants::Mirror),
            ("ice_cream", UpgradeDiscriminants::IceCream),
            ("spanner", UpgradeDiscriminants::Spanner),
            ("pea", UpgradeDiscriminants::Pea),
            ("slot_machine", UpgradeDiscriminants::SlotMachine),
            ("piggy_bank", UpgradeDiscriminants::PiggyBank),
            ("camera", UpgradeDiscriminants::Camera),
            ("gift_box", UpgradeDiscriminants::GiftBox),
            ("fang", UpgradeDiscriminants::Fang),
            ("popcorn", UpgradeDiscriminants::Popcorn),
            ("membership_card", UpgradeDiscriminants::MembershipCard),
            ("broken_pottery", UpgradeDiscriminants::BrokenPottery),
            ("strawberry", UpgradeDiscriminants::Strawberry),
            ("watermelon", UpgradeDiscriminants::Watermelon),
        ];
        assert_eq!(keys.len(), UPGRADE_VOCABULARY);

        let mut seen = std::collections::BTreeSet::new();
        for (index, (key, discriminant)) in keys.iter().enumerate() {
            let string_id = upgrade_key_id(key);
            assert_eq!(string_id, index as u16 + 1, "upgrade key {key}");
            let enum_id = upgrade_discriminant_id(*discriminant);
            assert_eq!(string_id, enum_id, "id mismatch for upgrade {key}");
            seen.insert(string_id);
        }
        assert_eq!(seen.len(), UPGRADE_VOCABULARY);
    }
}
