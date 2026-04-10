use crate::*;
use rand::Rng;
use rand::RngCore;
use rand::seq::SliceRandom;
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

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord, State)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
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
            Rank::Two => 0,
            Rank::Three => 0,
            Rank::Four => 0,
            Rank::Five => 1,
            Rank::Six => 1,
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
        matches!(
            self,
            Rank::Two | Rank::Four | Rank::Six | Rank::Eight | Rank::Ten | Rank::Queen
        )
    }
    pub fn is_face(&self) -> bool {
        matches!(self, Rank::Jack | Rank::Queen | Rank::King)
    }
    pub fn is_number_card(&self) -> bool {
        matches!(
            self,
            Rank::Two
                | Rank::Three
                | Rank::Four
                | Rank::Five
                | Rank::Six
                | Rank::Seven
                | Rank::Eight
                | Rank::Nine
                | Rank::Ten
        )
    }
    pub fn ordinal(&self) -> usize {
        match self {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12,
        }
    }
    pub fn ace_low_value(&self) -> usize {
        if matches!(self, Rank::Ace) {
            0
        } else {
            self.ordinal() + 1
        }
    }
    pub fn ace_high_value(&self) -> usize {
        if matches!(self, Rank::Ace) {
            13
        } else {
            self.ordinal() + 1
        }
    }
}
impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{s}")
    }
}
pub const RANKS: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];
pub const REVERSED_RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::King,
    Rank::Queen,
    Rank::Jack,
    Rank::Ten,
    Rank::Nine,
    Rank::Eight,
    Rank::Seven,
    Rank::Six,
    Rank::Five,
    Rank::Four,
    Rank::Three,
    Rank::Two,
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
        let total_cards = SUITS.len() * RANKS.len();
        let card = rand::thread_rng().gen_range(0..total_cards);
        let suit = SUITS[card / RANKS.len()];
        let rank = RANKS[card % RANKS.len()];
        Self { suit, rank }
    }
    pub fn face_image(&self) -> Image {
        (self.rank, self.suit).image()
    }
}

#[derive(Debug, Clone, State)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(removed_number_rank_count: usize) -> Self {
        let removed_number_rank_count = removed_number_rank_count.min(5);
        let mut cards = Vec::with_capacity(SUITS.len() * RANKS.len());

        for &rank in &RANKS {
            if rank.ordinal() < removed_number_rank_count {
                continue;
            }
            for &suit in &SUITS {
                cards.push(Card { suit, rank });
            }
        }
        cards.shuffle(&mut rand::thread_rng());
        Self { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn put_back(&mut self, cards: impl IntoIterator<Item = Card>) {
        self.cards.extend(cards);
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn sample<R: RngCore + ?Sized>(&self, count: usize, rng: &mut R) -> Vec<Card> {
        let available = self.cards.len().min(count);
        let mut cards = self
            .cards
            .choose_multiple(rng, available)
            .copied()
            .collect::<Vec<_>>();

        for _ in available..count {
            cards.push(Card::new_random());
        }

        cards
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}

pub trait FaceCardImage {
    fn image(self) -> Image;
}

impl FaceCardImage for (Rank, Suit) {
    fn image(self) -> Image {
        let (rank, suit) = self;
        match (rank, suit) {
            (Rank::Jack, Suit::Spades) => crate::asset::image::face::spades::JACK,
            (Rank::Jack, Suit::Hearts) => crate::asset::image::face::hearts::JACK,
            (Rank::Jack, Suit::Diamonds) => crate::asset::image::face::diamonds::JACK,
            (Rank::Jack, Suit::Clubs) => crate::asset::image::face::clubs::JACK,
            (Rank::Queen, Suit::Spades) => crate::asset::image::face::spades::QUEEN,
            (Rank::Queen, Suit::Hearts) => crate::asset::image::face::hearts::QUEEN,
            (Rank::Queen, Suit::Diamonds) => crate::asset::image::face::diamonds::QUEEN,
            (Rank::Queen, Suit::Clubs) => crate::asset::image::face::clubs::QUEEN,
            (Rank::King, Suit::Spades) => crate::asset::image::face::spades::KING,
            (Rank::King, Suit::Hearts) => crate::asset::image::face::hearts::KING,
            (Rank::King, Suit::Diamonds) => crate::asset::image::face::diamonds::KING,
            (Rank::King, Suit::Clubs) => crate::asset::image::face::clubs::KING,
            _ => panic!("Not a face card: {:?} {:?}", rank, suit),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_removes_two_rank_when_level_one() {
        let mut deck = Deck::new(1);
        let cards: Vec<Card> = std::iter::from_fn(|| deck.draw()).collect();

        assert_eq!(cards.len(), 48);
        assert!(cards.iter().all(|card| card.rank != Rank::Two));
    }

    #[test]
    fn test_deck_removes_two_through_six_when_level_five() {
        let mut deck = Deck::new(5);
        let cards: Vec<Card> = std::iter::from_fn(|| deck.draw()).collect();

        assert_eq!(cards.len(), 32);
        assert!(cards.iter().all(|card| {
            !matches!(
                card.rank,
                Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six
            )
        }));
    }
}
