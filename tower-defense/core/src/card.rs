use std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(into = "u8", try_from = "u8")]
pub enum Suit {
    Spades = 0,
    Hearts = 1,
    Diamonds = 2,
    Clubs = 3,
}

impl Suit {
    pub const ALL: [Self; 4] = [Self::Spades, Self::Hearts, Self::Diamonds, Self::Clubs];

    pub const fn raw(self) -> u8 {
        self as u8
    }

    pub const fn from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::Spades,
            1 => Self::Hearts,
            2 => Self::Diamonds,
            3 => Self::Clubs,
            _ => return None,
        })
    }
}

impl From<Suit> for u8 {
    fn from(suit: Suit) -> Self {
        suit.raw()
    }
}

impl TryFrom<u8> for Suit {
    type Error = String;

    fn try_from(raw: u8) -> Result<Self, Self::Error> {
        Self::from_raw(raw).ok_or_else(|| format!("invalid suit raw value: {raw}"))
    }
}

impl Display for Suit {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Spades => "♠",
            Self::Hearts => "♥",
            Self::Diamonds => "◆",
            Self::Clubs => "♣",
        };
        write!(formatter, "{symbol}")
    }
}

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(into = "u8", try_from = "u8")]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

impl Rank {
    pub const ALL: [Self; 13] = [
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
        Self::Ten,
        Self::Jack,
        Self::Queen,
        Self::King,
        Self::Ace,
    ];
    pub const REVERSED: [Self; 13] = [
        Self::Ace,
        Self::King,
        Self::Queen,
        Self::Jack,
        Self::Ten,
        Self::Nine,
        Self::Eight,
        Self::Seven,
        Self::Six,
        Self::Five,
        Self::Four,
        Self::Three,
        Self::Two,
    ];

    pub const fn raw(self) -> u8 {
        self as u8
    }

    pub const fn from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::Two,
            1 => Self::Three,
            2 => Self::Four,
            3 => Self::Five,
            4 => Self::Six,
            5 => Self::Seven,
            6 => Self::Eight,
            7 => Self::Nine,
            8 => Self::Ten,
            9 => Self::Jack,
            10 => Self::Queen,
            11 => Self::King,
            12 => Self::Ace,
            _ => return None,
        })
    }

    pub const fn ordinal(self) -> usize {
        self as usize
    }

    pub const fn is_even(self) -> bool {
        matches!(
            self,
            Self::Two | Self::Four | Self::Six | Self::Eight | Self::Ten | Self::Queen
        )
    }

    pub const fn is_face(self) -> bool {
        matches!(self, Self::Jack | Self::Queen | Self::King)
    }

    pub const fn is_number_card(self) -> bool {
        self.ordinal() <= Self::Ten.ordinal()
    }

    pub const fn ace_low_value(self) -> usize {
        if matches!(self, Self::Ace) {
            0
        } else {
            self.ordinal() + 1
        }
    }

    pub const fn ace_high_value(self) -> usize {
        if matches!(self, Self::Ace) {
            13
        } else {
            self.ordinal() + 1
        }
    }
}

impl From<Rank> for u8 {
    fn from(rank: Rank) -> Self {
        rank.raw()
    }
}

impl TryFrom<u8> for Rank {
    type Error = String;

    fn try_from(raw: u8) -> Result<Self, Self::Error> {
        Self::from_raw(raw).ok_or_else(|| format!("invalid rank raw value: {raw}"))
    }
}

impl Display for Rank {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
            Self::Ace => "A",
        };
        write!(formatter, "{symbol}")
    }
}

#[cfg(test)]
mod tests {
    use super::{Rank, Suit};

    #[test]
    fn serde_preserves_raw_card_enum_values() {
        assert_eq!(serde_json::to_string(&Suit::Spades).unwrap(), "0");
        assert_eq!(serde_json::to_string(&Suit::Clubs).unwrap(), "3");
        assert_eq!(serde_json::to_string(&Rank::Two).unwrap(), "0");
        assert_eq!(serde_json::to_string(&Rank::Ace).unwrap(), "12");
    }

    #[test]
    fn serde_rejects_unknown_card_enum_values() {
        assert!(serde_json::from_str::<Suit>("4").is_err());
        assert!(serde_json::from_str::<Rank>("13").is_err());
    }
}
