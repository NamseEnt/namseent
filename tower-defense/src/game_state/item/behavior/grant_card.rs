use super::*;
use rand::{Rng, RngCore};

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct GrantCardItem {
    pub card: Card,
}

impl GrantCardItem {
    pub fn new(card: Card) -> Self {
        Self { card }
    }

    pub fn into_item(self) -> Item {
        Item::GrantCard(self)
    }
}

impl ItemBehavior for GrantCardItem {
    fn key(&self) -> &'static str {
        "grant_card"
    }

    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        game_state.action(crate::game_state::GameStateAction::GrantHandItem(
            crate::hand::HandItem::Card(self.card),
        ));
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "급조카드",
            crate::l10n::Language::English => "Emergency Card",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        let card = self.card;
        match locale.language {
            crate::l10n::Language::Korean => {
                builder
                    .card_rank(card.rank)
                    .card_suit(card.suit)
                    .static_text(" 카드 획득");
            }
            crate::l10n::Language::English => {
                builder
                    .card_rank(card.rank)
                    .card_suit(card.suit)
                    .static_text(" card");
            }
        }
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Card(&self.card)
    }
}

pub(super) const DEFINITION: crate::game_state::item::definition::ItemDefinition =
    crate::game_state::item::definition::ItemDefinition::new(generate_grant_card_item, || {
        crate::Rarity::Rare
    });

fn generate_grant_card_item(rng: &mut dyn RngCore) -> Item {
    let suit =
        crate::game_state::card::SUITS[rng.gen_range(0..crate::game_state::card::SUITS.len())];
    let rank =
        crate::game_state::card::RANKS[rng.gen_range(0..crate::game_state::card::RANKS.len())];
    GrantCardItem::new(Card::new(rank, suit)).into_item()
}
