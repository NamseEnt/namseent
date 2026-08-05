use super::*;
use crate::{
    card::{CardId, Engraving},
    game_state::{
        GameState,
        action::{DeckEdit, DeckEditChange, DeckEnhance},
        set_modal,
    },
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CactusCardService;

impl CactusCardService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Cactus(self)
    }
}

impl CardServiceBehavior for CactusCardService {
    fn key(&self) -> &'static str {
        "cactus"
    }

    fn purchase_block_reasons(
        &self,
        context: &CardServicePurchaseContext,
    ) -> Vec<CardServicePurchaseBlockReason> {
        let available = context.unengraved_card_count;
        if available < 1 {
            vec![CardServicePurchaseBlockReason::NotEnoughUnengravedCards {
                required: 1,
                available,
            }]
        } else {
            Vec::new()
        }
    }

    fn acquire(self, game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        let title = match game_state.locale.language {
            crate::l10n::locale::Language::English => "Select a card to engrave",
            crate::l10n::locale::Language::Korean => "각인할 카드를 선택하세요",
        }
        .to_string();

        let selection = crate::game_state::modal::deck::CardSelectionState::new(
            vec![crate::game_state::modal::deck::CardSelectionStep {
                title,
                count: 1,
                filter: crate::game_state::modal::deck::CardSelectionFilter::NotEngraved,
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
                            changes: vec![DeckEditChange::SetEngraving(Some(Engraving::Cactus))],
                        })
                        .collect(),
                },
            ));
        }
    }

    fn thumbnail(&self, wh: Wh<Px>, stroke_px: Px, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::CACTUS,
            wh,
            stroke_px,
            shadow,
        )
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Cactus",
            crate::l10n::locale::Language::Korean => "선인장",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Select 1 card and engrave cactus on it.")
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("카드 1장을 선택해 선인장 각인을 부여합니다.")
            }
        };
    }

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![
            self.tooltip_section(locale),
            crate::l10n::word::Word::Engraving(Some(Engraving::Cactus)).tooltip_section(locale),
        ]
    }

    fn heuristic_best_selection(&self, game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        let card_id = game_state
            .deck
            .all_cards()
            .iter()
            .filter(|card| card.engraving().is_none())
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
        generate_cactus_card_service,
        || crate::Rarity::Common,
    );

fn generate_cactus_card_service() -> CardService {
    CactusCardService::new().into_card_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::card_service::CardServiceBehavior;

    #[test]
    fn cactus_engraving_adds_thirty_percent_splash_damage() {
        assert_eq!(
            Engraving::Cactus.tower_modifier().on_attack_splashes,
            vec![crate::card::EngravingSplash {
                radius: 2.0,
                damage_pct: 0.3,
            }]
        );
    }

    #[test]
    fn cactus_heuristic_selects_one_unengraved_card() {
        let game_state = crate::game_state::create_initial_game_state();

        let selected = CactusCardService.heuristic_best_selection(&game_state);

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].len(), 1);
        assert!(
            game_state
                .deck
                .get_card(selected[0][0])
                .unwrap()
                .engraving()
                .is_none()
        );
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn cactus_selection_only_engraves_the_selected_card() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let selected = game_state.deck.all_cards()[0].id;
        let unrelated = game_state.deck.all_cards()[1].id;

        CactusCardService.select_cards(&mut game_state, vec![vec![selected]]);

        assert_eq!(
            game_state.deck.get_card(selected).unwrap().engraving(),
            Some(Engraving::Cactus)
        );
        assert_eq!(
            game_state.deck.get_card(unrelated).unwrap().engraving(),
            None
        );
    }

    #[test]
    fn cactus_heuristic_handles_a_fully_engraved_deck() {
        let mut game_state = crate::game_state::create_initial_game_state();
        for card in game_state.deck.all_cards().to_vec() {
            game_state.deck.modify_card(card.id, |card| {
                card.effects.engraving = Some(Engraving::Magnet);
            });
        }

        assert_eq!(
            CactusCardService.heuristic_best_selection(&game_state),
            vec![vec![]]
        );
    }

    #[cfg(feature = "simulator")]
    #[test]
    fn cactus_headless_use_card_service_engraves_the_selected_card() {
        let mut game_state = crate::game_state::create_initial_game_state();
        let service = CactusCardService;
        let selected = service.heuristic_best_selection(&game_state)[0][0];
        game_state.headless = true;

        game_state.action(crate::game_state::GameStateAction::UseCardService(
            service.into_card_service(),
        ));

        assert_eq!(
            game_state.deck.get_card(selected).unwrap().engraving(),
            Some(Engraving::Cactus)
        );
    }
}
