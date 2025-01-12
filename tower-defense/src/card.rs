use rand::Rng;
use std::fmt::Display;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
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
        write!(f, "{}", s)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Rank {
    Ace,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
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
        write!(f, "{}", s)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
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
