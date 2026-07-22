use crate::card::{Card, CardId, Rank, Suit};
use crate::game_state::GameState;
use crate::game_state::card_notification::CardServiceNotification;
use namui::time::now;

#[derive(Debug, Clone)]
pub(crate) enum DeckEditChange {
    AddDamageBonusPct(f32),
    SetSuit(Suit),
    SetRank(Rank),
}

#[derive(Debug, Clone)]
pub(crate) struct DeckEnhance {
    pub(crate) card_id: CardId,
    pub(crate) changes: Vec<DeckEditChange>,
}
impl DeckEnhance {
    pub(crate) fn apply(&self, card: &mut Card) {
        for change in &self.changes {
            match change {
                DeckEditChange::AddDamageBonusPct(bonus_pct) => {
                    card.add_damage_bonus_pct(*bonus_pct);
                }
                DeckEditChange::SetSuit(suit) => {
                    card.suit = *suit;
                }
                DeckEditChange::SetRank(rank) => {
                    card.rank = *rank;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum DeckEdit {
    Add { cards: Vec<Card> },
    Remove { card_ids: Vec<CardId> },
    Enhance { enhances: Vec<DeckEnhance> },
}

pub(super) fn apply(game_state: &mut GameState, edit: DeckEdit) {
    let mut notification = CardServiceNotification::new();
    match edit {
        DeckEdit::Add { cards } => {
            for card in cards {
                game_state.deck.add_card(card);
                notification.added(card);
            }
        }
        DeckEdit::Remove { card_ids } => {
            for card_id in card_ids {
                if let Some(card) = game_state.deck.remove_card(card_id) {
                    notification.removed(card);
                }
            }
        }
        DeckEdit::Enhance { enhances } => {
            for enhance in enhances {
                let Some(from) = game_state.deck.get_card(enhance.card_id) else {
                    continue;
                };
                let Some(to) = game_state.deck.modify_card(enhance.card_id, |card| {
                    enhance.apply(card);
                }) else {
                    continue;
                };
                notification.enhanced(from, to);
            }
        }
    }
    game_state
        .card_service_notifications
        .enqueue(now(), notification);
}
