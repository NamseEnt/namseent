use super::{Language, Locale, LocalizedText};
use crate::Rarity;
use crate::theme::typography::TypographyBuilder;
use namui::*;

#[derive(Debug, Clone, Copy, State)]
pub struct RarityText(pub Rarity);

impl From<Rarity> for RarityText {
    fn from(rarity: Rarity) -> Self {
        Self(rarity)
    }
}

impl LocalizedText for RarityText {
    fn apply_to_builder<'a>(self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        });
    }
}

impl RarityText {
    pub(super) const fn to_korean(self) -> &'static str {
        match self.0 {
            Rarity::Common => "일반",
            Rarity::Rare => "희귀",
            Rarity::Epic => "에픽",
            Rarity::Legendary => "전설",
        }
    }

    pub(super) const fn to_english(self) -> &'static str {
        match self.0 {
            Rarity::Common => "Common",
            Rarity::Rare => "Rare",
            Rarity::Epic => "Epic",
            Rarity::Legendary => "Legendary",
        }
    }
}
