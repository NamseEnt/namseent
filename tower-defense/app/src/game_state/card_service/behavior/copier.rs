use super::*;
#[cfg(test)]
use crate::{
    card::{Card, Rank},
    game_state::GameState,
};

#[cfg(test)]
const ROYAL_RANKS: [Rank; 5] = [Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CopierCardService;

impl CopierCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Copier(self)
    }
}

impl CardServiceBehavior for CopierCardService {
    fn key(&self) -> &'static str {
        "copier"
    }

    fn acquire_selection_steps(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::game_state::modal::deck::CardSelectionStep> {
        let title = match locale.language {
            crate::l10n::locale::Language::English => "Select a card to copy",
            crate::l10n::locale::Language::Korean => "복제할 카드를 선택하세요",
        }
        .to_string();

        vec![crate::game_state::modal::deck::CardSelectionStep {
            title,
            count: 1,
            filter: crate::game_state::modal::deck::CardSelectionFilter::Any,
        }]
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::COPIER)
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Copier",
            crate::l10n::locale::Language::Korean => "복사기",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select 1 card and add a copy of it to the deck.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드를 1장 선택해 복제본을 덱에 추가합니다.")
            }
        };
    }

    #[cfg(test)]
    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let deck = headed_cards_from_raw_core(game_state.raw_core_state());
        let card_id = deck
            .iter()
            .map(|card| (copy_priority(card, &deck), card.id))
            .max_by(|(a, _), (b, _)| {
                a.0.cmp(&b.0)
                    .then_with(|| (a.1, a.2, a.3).cmp(&(b.1, b.2, b.3)))
            })
            .map(|(_, card_id)| card_id)
            .into_iter()
            .collect();
        vec![card_id]
    }
}

#[cfg(test)]
fn copy_priority(card: &Card, deck: &[Card]) -> (crate::FixedRatio, usize, usize, usize) {
    let enhancement = card.polish_pct();

    let suit_count = deck.iter().filter(|other| other.suit == card.suit).count();

    let royal_gain = match ROYAL_RANKS.contains(&card.rank) {
        true => {
            let count_of = |rank: Rank| {
                deck.iter()
                    .filter(|other| other.suit == card.suit && other.rank == rank)
                    .count()
            };
            ROYAL_RANKS
                .iter()
                .map(|&rank| count_of(rank))
                .product::<usize>()
                / count_of(card.rank)
        }
        false => 0,
    };

    (enhancement, suit_count, royal_gain, card.rank.ordinal())
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_copier_card_service,
        || crate::Rarity::Epic,
    );

fn generate_copier_card_service() -> CardService {
    CopierCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn copier_heuristic_selects_a_royal_card_in_a_fresh_deck() {
        let game_state = crate::game_state::create_initial_game_state();
        let selected_card_id = CopierCardService.heuristic_best_selection(&game_state)[0][0];

        assert_eq!(
            game_state.deck.get_card(selected_card_id).unwrap().rank,
            Rank::Ace
        );
    }

    #[test]
    fn copier_heuristic_prefers_the_most_enhanced_card() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let low_card_id = game_state
            .deck
            .all_cards()
            .iter()
            .find(|card| card.rank == Rank::Two)
            .unwrap()
            .id;
        game_state.deck.modify_card(low_card_id, |card| {
            card.add_polish_pct(crate::FixedRatio::from_raw(500_000));
        });
        game_state.sync_raw_core_from_projection();

        let selected_card_id = CopierCardService.heuristic_best_selection(&game_state)[0][0];

        assert_eq!(selected_card_id, low_card_id);
    }

    #[test]
    fn copier_heuristic_prefers_the_densest_suit() {
        let mut game_state = crate::game_state::create_initial_game_state();
        for rank in [Rank::Two, Rank::Three, Rank::Four] {
            game_state
                .deck
                .add_card(Card::new(rank, crate::card::Suit::Clubs));
        }
        game_state.sync_raw_core_from_projection();

        let selected_card = game_state
            .deck
            .get_card(CopierCardService.heuristic_best_selection(&game_state)[0][0])
            .unwrap();

        assert_eq!(selected_card.suit, crate::card::Suit::Clubs);
        assert_eq!(selected_card.rank, Rank::Ace);
    }

    #[test]
    fn copier_heuristic_targets_the_scarcest_royal_rank_of_the_completed_suit() {
        let mut game_state = crate::game_state::create_initial_game_state();
        for rank in [Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
            game_state
                .deck
                .add_card(Card::new(rank, crate::card::Suit::Spades));
        }
        game_state.sync_raw_core_from_projection();

        let selected_card = game_state
            .deck
            .get_card(CopierCardService.heuristic_best_selection(&game_state)[0][0])
            .unwrap();

        assert_eq!(selected_card.suit, crate::card::Suit::Spades);
        assert_eq!(selected_card.rank, Rank::Ten);
    }

    #[cfg(test)]
    #[test]
    fn copier_headless_use_card_service_adds_a_copy_of_the_selected_card() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let service = CopierCardService;
        let selected_card_id = service.heuristic_best_selection(&game_state)[0][0];
        let selected_card = game_state.deck.get_card(selected_card_id).unwrap();
        let card_count = game_state.deck.all_cards().len();
        game_state.headless = true;

        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::UseCardService {
                card_service: service.into_card_service(),
                locale: crate::l10n::Locale::KOREAN,
            },
        );

        assert_eq!(game_state.deck.all_cards().len(), card_count + 1);
        let copy = game_state.deck.all_cards().last().unwrap();
        assert_ne!(copy.id, selected_card.id);
        assert_eq!(copy.rank, selected_card.rank);
        assert_eq!(copy.suit, selected_card.suit);
    }
}
