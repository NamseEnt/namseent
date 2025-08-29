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

pub fn make_card(suit: crate::card::Suit, rank: crate::card::Rank) -> crate::card::Card {
    crate::card::Card { suit, rank }
}
