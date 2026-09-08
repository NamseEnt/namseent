use super::presentation_transition::diff_projection;
use crate::{
    PresentationInstant,
    card::{Card, CardId, Deck},
};
use namui::*;

#[derive(Clone, Copy, Debug, State)]
pub(crate) struct PresentationDeckCard {
    pub card: Card,
    pub order: usize,
    pub exit_started_at: Option<PresentationInstant>,
}

impl PresentationDeckCard {
    pub(crate) fn is_exiting(self) -> bool {
        self.exit_started_at.is_some()
    }
}

#[derive(Clone, Debug, State)]
pub(crate) struct PresentationDeckZones {
    zones: [Vec<PresentationDeckCard>; 3],
}

impl Default for PresentationDeckZones {
    fn default() -> Self {
        Self {
            zones: std::array::from_fn(|_| Vec::new()),
        }
    }
}

impl PresentationDeckZones {
    pub(crate) fn sync(
        &mut self,
        deck: &Deck,
        presentation_instant: PresentationInstant,
        restore: bool,
    ) {
        self.sync_zone(0, deck.all_cards(), presentation_instant, restore);
        self.sync_zone(1, deck.draw_pile(), presentation_instant, restore);
        self.sync_zone(2, deck.discard_pile(), presentation_instant, restore);
    }

    fn sync_zone(
        &mut self,
        zone: usize,
        cards: &[Card],
        presentation_instant: PresentationInstant,
        restore: bool,
    ) {
        let current = cards
            .iter()
            .map(|card| (card.id, ()))
            .collect::<Vec<(CardId, ())>>();
        let existing = self.zones[zone]
            .iter()
            .filter(|entry| !entry.is_exiting())
            .map(|entry| (entry.card.id, ()))
            .collect::<Vec<(CardId, ())>>();
        let _ = diff_projection(&existing, &current);

        if restore {
            self.zones[zone] = cards
                .iter()
                .copied()
                .enumerate()
                .map(|(order, card)| PresentationDeckCard {
                    card,
                    order,
                    exit_started_at: None,
                })
                .collect();
            return;
        }

        for entry in &mut self.zones[zone] {
            if let Some((order, card)) = cards
                .iter()
                .enumerate()
                .find(|(_, card)| card.id == entry.card.id)
            {
                entry.card = *card;
                entry.order = order;
                entry.exit_started_at = None;
            } else if entry.exit_started_at.is_none() {
                entry.exit_started_at = Some(presentation_instant);
            }
        }

        for (order, card) in cards.iter().enumerate() {
            if !self.zones[zone]
                .iter()
                .any(|entry| entry.card.id == card.id)
            {
                self.zones[zone].push(PresentationDeckCard {
                    card: *card,
                    order,
                    exit_started_at: None,
                });
            }
        }
        self.zones[zone].sort_by_key(|entry| (entry.is_exiting(), entry.order));
    }

    pub(crate) fn update(&mut self, presentation_instant: PresentationInstant) {
        for zone in &mut self.zones {
            zone.retain(|entry| {
                entry
                    .exit_started_at
                    .is_none_or(|started| (presentation_instant - started).as_secs_f32() < 0.5)
            });
        }
    }

    pub(crate) fn snapshot(&self, zone: usize) -> Vec<PresentationDeckCard> {
        self.zones[zone].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moved_cards_exit_the_old_zone_and_enter_the_new_zone() {
        let mut zones = PresentationDeckZones::default();
        let source = Deck::new();
        let first = source.all_cards()[1];
        let second = source.all_cards()[2];
        let mut raw = source.to_core_state();
        raw.all_cards = vec![first.to_core_state(), second.to_core_state()];
        raw.next_card_id = 3;
        raw.draw_pile.clear();
        raw.discard_pile = vec![second.to_core_state()];
        let initial = Deck::from_core_state(raw.clone()).expect("initial test deck");
        raw.draw_pile = vec![second.to_core_state()];
        raw.discard_pile.clear();
        let deck = Deck::from_core_state(raw).expect("moved test deck");

        zones.sync(&initial, PresentationInstant::zero(), true);
        zones.sync(&deck, PresentationInstant::zero(), false);

        assert!(
            zones
                .snapshot(2)
                .iter()
                .any(|entry| { entry.card.id == CardId::from_raw(2) && entry.is_exiting() })
        );
        assert!(
            zones
                .snapshot(1)
                .iter()
                .any(|entry| { entry.card.id == CardId::from_raw(2) && !entry.is_exiting() })
        );
    }
}
