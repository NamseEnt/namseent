use crate::game_state::card::{Card, RANKS, SUITS};
use namui::*;
use rand::RngCore;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, State)]
pub struct Deck {
    all_cards: Vec<Card>,
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut all_cards = Vec::with_capacity(SUITS.len() * RANKS.len());
        for &rank in &RANKS {
            for &suit in &SUITS {
                all_cards.push(Card { suit, rank });
            }
        }
        Self {
            all_cards,
            draw_pile: Vec::new(),
            discard_pile: Vec::new(),
        }
    }

    pub fn prepare_draw_pile(&mut self, rng: &mut dyn RngCore) {
        self.draw_pile = self.all_cards.clone();
        self.discard_pile.clear();
        self.draw_pile.shuffle(rng);
    }

    pub fn draw(&mut self, rng: &mut dyn RngCore, count: usize) -> Vec<Card> {
        let mut cards = Vec::new();
        while cards.len() < count {
            let Some(card) = self.draw_pile.pop() else {
                if self.discard_pile.is_empty() {
                    break;
                }
                self.draw_pile = self.discard_pile.clone();
                self.discard_pile.clear();
                self.draw_pile.shuffle(rng);
                continue;
            };
            cards.push(card);
        }
        println!(
            "left: {}, discard: {}, drawn: {}",
            self.draw_pile.len(),
            self.discard_pile.len(),
            cards.len()
        );
        cards
    }

    pub fn discard(&mut self, cards: impl IntoIterator<Item = Card>) {
        self.discard_pile.extend(cards);
    }
}
