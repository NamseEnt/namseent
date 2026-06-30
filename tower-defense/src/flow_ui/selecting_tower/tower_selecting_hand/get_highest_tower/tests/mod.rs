use crate::game_state::card::{Card, Rank, Suit};

mod flush;
mod four_of_a_kind;
mod full_house;
mod high_card;
mod one_pair;
mod royal_flush;
mod straight;
mod straight_flush;
mod three_of_a_kind;
mod two_pair;

pub fn make_card(suit: Suit, rank: Rank) -> Card {
    Card { suit, rank }
}
