use crate::Rarity;
use crate::game_state::card_service::definition::CardServiceDefinition;
use crate::theme::typography::TypographyBuilder;
#[cfg(test)]
use crate::{card::Card, game_state::GameState};
use enum_dispatch::enum_dispatch;
use namui::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(test)]
pub(crate) fn headed_cards_from_raw_core(core: &td_core::CoreState) -> Vec<Card> {
    core.deck()
        .all_cards
        .iter()
        .cloned()
        .filter_map(Card::from_core_state)
        .collect()
}

#[enum_dispatch]
pub(crate) trait CardServiceBehavior {
    fn key(&self) -> &'static str;

    #[cfg(test)]
    fn acquire(self, game_state: &mut GameState, locale: crate::l10n::Locale)
    where
        Self: Sized + Into<CardService>,
    {
        let selection = crate::game_state::modal::deck::CardSelectionState::new(
            self.acquire_selection_steps(locale),
            self.into(),
        );
        game_state.set_card_service_selection(selection);
    }

    fn acquire_selection_steps(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::game_state::modal::deck::CardSelectionStep>;

    #[cfg(test)]
    fn heuristic_best_selection(&self, _game_state: &GameState) -> Vec<Vec<crate::card::CardId>> {
        vec![vec![]]
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale);

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    );

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_>;

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![self.tooltip_section(locale)]
    }

    fn tooltip_section(&self, locale: crate::l10n::Locale) -> crate::tooltip::TooltipSection<'_> {
        crate::tooltip::TooltipSection {
            title: Some(crate::tooltip::SectionText {
                key: format!("card_service:{}:name", self.key()),
                apply: Box::new(move |builder| {
                    self.l10n_name(builder, &locale);
                }),
            }),
            body: crate::tooltip::SectionText {
                key: format!("card_service:{}:desc", self.key()),
                apply: Box::new(move |builder| {
                    self.l10n_description(builder, &locale);
                }),
            },
        }
    }
}

#[cfg(test)]
pub(crate) fn validate_selection(
    deck: &crate::card::Deck,
    selection: &crate::game_state::modal::deck::CardSelectionState,
    selected_card_ids: &[Vec<crate::card::CardId>],
) -> Result<(), crate::game_state::CommandError> {
    if selected_card_ids.len() != selection.steps.len() {
        return Err(crate::game_state::CommandError::InvalidSelection);
    }

    let mut selected_ids = std::collections::HashSet::new();
    for (step, card_ids) in selection.steps.iter().zip(selected_card_ids) {
        if card_ids.len() != step.count {
            return Err(crate::game_state::CommandError::InvalidSelection);
        }
        for card_id in card_ids {
            if !selected_ids.insert(*card_id) {
                return Err(crate::game_state::CommandError::InvalidSelection);
            }
            let Some(card) = deck.get_card(*card_id) else {
                return Err(crate::game_state::CommandError::InvalidIndex);
            };
            if !step.filter.matches(&card) {
                return Err(crate::game_state::CommandError::InvalidSelection);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{CardServiceBehavior, validate_selection};
    use crate::config::GameConfig;
    use crate::game_state::card_service::CardServiceDiscriminants;
    use crate::game_state::modal::deck::{
        CardSelectionFilter, CardSelectionState, CardSelectionStep,
    };

    #[test]
    fn selection_validation_rejects_invalid_shape_cards_and_duplicates() {
        let game_state = crate::game_state::create_game_state_with_config(
            std::sync::Arc::new(GameConfig::default_config()),
            7,
        );
        let deck = game_state.presentation_deck_snapshot();
        let card = deck.all_cards()[0];
        let number_card = *game_state
            .presentation_deck_snapshot()
            .all_cards()
            .iter()
            .find(|card| card.rank.is_number_card())
            .expect("the default deck contains a number card");
        let face_card = *game_state
            .presentation_deck_snapshot()
            .all_cards()
            .iter()
            .find(|card| card.rank.is_face())
            .expect("the default deck contains a face card");
        let service = CardServiceDiscriminants::Eraser.generate();

        let one_card_selection = CardSelectionState::new(
            vec![CardSelectionStep {
                title: String::new(),
                count: 1,
                filter: CardSelectionFilter::Any,
            }],
            service.clone(),
        );
        assert_eq!(
            validate_selection(&deck, &one_card_selection, &[]),
            Err(crate::game_state::CommandError::InvalidSelection)
        );
        assert_eq!(
            validate_selection(
                &deck,
                &one_card_selection,
                &[vec![crate::card::CardId::from_raw(u64::MAX as usize)]],
            ),
            Err(crate::game_state::CommandError::InvalidIndex)
        );

        let face_selection = CardSelectionState::new(
            vec![CardSelectionStep {
                title: String::new(),
                count: 1,
                filter: CardSelectionFilter::Face,
            }],
            service.clone(),
        );
        assert_eq!(
            validate_selection(&deck, &face_selection, &[vec![number_card.id]],),
            Err(crate::game_state::CommandError::InvalidSelection)
        );
        assert_eq!(
            validate_selection(&deck, &face_selection, &[vec![face_card.id]],),
            Ok(())
        );

        let duplicate_selection = CardSelectionState::new(
            vec![
                CardSelectionStep {
                    title: String::new(),
                    count: 1,
                    filter: CardSelectionFilter::Any,
                },
                CardSelectionStep {
                    title: String::new(),
                    count: 1,
                    filter: CardSelectionFilter::Any,
                },
            ],
            service,
        );
        assert_eq!(
            validate_selection(&deck, &duplicate_selection, &[vec![card.id], vec![card.id]],),
            Err(crate::game_state::CommandError::InvalidSelection)
        );
    }

    #[test]
    fn core_card_service_catalog_has_presentation_and_localization_coverage() {
        for &kind in td_core::CardServiceKind::ALL {
            let discriminant = CardServiceDiscriminants::from_core_kind(kind);
            let service = discriminant.generate();
            assert_eq!(discriminant.to_core_kind(), kind);
            assert_eq!(service.key(), kind.key());
            assert_eq!(
                td_core::CardServiceSelectionState::service_key(kind),
                service.key()
            );
        }
    }
}

mod battery;
mod brush;
mod cactus;
mod club_sword;
mod copier;
mod eraser;
mod fountain_pen;
mod long_sword;
mod mace;
mod magic_wand;
mod magnet;
mod pliers;
mod screwdriver;
mod spinning_top;
mod staff;
mod tricycle;

use battery::BatteryCardService;
use brush::BrushCardService;
use cactus::CactusCardService;
use club_sword::ClubSwordCardService;
use copier::CopierCardService;
use eraser::EraserCardService;
use fountain_pen::FountainPenCardService;
use long_sword::LongSwordCardService;
use mace::MaceCardService;
use magic_wand::MagicWandCardService;
use magnet::MagnetCardService;
use pliers::PliersCardService;
use screwdriver::ScrewdriverCardService;
use spinning_top::SpinningTopCardService;
use staff::StaffCardService;
use tricycle::TricycleCardService;

#[enum_dispatch(CardServiceBehavior)]
#[derive(Clone, Debug, State, PartialEq, strum_macros::EnumDiscriminants)]
#[strum_discriminants(
    derive(strum_macros::EnumIter, strum_macros::AsRefStr),
    name(CardServiceDiscriminants)
)]
pub enum CardService {
    LongSword(LongSwordCardService),
    Staff(StaffCardService),
    Mace(MaceCardService),
    ClubSword(ClubSwordCardService),
    Brush(BrushCardService),
    FountainPen(FountainPenCardService),
    Tricycle(TricycleCardService),
    Eraser(EraserCardService),
    MagicWand(MagicWandCardService),
    Pliers(PliersCardService),
    Screwdriver(ScrewdriverCardService),
    Copier(CopierCardService),
    Magnet(MagnetCardService),
    Battery(BatteryCardService),
    Cactus(CactusCardService),
    SpinningTop(SpinningTopCardService),
}

#[derive(Debug, Clone, Copy, State, PartialEq, Eq)]
pub struct CardServiceId(pub u64);

#[derive(Debug, Clone, State, PartialEq)]
pub struct CardServiceWithId {
    pub id: CardServiceId,
    pub card_service: CardService,
}

static NEXT_CARD_SERVICE_ID: AtomicU64 = AtomicU64::new(1);

impl CardServiceWithId {
    pub fn new(card_service: CardService) -> Self {
        Self {
            id: CardServiceId(NEXT_CARD_SERVICE_ID.fetch_add(1, Ordering::Relaxed)),
            card_service,
        }
    }
}

impl std::ops::Deref for CardServiceWithId {
    type Target = CardService;

    fn deref(&self) -> &Self::Target {
        &self.card_service
    }
}

impl std::ops::DerefMut for CardServiceWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.card_service
    }
}

impl PartialEq<CardService> for CardServiceWithId {
    fn eq(&self, other: &CardService) -> bool {
        self.card_service == *other
    }
}

impl CardService {
    pub fn with_unique_id(self) -> CardServiceWithId {
        CardServiceWithId::new(self)
    }

    pub fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        CardServiceBehavior::thumbnail_source(self)
    }
}

impl CardServiceDiscriminants {
    pub const fn to_core_kind(self) -> td_core::CardServiceKind {
        match self {
            Self::LongSword => td_core::CardServiceKind::LongSword,
            Self::Staff => td_core::CardServiceKind::Staff,
            Self::Mace => td_core::CardServiceKind::Mace,
            Self::ClubSword => td_core::CardServiceKind::ClubSword,
            Self::Brush => td_core::CardServiceKind::Brush,
            Self::FountainPen => td_core::CardServiceKind::FountainPen,
            Self::Tricycle => td_core::CardServiceKind::Tricycle,
            Self::Eraser => td_core::CardServiceKind::Eraser,
            Self::MagicWand => td_core::CardServiceKind::MagicWand,
            Self::Pliers => td_core::CardServiceKind::Pliers,
            Self::Screwdriver => td_core::CardServiceKind::Screwdriver,
            Self::Copier => td_core::CardServiceKind::Copier,
            Self::Magnet => td_core::CardServiceKind::Magnet,
            Self::Cactus => td_core::CardServiceKind::Cactus,
            Self::SpinningTop => td_core::CardServiceKind::SpinningTop,
            Self::Battery => td_core::CardServiceKind::Battery,
        }
    }

    #[deprecated(note = "use to_core_kind().raw() at serialized boundaries")]
    pub const fn to_core_raw(self) -> u8 {
        self.to_core_kind().raw()
    }

    #[deprecated(note = "use from_core_kind(CardServiceKind::from_raw(...))")]
    pub const fn from_core_raw(value: u8) -> Option<Self> {
        match td_core::CardServiceKind::from_raw(value) {
            Some(kind) => Some(Self::from_core_kind(kind)),
            None => None,
        }
    }

    pub const fn from_core_kind(kind: td_core::CardServiceKind) -> Self {
        match kind {
            td_core::CardServiceKind::LongSword => Self::LongSword,
            td_core::CardServiceKind::Staff => Self::Staff,
            td_core::CardServiceKind::Mace => Self::Mace,
            td_core::CardServiceKind::ClubSword => Self::ClubSword,
            td_core::CardServiceKind::Brush => Self::Brush,
            td_core::CardServiceKind::FountainPen => Self::FountainPen,
            td_core::CardServiceKind::Tricycle => Self::Tricycle,
            td_core::CardServiceKind::Eraser => Self::Eraser,
            td_core::CardServiceKind::MagicWand => Self::MagicWand,
            td_core::CardServiceKind::Pliers => Self::Pliers,
            td_core::CardServiceKind::Screwdriver => Self::Screwdriver,
            td_core::CardServiceKind::Copier => Self::Copier,
            td_core::CardServiceKind::Magnet => Self::Magnet,
            td_core::CardServiceKind::Cactus => Self::Cactus,
            td_core::CardServiceKind::SpinningTop => Self::SpinningTop,
            td_core::CardServiceKind::Battery => Self::Battery,
        }
    }

    fn definition(self) -> CardServiceDefinition {
        match self {
            CardServiceDiscriminants::LongSword => long_sword::DEFINITION,
            CardServiceDiscriminants::Staff => staff::DEFINITION,
            CardServiceDiscriminants::Mace => mace::DEFINITION,
            CardServiceDiscriminants::ClubSword => club_sword::DEFINITION,
            CardServiceDiscriminants::Brush => brush::DEFINITION,
            CardServiceDiscriminants::FountainPen => fountain_pen::DEFINITION,
            CardServiceDiscriminants::Tricycle => tricycle::DEFINITION,
            CardServiceDiscriminants::Eraser => eraser::DEFINITION,
            CardServiceDiscriminants::MagicWand => magic_wand::DEFINITION,
            CardServiceDiscriminants::Pliers => pliers::DEFINITION,
            CardServiceDiscriminants::Screwdriver => screwdriver::DEFINITION,
            CardServiceDiscriminants::Copier => copier::DEFINITION,
            CardServiceDiscriminants::Magnet => magnet::DEFINITION,
            CardServiceDiscriminants::Cactus => cactus::DEFINITION,
            CardServiceDiscriminants::SpinningTop => spinning_top::DEFINITION,
            CardServiceDiscriminants::Battery => battery::DEFINITION,
        }
    }

    pub fn generate(self) -> CardService {
        self.definition().generate()
    }

    pub fn rarity(self) -> Rarity {
        self.definition().rarity()
    }
}
