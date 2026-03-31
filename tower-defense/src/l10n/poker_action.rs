use super::{Language, Locale, LocalizedText};
use crate::{game_state::poker_action::NextStageOffer, *};

#[derive(Clone, State)]
pub enum NextStageOfferText {
    Description(NextStageOffer),
}

impl LocalizedText for NextStageOfferText {
    fn apply_to_builder<'a>(
        self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &Locale,
    ) {
        match locale.language {
            Language::Korean => self.apply_korean(builder),
            Language::English => self.apply_english(builder),
        }
    }
}

impl NextStageOfferText {
    fn apply_korean<'a>(self, builder: &mut crate::theme::typography::TypographyBuilder<'a>) {
        match self {
            NextStageOfferText::Description(NextStageOffer::None) => builder.text("다음 오퍼 없음"),
            NextStageOfferText::Description(NextStageOffer::Shop) => {
                builder.text("상인이 방문합니다")
            }
            NextStageOfferText::Description(NextStageOffer::TreasureSelection) => {
                builder.text("보물상자를 획득합니다")
            }
        };
    }

    fn apply_english<'a>(self, builder: &mut crate::theme::typography::TypographyBuilder<'a>) {
        match self {
            NextStageOfferText::Description(NextStageOffer::None) => {
                builder.text("Next offer: none")
            }
            NextStageOfferText::Description(NextStageOffer::Shop) => {
                builder.text("Shopkeeper visits")
            }
            NextStageOfferText::Description(NextStageOffer::TreasureSelection) => {
                builder.text("Treasure chest obtained")
            }
        };
    }
}
