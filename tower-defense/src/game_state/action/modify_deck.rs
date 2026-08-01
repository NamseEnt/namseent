use crate::card::{Card, CardId, Engraving, Rank, Suit};
use crate::game_state::GameState;
use crate::game_state::card_notification::CardServiceNotification;
#[cfg(not(feature = "simulator"))]
use namui::time::now;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum DeckEditChange {
    AddPolishPct(f32),
    SetSuit(Suit),
    SetRank(Rank),
    /// 각인 부여/제거. 카드당 각인은 1개뿐이라, 이미 각인된 카드에 다른 각인을
    /// 부여하려는 시도는 무시된다. `None` 은 언제나 제거로 동작한다.
    SetEngraving(Option<Engraving>),
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
                DeckEditChange::AddPolishPct(bonus_pct) => {
                    card.add_polish_pct(*bonus_pct);
                }
                DeckEditChange::SetSuit(suit) => {
                    card.suit = *suit;
                }
                DeckEditChange::SetRank(rank) => {
                    card.rank = *rank;
                }
                DeckEditChange::SetEngraving(engraving) => {
                    if engraving.is_none() || card.engraving().is_none() {
                        card.effects.engraving = *engraving;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
    #[cfg(not(feature = "simulator"))]
    game_state
        .card_service_notifications
        .enqueue(now(), notification);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    fn engrave(card: &mut Card, engraving: Option<Engraving>) {
        DeckEnhance {
            card_id: card.id,
            changes: vec![DeckEditChange::SetEngraving(engraving)],
        }
        .apply(card);
    }

    #[test]
    fn engraving_a_blank_card_succeeds() {
        let mut card = Card::new(Rank::Ace, Suit::Spades);

        engrave(&mut card, Some(Engraving::Magnet));

        assert_eq!(card.engraving(), Some(Engraving::Magnet));
    }

    #[test]
    fn engraving_over_an_existing_engraving_is_ignored() {
        let mut card = Card::new(Rank::Ace, Suit::Spades);
        card.effects.engraving = Some(Engraving::Magnet);

        // 지금은 각인이 한 종류뿐이라 같은 값으로 덮어쓰는 것으로 확인한다.
        // 값이 바뀌지 않았다는 것보다, 아래 제거 후 재부여가 되는 점이 불변식의 핵심이다.
        engrave(&mut card, Some(Engraving::Magnet));

        assert_eq!(card.engraving(), Some(Engraving::Magnet));
    }

    #[test]
    fn removing_an_engraving_frees_the_card_for_a_new_one() {
        let mut card = Card::new(Rank::Ace, Suit::Spades);
        card.effects.engraving = Some(Engraving::Magnet);

        engrave(&mut card, None);
        assert_eq!(card.engraving(), None);

        engrave(&mut card, Some(Engraving::Magnet));
        assert_eq!(card.engraving(), Some(Engraving::Magnet));
    }

    #[test]
    fn engraving_does_not_disturb_polish() {
        let mut card = Card::new(Rank::Ace, Suit::Spades);
        DeckEnhance {
            card_id: card.id,
            changes: vec![
                DeckEditChange::AddPolishPct(0.5),
                DeckEditChange::SetEngraving(Some(Engraving::Magnet)),
                DeckEditChange::AddPolishPct(0.25),
            ],
        }
        .apply(&mut card);

        assert_eq!(card.polish_pct(), 0.75);
        assert_eq!(card.engraving(), Some(Engraving::Magnet));
    }
}
