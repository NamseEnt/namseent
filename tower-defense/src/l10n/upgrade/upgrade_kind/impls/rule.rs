use crate::game_state::upgrade::*;
use crate::l10n::locale::Language;
use crate::l10n::Locale;
use crate::theme::typography::TypographyBuilder;

use super::UpgradeKindL10n;

impl UpgradeKindL10n for FourLeafCloverUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Four Leaf Clover",
            Language::Korean => "네잎클로버",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Enable straight flush with 4 cards",
            Language::Korean => "스트레이트와 플러시를 4장으로 만들 수 있습니다",
        });
    }
}

impl UpgradeKindL10n for RabbitUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Rabbit",
            Language::Korean => "토끼",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Skip one rank in a straight",
            Language::Korean => "스트레이트를 만들 때 하나를 건너뛸 수 있습니다",
        });
    }
}

impl UpgradeKindL10n for BlackWhiteUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Black & White",
            Language::Korean => "흑백",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Treat all suits as one",
            Language::Korean => "하트와 다이아를, 클럽과 스페이드를 같은 문양으로 간주합니다",
        });
    }
}
