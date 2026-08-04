use super::*;
use crate::{
    card::CardId,
    game_state::{
        GameState,
        action::{DeckEdit, DeckEditChange, DeckEnhance},
        set_modal,
    },
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PliersCardService;

impl PliersCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Pliers(self)
    }
}

impl CardServiceBehavior for PliersCardService {
    fn key(&self) -> &'static str {
        "pliers"
    }

    fn acquire(self, game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        let title = match game_state.locale.language {
            crate::l10n::locale::Language::English => {
                "Select an engraved card to remove its engraving"
            }
            crate::l10n::locale::Language::Korean => "각인을 제거할 카드를 선택하세요",
        }
        .to_string();

        let selection = crate::game_state::modal::deck::CardSelectionState::new(
            vec![crate::game_state::modal::deck::CardSelectionStep {
                title,
                count: 1,
                filter: crate::game_state::modal::deck::CardSelectionFilter::Engraved,
            }],
            self.into_card_service(),
        );

        set_modal(Some(crate::game_state::modal::UserModal::Deck(
            crate::game_state::modal::deck::DeckModal {
                deck_kind: crate::game_state::modal::deck::DeckKind::Deck,
                selection: Some(selection),
            },
        )));
    }

    fn select_cards(self, game_state: &mut GameState, selected_card_ids: Vec<Vec<CardId>>)
    where
        Self: Sized + Into<CardService>,
    {
        for card_ids in selected_card_ids {
            game_state.action(crate::game_state::GameStateAction::ModifyDeck(
                DeckEdit::Enhance {
                    enhances: card_ids
                        .into_iter()
                        .map(|card_id| DeckEnhance {
                            card_id,
                            changes: vec![DeckEditChange::SetEngraving(None)],
                        })
                        .collect(),
                },
            ));
        }
    }

    fn thumbnail(&self, wh: Wh<Px>, _stroke_px: Px, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::PLIERS,
            wh,
            crate::thumbnail::STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Pliers",
            crate::l10n::locale::Language::Korean => "플라이어",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select 1 engraved card and remove its engraving.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("각인된 카드 1장을 선택해 각인을 제거합니다.")
            }
        };
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let deck = &game_state.deck;
        let card_id = deck
            .all_cards()
            .iter()
            .filter(|card| card.engraving().is_some())
            .max_by(|a, b| {
                a.polish_pct()
                    .total_cmp(&b.polish_pct())
                    .then_with(|| a.rank.ordinal().cmp(&b.rank.ordinal()))
            })
            .map(|card| card.id)
            .into_iter()
            .collect();
        vec![card_id]
    }
}

pub(super) const DEFINITION: crate::game_state::card_service::definition::CardServiceDefinition =
    crate::game_state::card_service::definition::CardServiceDefinition::new(
        generate_pliers_card_service,
        || crate::Rarity::Common,
    );

fn generate_pliers_card_service() -> CardService {
    PliersCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Engraving;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn heuristic_selects_one_engraved_card() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let engraved = game_state.deck.all_cards()[0].id;
        game_state.deck.modify_card(engraved, |card| {
            card.effects.engraving = Some(Engraving::Cactus);
        });

        let selected = PliersCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected, vec![vec![engraved]]);
    }

    #[test]
    fn heuristic_handles_a_deck_without_engraved_cards() {
        let game_state = crate::game_state::create_initial_game_state();

        assert_eq!(
            PliersCardService.heuristic_best_selection(&game_state),
            vec![vec![]]
        );
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn selection_removes_only_the_selected_card_engraving() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let selected = game_state.deck.all_cards()[0].id;
        let unrelated = game_state.deck.all_cards()[1].id;
        game_state.deck.modify_card(selected, |card| {
            card.effects.engraving = Some(Engraving::Cactus);
            card.add_polish_pct(1.0);
        });
        game_state.deck.modify_card(unrelated, |card| {
            card.effects.engraving = Some(Engraving::Overcharge);
        });
        let polish = game_state.deck.get_card(selected).unwrap().polish_pct();

        PliersCardService.select_cards(&mut game_state, vec![vec![selected]]);

        assert_eq!(
            game_state.deck.get_card(selected).unwrap().engraving(),
            None
        );
        assert_eq!(
            game_state.deck.get_card(selected).unwrap().polish_pct(),
            polish
        );
        assert_eq!(
            game_state.deck.get_card(unrelated).unwrap().engraving(),
            Some(Engraving::Overcharge)
        );
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn headless_use_card_service_removes_the_selected_engraving() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let selected = game_state.deck.all_cards()[0].id;
        game_state.deck.modify_card(selected, |card| {
            card.effects.engraving = Some(Engraving::Cactus);
        });
        game_state.headless = true;

        game_state.action(crate::game_state::GameStateAction::UseCardService(
            PliersCardService.into_card_service(),
        ));

        assert_eq!(
            game_state.deck.get_card(selected).unwrap().engraving(),
            None
        );
    }
}
