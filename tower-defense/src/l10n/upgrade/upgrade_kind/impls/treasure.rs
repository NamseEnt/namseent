use crate::game_state::upgrade::*;
use crate::icon::IconKind;
use crate::l10n::locale::Language;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::Locale;
use crate::theme::typography::TypographyBuilder;

use super::UpgradeKindL10n;

impl UpgradeKindL10n for BackpackUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Backpack",
            Language::Korean => "배낭",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Shop slot ")
                .with_icon_bold(IconKind::Shop, format!("+{}", self.add)),
            Language::Korean => builder.with_icon_bold(IconKind::Shop, "상점 슬롯 +1"),
        };
    }
}

impl UpgradeKindL10n for DiceBundleUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Dice Bundle",
            Language::Korean => "주사위 꾸러미",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Dice ")
                .with_icon_bold(IconKind::Refresh, format!("+{}", self.add)),
            Language::Korean => builder.with_icon_bold(IconKind::Refresh, "+1"),
        };
    }
}

impl UpgradeKindL10n for EnergyDrinkUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Energy Drink",
            Language::Korean => "에너지드링크",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Shop price ")
                .with_icon_bold(IconKind::Gold, format!("-{}", self.add)),
            Language::Korean => builder
                .static_text("상점 가격 ")
                .with_icon_bold(IconKind::Gold, format!("-{}", self.add))
                .static_text(" 할인"),
        };
    }
}

impl UpgradeKindL10n for EraserUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Eraser",
            Language::Korean => "지우개",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Remove ")
                .with_positive_effect(format!("{} rank", self.add))
                .static_text(" from the deck"),
            Language::Korean => {
                let desc = Box::leak(format!("덱에서 {}개 숫자카드를 제거합니다", self.add).into_boxed_str());
                builder.static_text(desc)
            }
        };
    }
}
