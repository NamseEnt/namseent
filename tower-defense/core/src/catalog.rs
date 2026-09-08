//! Stable, presentation-free names for authoritative content kinds.
//!
//! The numeric values are the raw ids persisted in [`CoreState`].  ML
//! vocabularies can derive their one-based ids from these tables without
//! importing the headed crate.

/// Version of the raw catalog key and numeric-id contract.
pub const CATALOG_SCHEMA_VERSION: u32 = 1;

pub const ITEM_KIND_KEYS: [&str; 11] = [
    "Bread",
    "Candy",
    "Cannoli",
    "Cookie",
    "Donut",
    "RiceBall",
    "LunchBox",
    "LumpSugar",
    "Milk",
    "RubberCone",
    "Gimbap",
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ItemKind {
    Bread = 0,
    Candy = 1,
    Cannoli = 2,
    Cookie = 3,
    Donut = 4,
    RiceBall = 5,
    LunchBox = 6,
    LumpSugar = 7,
    Milk = 8,
    RubberCone = 9,
    Gimbap = 10,
}

impl ItemKind {
    pub const ALL: &'static [Self] = &[
        Self::Bread,
        Self::Candy,
        Self::Cannoli,
        Self::Cookie,
        Self::Donut,
        Self::RiceBall,
        Self::LunchBox,
        Self::LumpSugar,
        Self::Milk,
        Self::RubberCone,
        Self::Gimbap,
    ];
    pub const COUNT: usize = Self::ALL.len();

    pub const fn raw(self) -> u8 {
        self as u8
    }

    pub const fn from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::Bread,
            1 => Self::Candy,
            2 => Self::Cannoli,
            3 => Self::Cookie,
            4 => Self::Donut,
            5 => Self::RiceBall,
            6 => Self::LunchBox,
            7 => Self::LumpSugar,
            8 => Self::Milk,
            9 => Self::RubberCone,
            10 => Self::Gimbap,
            _ => return None,
        })
    }

    pub const fn key(self) -> &'static str {
        ITEM_KIND_KEYS[self.raw() as usize]
    }

    pub const fn id(self) -> u16 {
        self.raw() as u16 + 1
    }
}

pub const UPGRADE_KIND_KEYS: [&str; 37] = [
    "Apple",
    "Banana",
    "Carrot",
    "Cat",
    "Backpack",
    "DiceBundle",
    "EnergyDrink",
    "PerfectPottery",
    "FourLeafClover",
    "Rabbit",
    "BlackWhite",
    "Trophy",
    "Crock",
    "CupNoodles",
    "FrenchFries",
    "Hamburger",
    "Pizza",
    "DemolitionHammer",
    "Metronome",
    "Tape",
    "NameTag",
    "ShoppingBag",
    "Resolution",
    "Mirror",
    "IceCream",
    "Spanner",
    "Pea",
    "SlotMachine",
    "PiggyBank",
    "Camera",
    "GiftBox",
    "Fang",
    "Popcorn",
    "MembershipCard",
    "BrokenPottery",
    "Strawberry",
    "Watermelon",
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum UpgradeKind {
    Apple = 0,
    Banana = 1,
    Carrot = 2,
    Cat = 3,
    Backpack = 4,
    DiceBundle = 5,
    EnergyDrink = 6,
    PerfectPottery = 7,
    FourLeafClover = 8,
    Rabbit = 9,
    BlackWhite = 10,
    Trophy = 11,
    Crock = 12,
    CupNoodles = 13,
    FrenchFries = 14,
    Hamburger = 15,
    Pizza = 16,
    DemolitionHammer = 17,
    Metronome = 18,
    Tape = 19,
    NameTag = 20,
    ShoppingBag = 21,
    Resolution = 22,
    Mirror = 23,
    IceCream = 24,
    Spanner = 25,
    Pea = 26,
    SlotMachine = 27,
    PiggyBank = 28,
    Camera = 29,
    GiftBox = 30,
    Fang = 31,
    Popcorn = 32,
    MembershipCard = 33,
    BrokenPottery = 34,
    Strawberry = 35,
    Watermelon = 36,
}

impl UpgradeKind {
    pub const ALL: &'static [Self] = &[
        Self::Apple,
        Self::Banana,
        Self::Carrot,
        Self::Cat,
        Self::Backpack,
        Self::DiceBundle,
        Self::EnergyDrink,
        Self::PerfectPottery,
        Self::FourLeafClover,
        Self::Rabbit,
        Self::BlackWhite,
        Self::Trophy,
        Self::Crock,
        Self::CupNoodles,
        Self::FrenchFries,
        Self::Hamburger,
        Self::Pizza,
        Self::DemolitionHammer,
        Self::Metronome,
        Self::Tape,
        Self::NameTag,
        Self::ShoppingBag,
        Self::Resolution,
        Self::Mirror,
        Self::IceCream,
        Self::Spanner,
        Self::Pea,
        Self::SlotMachine,
        Self::PiggyBank,
        Self::Camera,
        Self::GiftBox,
        Self::Fang,
        Self::Popcorn,
        Self::MembershipCard,
        Self::BrokenPottery,
        Self::Strawberry,
        Self::Watermelon,
    ];
    pub const COUNT: usize = Self::ALL.len();

    pub const fn raw(self) -> u8 {
        self as u8
    }

    pub const fn from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::Apple,
            1 => Self::Banana,
            2 => Self::Carrot,
            3 => Self::Cat,
            4 => Self::Backpack,
            5 => Self::DiceBundle,
            6 => Self::EnergyDrink,
            7 => Self::PerfectPottery,
            8 => Self::FourLeafClover,
            9 => Self::Rabbit,
            10 => Self::BlackWhite,
            11 => Self::Trophy,
            12 => Self::Crock,
            13 => Self::CupNoodles,
            14 => Self::FrenchFries,
            15 => Self::Hamburger,
            16 => Self::Pizza,
            17 => Self::DemolitionHammer,
            18 => Self::Metronome,
            19 => Self::Tape,
            20 => Self::NameTag,
            21 => Self::ShoppingBag,
            22 => Self::Resolution,
            23 => Self::Mirror,
            24 => Self::IceCream,
            25 => Self::Spanner,
            26 => Self::Pea,
            27 => Self::SlotMachine,
            28 => Self::PiggyBank,
            29 => Self::Camera,
            30 => Self::GiftBox,
            31 => Self::Fang,
            32 => Self::Popcorn,
            33 => Self::MembershipCard,
            34 => Self::BrokenPottery,
            35 => Self::Strawberry,
            36 => Self::Watermelon,
            _ => return None,
        })
    }

    pub const fn key(self) -> &'static str {
        UPGRADE_KIND_KEYS[self.raw() as usize]
    }

    pub const fn id(self) -> u16 {
        self.raw() as u16 + 1
    }
}

pub const CARD_SERVICE_KIND_KEYS: [&str; 16] = [
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
    "cactus",
    "spinning_top",
    "battery",
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardServiceKind {
    LongSword = 0,
    Staff = 1,
    Mace = 2,
    ClubSword = 3,
    Brush = 4,
    FountainPen = 5,
    Tricycle = 6,
    Eraser = 7,
    MagicWand = 8,
    Pliers = 9,
    Screwdriver = 10,
    Copier = 11,
    Magnet = 12,
    Cactus = 13,
    SpinningTop = 14,
    Battery = 15,
}

impl CardServiceKind {
    pub const ALL: &'static [Self] = &[
        Self::LongSword,
        Self::Staff,
        Self::Mace,
        Self::ClubSword,
        Self::Brush,
        Self::FountainPen,
        Self::Tricycle,
        Self::Eraser,
        Self::MagicWand,
        Self::Pliers,
        Self::Screwdriver,
        Self::Copier,
        Self::Magnet,
        Self::Cactus,
        Self::SpinningTop,
        Self::Battery,
    ];
    pub const COUNT: usize = Self::ALL.len();

    pub const fn raw(self) -> u8 {
        self as u8
    }

    pub const fn from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::LongSword,
            1 => Self::Staff,
            2 => Self::Mace,
            3 => Self::ClubSword,
            4 => Self::Brush,
            5 => Self::FountainPen,
            6 => Self::Tricycle,
            7 => Self::Eraser,
            8 => Self::MagicWand,
            9 => Self::Pliers,
            10 => Self::Screwdriver,
            11 => Self::Copier,
            12 => Self::Magnet,
            13 => Self::Cactus,
            14 => Self::SpinningTop,
            15 => Self::Battery,
            _ => return None,
        })
    }

    pub const fn key(self) -> &'static str {
        CARD_SERVICE_KIND_KEYS[self.raw() as usize]
    }

    pub const fn id(self) -> u16 {
        self.raw() as u16 + 1
    }
}

pub const TOWER_KIND_KEYS: [&str; 11] = [
    "RubberCone",
    "High",
    "OnePair",
    "TwoPair",
    "ThreeOfAKind",
    "Straight",
    "Flush",
    "FullHouse",
    "FourOfAKind",
    "StraightFlush",
    "RoyalFlush",
];

pub const MONSTER_KIND_KEYS: [&str; 64] = [
    "Mob01", "Mob02", "Mob03", "Mob04", "Mob05", "Mob06", "Mob07", "Mob08", "Mob09", "Mob10",
    "Mob11", "Mob12", "Mob13", "Mob14", "Mob15", "Mob16", "Mob17", "Mob18", "Mob19", "Mob20",
    "Mob21", "Mob22", "Mob23", "Mob24", "Mob25", "Mob26", "Mob27", "Mob28", "Mob29", "Mob30",
    "Mob31", "Mob32", "Mob33", "Mob34", "Mob35", "Mob36", "Mob37", "Mob38", "Mob39", "Mob40",
    "Mob41", "Mob42", "Mob43", "Mob44", "Mob45", "Mob46", "Mob47", "Mob48", "Mob49", "Mob50",
    "Boss01", "Boss02", "Boss03", "Boss04", "Boss05", "Boss06", "Boss07", "Boss08", "Boss09",
    "Boss10", "Boss11", "Boss12", "Boss13", "Boss14",
];

pub const UPGRADE_RARITY_PREFIXES: [&str; 37] = [
    "[C]", "[R]", "[L]", "[E]", "[C]", "[R]", "[C]", "[C]", "[R]", "[R]", "[L]", "[L]", "[E]",
    "[C]", "[C]", "[R]", "[R]", "[L]", "[C]", "[C]", "[E]", "[L]", "[R]", "[R]", "[R]", "[E]",
    "[R]", "[R]", "[R]", "[L]", "[L]", "[R]", "[R]", "[R]", "[C]", "[C]", "[E]",
];

pub fn item_kind_key(raw: u8) -> Option<&'static str> {
    ItemKind::from_raw(raw).map(ItemKind::key)
}

pub fn upgrade_kind_key(raw: u8) -> Option<&'static str> {
    UpgradeKind::from_raw(raw).map(UpgradeKind::key)
}

pub fn card_service_kind_key(raw: u8) -> Option<&'static str> {
    CardServiceKind::from_raw(raw).map(CardServiceKind::key)
}

pub fn tower_kind_key(raw: u8) -> Option<&'static str> {
    TOWER_KIND_KEYS.get(raw as usize).copied()
}

pub fn monster_kind_key(raw: u8) -> Option<&'static str> {
    MONSTER_KIND_KEYS.get(raw as usize).copied()
}

pub fn item_kind_id(raw: u8) -> u16 {
    ItemKind::from_raw(raw).map_or(0, ItemKind::id)
}

pub fn upgrade_kind_id(raw: u8) -> u16 {
    UpgradeKind::from_raw(raw).map_or(0, UpgradeKind::id)
}

pub fn card_service_kind_id(raw: u8) -> u16 {
    CardServiceKind::from_raw(raw).map_or(0, CardServiceKind::id)
}

pub fn tower_kind_id(raw: u8) -> u16 {
    tower_kind_key(raw)
        .map(|_| raw as u16 + 1)
        .unwrap_or_default()
}

pub fn monster_kind_id(raw: u8) -> u16 {
    monster_kind_key(raw)
        .map(|_| raw as u16 + 1)
        .unwrap_or_default()
}

pub fn upgrade_rarity_prefix(raw: u8) -> Option<&'static str> {
    UpgradeKind::from_raw(raw).map(upgrade_rarity_prefix_for_kind)
}

pub fn upgrade_rarity_prefix_for_kind(kind: UpgradeKind) -> &'static str {
    UPGRADE_RARITY_PREFIXES[kind.raw() as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_catalogs_are_exhaustive_and_one_based_ids_are_stable() {
        assert_eq!(item_kind_id(0), 1);
        assert_eq!(item_kind_id(10), ITEM_KIND_KEYS.len() as u16);
        assert_eq!(upgrade_kind_id(36), UPGRADE_KIND_KEYS.len() as u16);
        assert_eq!(tower_kind_id(10), TOWER_KIND_KEYS.len() as u16);
        assert_eq!(monster_kind_id(63), MONSTER_KIND_KEYS.len() as u16);
        assert_eq!(monster_kind_key(64), None);
    }

    #[test]
    fn typed_catalogs_round_trip_every_stable_mapping() {
        for (raw, kind) in ItemKind::ALL.iter().copied().enumerate() {
            let raw = raw as u8;
            assert_eq!(kind.raw(), raw);
            assert_eq!(ItemKind::from_raw(raw), Some(kind));
            assert_eq!(kind.key(), ITEM_KIND_KEYS[raw as usize]);
            assert_eq!(kind.id(), raw as u16 + 1);
        }
        for (raw, kind) in UpgradeKind::ALL.iter().copied().enumerate() {
            let raw = raw as u8;
            assert_eq!(kind.raw(), raw);
            assert_eq!(UpgradeKind::from_raw(raw), Some(kind));
            assert_eq!(kind.key(), UPGRADE_KIND_KEYS[raw as usize]);
            assert_eq!(kind.id(), raw as u16 + 1);
        }
        for (raw, kind) in CardServiceKind::ALL.iter().copied().enumerate() {
            let raw = raw as u8;
            assert_eq!(kind.raw(), raw);
            assert_eq!(CardServiceKind::from_raw(raw), Some(kind));
            assert_eq!(kind.key(), CARD_SERVICE_KIND_KEYS[raw as usize]);
            assert_eq!(kind.id(), raw as u16 + 1);
        }
    }

    #[test]
    fn typed_catalogs_reject_unknown_raw_values() {
        assert_eq!(ItemKind::from_raw(ItemKind::COUNT as u8), None);
        assert_eq!(UpgradeKind::from_raw(UpgradeKind::COUNT as u8), None);
        assert_eq!(
            CardServiceKind::from_raw(CardServiceKind::COUNT as u8),
            None
        );
        assert_eq!(item_kind_id(u8::MAX), 0);
        assert_eq!(upgrade_kind_id(u8::MAX), 0);
        assert_eq!(card_service_kind_id(u8::MAX), 0);
    }

    #[test]
    fn catalog_schema_and_stable_hash_inputs_are_explicit() {
        assert_eq!(CATALOG_SCHEMA_VERSION, 1);
        assert_eq!(ITEM_KIND_KEYS.len(), ItemKind::COUNT);
        assert_eq!(UPGRADE_KIND_KEYS.len(), UpgradeKind::COUNT);
        assert_eq!(CARD_SERVICE_KIND_KEYS.len(), CardServiceKind::COUNT);

        assert_eq!(crate::stable_key_hash("item:Bread"), 0x75a8_bbec_9f09_f6ea);
        assert_eq!(
            crate::stable_key_hash("card_service:long_sword"),
            0x61a7_d848_61ab_3e9b
        );
        assert_eq!(
            crate::stable_key_hash("upgrade:Apple"),
            0x5694_c0f5_d65d_0ccd
        );
    }
}
