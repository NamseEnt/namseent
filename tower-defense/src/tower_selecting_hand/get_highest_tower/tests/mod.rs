mod high_card;
mod one_pair;
mod two_pair;
mod three_of_a_kind;
mod full_house;
mod straight;
mod flush;
mod four_of_a_kind;
mod straight_flush;
mod royal_flush;

pub fn make_card(suit: crate::card::Suit, rank: crate::card::Rank) -> crate::card::Card {
    crate::card::Card { suit, rank }
}
