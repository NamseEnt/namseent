use super::*;

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

    fn thumbnail_with_shadow(
        &self,
        width_height: Wh<Px>,
        stroke_px: Px,
        shadow: bool,
    ) -> RenderingTree {
        render_card(&self.card, width_height, stroke_px, shadow)
    }
}
