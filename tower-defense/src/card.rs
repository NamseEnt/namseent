use crate::*;
use rand::Rng;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord, State)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "◆",
            Suit::Clubs => "♣",
        };
        write!(f, "{s}")
    }
}
pub const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord, State)]
pub enum Rank {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Rank {
    pub fn bonus_damage(&self) -> usize {
        match self {
            Rank::Seven => 1,
            Rank::Eight => 2,
            Rank::Nine => 3,
            Rank::Ten => 4,
            Rank::Jack => 6,
            Rank::Queen => 8,
            Rank::King => 10,
            Rank::Ace => 15,
        }
    }
    pub fn is_even(&self) -> bool {
        match self {
            Rank::Ace | Rank::Seven | Rank::Nine | Rank::Jack | Rank::King => false,
            Rank::Eight | Rank::Ten | Rank::Queen => true,
        }
    }
    pub fn is_face(&self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }
}
impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rank::Ace => "A",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        };
        write!(f, "{s}")
    }
}
pub const REVERSED_RANKS: [Rank; 8] = [
    Rank::Ace,
    Rank::King,
    Rank::Queen,
    Rank::Jack,
    Rank::Ten,
    Rank::Nine,
    Rank::Eight,
    Rank::Seven,
];

#[derive(Eq, Debug, PartialEq, Hash, Clone, Copy, State)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank_cmp = (self.rank as usize).cmp(&(other.rank as usize));
        if let std::cmp::Ordering::Equal = rank_cmp {
            return (self.suit as usize).cmp(&(other.suit as usize));
        }
        rank_cmp
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    pub fn new_random() -> Self {
        let card = rand::thread_rng().gen_range(0..32usize);
        let suit = match card / 8 {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Diamonds,
            3 => Suit::Clubs,
            _ => unreachable!(),
        };
        let rank = match card % 8 {
            0 => Rank::Ace,
            1 => Rank::Seven,
            2 => Rank::Eight,
            3 => Rank::Nine,
            4 => Rank::Ten,
            5 => Rank::Jack,
            6 => Rank::Queen,
            7 => Rank::King,
            _ => unreachable!(),
        };
        Self { suit, rank }
    }
}
