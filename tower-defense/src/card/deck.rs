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
        self.pull_magnet_engraved_cards(&mut cards);
        cards
    }

    /// 자석 각인 카드를 뽑았다면 뽑을 카드 더미에 남은 자석 카드를 모두 딸려 보낸다.
    /// 버린 카드 더미는 건드리지 않는다.
    ///
    /// 각인의 덱 이벤트 채널이 걸리는 지점이다. 실제 드로우 경로가 스테이지 시작과
    /// 카드 리롤 둘로 나뉘어 있어, 양쪽이 공유하는 `draw` 안에서 처리한다.
    fn pull_magnet_engraved_cards(&mut self, drawn: &mut Vec<Card>) {
        let is_magnet = |card: &Card| card.engraving() == Some(crate::card::Engraving::Magnet);

        if !drawn.iter().any(is_magnet) {
            return;
        }

        let mut pulled = Vec::new();
        self.draw_pile.retain(|card| {
            if is_magnet(card) {
                pulled.push(*card);
                false
            } else {
                true
            }
        });
        drawn.extend(pulled);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Engraving, Rank, Suit};

    /// draw_pile 을 지정한 순서로 세팅한다. `draw` 는 뒤에서부터 pop 하므로
    /// 마지막 원소가 가장 먼저 뽑힌다.
    fn deck_with_draw_pile(draw_pile: Vec<Card>) -> Deck {
        let mut deck = Deck::new();
        deck.all_cards = draw_pile.clone();
        deck.draw_pile = draw_pile;
        deck.discard_pile = Vec::new();
        deck
    }

    fn magnet_card(rank: Rank) -> Card {
        let mut card = Card::new(rank, Suit::Spades);
        card.effects.engraving = Some(Engraving::Magnet);
        card
    }

    #[test]
    fn drawing_a_magnet_card_pulls_every_magnet_card_left_in_the_draw_pile() {
        let plain = Card::new(Rank::Two, Suit::Hearts);
        let buried_magnet = magnet_card(Rank::King);
        let drawn_magnet = magnet_card(Rank::Ace);
        // 마지막 원소가 먼저 뽑히므로 drawn_magnet 한 장만 정상 드로우된다.
        let mut deck = deck_with_draw_pile(vec![buried_magnet, plain, drawn_magnet]);

        let drawn = deck.draw(&mut rand::thread_rng(), 1);

        assert_eq!(drawn.len(), 2);
        assert!(drawn.iter().any(|card| card.id == drawn_magnet.id));
        assert!(drawn.iter().any(|card| card.id == buried_magnet.id));
        // 딸려온 자석 카드는 더미에서 빠지고, 일반 카드는 남는다.
        assert_eq!(deck.draw_pile().len(), 1);
        assert_eq!(deck.draw_pile()[0].id, plain.id);
    }

    #[test]
    fn drawing_only_plain_cards_pulls_nothing() {
        let buried_magnet = magnet_card(Rank::King);
        let plain = Card::new(Rank::Two, Suit::Hearts);
        let mut deck = deck_with_draw_pile(vec![buried_magnet, plain]);

        let drawn = deck.draw(&mut rand::thread_rng(), 1);

        assert_eq!(drawn.len(), 1);
        assert_eq!(drawn[0].id, plain.id);
        assert_eq!(deck.draw_pile().len(), 1);
    }

    #[test]
    fn magnet_pull_leaves_the_discard_pile_alone() {
        let discarded_magnet = magnet_card(Rank::Queen);
        let drawn_magnet = magnet_card(Rank::Ace);
        let mut deck = deck_with_draw_pile(vec![drawn_magnet]);
        deck.discard_pile = vec![discarded_magnet];

        let drawn = deck.draw(&mut rand::thread_rng(), 1);

        assert_eq!(drawn.len(), 1);
        assert_eq!(deck.discard_pile().len(), 1);
        assert_eq!(deck.discard_pile()[0].id, discarded_magnet.id);
    }
}
