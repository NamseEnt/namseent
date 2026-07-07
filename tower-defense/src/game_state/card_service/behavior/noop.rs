use super::*;
use crate::{
    Rarity, game_state::card_service::definition::CardServiceDefinition,
    theme::typography::TypographyBuilder,
};

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CardServiceNoop;

impl CardServiceNoop {
    pub fn standard() -> Self {
        Self
    }

    pub fn into_card_service(self) -> CardService {
        CardService::Noop(self)
    }
}

impl CardServiceBehavior for CardServiceNoop {
    fn key(&self) -> &'static str {
        "card_service_noop"
    }

    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.text("No Op");
        let _ = locale;
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.text("No effect placeholder.");
        let _ = locale;
    }

    fn acquire(self, _game_state: &mut GameState)
    where
        Self: Sized + Into<CardService>,
    {
        todo!(
            "Interaction for card service is not implemented yet. Deck modal will be used to select cards from the deck."
        );
    }

    fn thumbnail(&self, _wh: Wh<Px>, _stroke_px: Px, _shadow: bool) -> RenderingTree {
        RenderingTree::Empty
    }
}

pub(super) const DEFINITION: CardServiceDefinition =
    CardServiceDefinition::new(|| CardService::Noop(CardServiceNoop), || Rarity::Common);
