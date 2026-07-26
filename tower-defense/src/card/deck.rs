use crate::card::{Card, CardId, RANKS, SUITS};
use namui::*;
use rand::RngCore;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, State)]
pub struct Deck {
    revision: usize,
    all_cards: Vec<Card>,
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut all_cards = Vec::with_capacity(SUITS.len() * RANKS.len());
        for &rank in &RANKS {
            for &suit in &SUITS {
                all_cards.push(Card::new(rank, suit));
            }
        }
        Self {
            revision: 0,
            all_cards,
            draw_pile: Vec::new(),
            discard_pile: Vec::new(),
        }
    }

    pub fn all_cards(&self) -> &[Card] {
        &self.all_cards
    }

    pub fn get_card(&self, card_id: CardId) -> Option<Card> {
        self.all_cards
            .iter()
            .copied()
            .find(|card| card.id == card_id)
    }

    pub fn draw_pile(&self) -> &[Card] {
        &self.draw_pile
    }

    pub fn discard_pile(&self) -> &[Card] {
        &self.discard_pile
    }

    pub fn apply_to_card<F>(&mut self, card_id: CardId, mut f: F)
    where
        F: FnMut(&mut Card),
    {
        for card in self.all_cards.iter_mut() {
            if card.id == card_id {
                f(card);
            }
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.increment_revision();
        self.all_cards.push(card);
        if self.draw_pile.is_empty() {
            self.discard_pile.push(card);
        } else {
            self.draw_pile.push(card);
        }
    }

    pub fn remove_card(&mut self, card_id: CardId) -> Option<Card> {
        let index = self.all_cards.iter().position(|card| card.id == card_id)?;
        self.increment_revision();
        let removed = self.all_cards.remove(index);
        self.draw_pile.retain(|card| card.id != card_id);
        self.discard_pile.retain(|card| card.id != card_id);
        Some(removed)
    }

    pub fn modify_card<F>(&mut self, card_id: CardId, mut f: F) -> Option<Card>
    where
        F: FnMut(&mut Card),
    {
        let index = self.all_cards.iter().position(|card| card.id == card_id)?;
        {
            let card = &mut self.all_cards[index];
            f(card);
        }
        self.increment_revision();
        Some(self.all_cards[index])
    }

    pub fn apply_to_card_ids<F>(&mut self, card_ids: impl IntoIterator<Item = CardId>, mut f: F)
    where
        F: FnMut(&mut Card),
    {
        for card_id in card_ids {
            self.apply_to_card(card_id, &mut f);
        }
    }

    fn increment_revision(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn prepare_draw_pile(&mut self, rng: &mut dyn RngCore) {
        self.increment_revision();
        self.draw_pile = self.all_cards.clone();
        self.discard_pile.clear();
        self.draw_pile.shuffle(rng);
    }

    pub fn draw(&mut self, rng: &mut dyn RngCore, count: usize) -> Vec<Card> {
        self.increment_revision();
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
        self.increment_revision();
        self.discard_pile.extend(cards);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision
    }
}
