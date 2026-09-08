//! Semantic rarity values used by authoritative generation and shop rules.

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(u8)]
pub enum Rarity {
    #[default]
    Common = 0,
    Rare = 1,
    Epic = 2,
    Legendary = 3,
}

impl Rarity {
    pub const ALL: [Self; 4] = [Self::Common, Self::Rare, Self::Epic, Self::Legendary];

    pub const fn raw(self) -> u8 {
        self as u8
    }

    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::Common),
            1 => Some(Self::Rare),
            2 => Some(Self::Epic),
            3 => Some(Self::Legendary),
            _ => None,
        }
    }

    pub const fn index(self) -> usize {
        self as usize
    }
}
