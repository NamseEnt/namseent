use crate::game_state::card_service::definition::CardServiceDefinition;
use crate::theme::typography::TypographyBuilder;
use crate::{Rarity, game_state::GameState};
use enum_dispatch::enum_dispatch;
use namui::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[enum_dispatch]
pub trait CardServiceBehavior {
    fn key(&self) -> &'static str;

    fn acquire(self, _game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>;

    fn select_cards(
        self,
        game_state: &mut GameState,
        selected_card_ids: Vec<Vec<crate::card::CardId>>,
    ) where
        Self: Sized + Into<CardService>;

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale);

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    );

    fn thumbnail(&self, wh: Wh<Px>, stroke_px: Px, shadow: bool) -> RenderingTree;

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

mod brush;
mod club_sword;
mod fountain_pen;
mod long_sword;
mod mace;
mod staff;
mod tricycle;

use brush::BrushCardService;
use club_sword::ClubSwordCardService;
use fountain_pen::FountainPenCardService;
use long_sword::LongSwordCardService;
use mace::MaceCardService;
use staff::StaffCardService;
use tricycle::TricycleCardService;

#[enum_dispatch(CardServiceBehavior)]
#[derive(Clone, Debug, State, PartialEq, strum_macros::EnumDiscriminants)]
#[strum_discriminants(derive(strum_macros::EnumIter), name(CardServiceDiscriminants))]
pub enum CardService {
    LongSword(LongSwordCardService),
    Staff(StaffCardService),
    Mace(MaceCardService),
    ClubSword(ClubSwordCardService),
    Brush(BrushCardService),
    FountainPen(FountainPenCardService),
    Tricycle(TricycleCardService),
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
}

impl CardServiceDiscriminants {
    fn definition(self) -> CardServiceDefinition {
        match self {
            CardServiceDiscriminants::LongSword => long_sword::DEFINITION,
            CardServiceDiscriminants::Staff => staff::DEFINITION,
            CardServiceDiscriminants::Mace => mace::DEFINITION,
            CardServiceDiscriminants::ClubSword => club_sword::DEFINITION,
            CardServiceDiscriminants::Brush => brush::DEFINITION,
            CardServiceDiscriminants::FountainPen => fountain_pen::DEFINITION,
            CardServiceDiscriminants::Tricycle => tricycle::DEFINITION,
        }
    }

    pub(crate) fn generate(self) -> CardService {
        self.definition().generate()
    }

    pub(crate) fn rarity(self) -> Rarity {
        self.definition().rarity()
    }
}
