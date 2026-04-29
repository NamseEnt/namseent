use crate::game_state::upgrade::*;
use crate::icon::IconKind;
use crate::l10n::locale::Language;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::l10n::Locale;
use crate::theme::typography::TypographyBuilder;

use super::UpgradeKindL10n;

impl UpgradeKindL10n for CatUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Cat",
            Language::Korean => "고양이",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Gain ")
                .with_icon_bold(IconKind::Gold, format!("+{}", self.add))
                .static_text(" on monster kills"),
            Language::Korean => builder
                .static_text("몬스터 처치 시 ")
                .with_icon_bold(IconKind::Gold, format!("{}", self.add)),
        };
    }
}

impl UpgradeKindL10n for StaffUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Staff",
            Language::Korean => "지팡이",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Diamond tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("다이아몬드 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for LongSwordUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Long Sword",
            Language::Korean => "롱소드",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Spade tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("스페이드 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for MaceUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Mace",
            Language::Korean => "메이스",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Heart tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("하트 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for ClubSwordUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Club",
            Language::Korean => "몽둥이",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Club tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("클럽 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for TricycleUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Tricycle",
            Language::Korean => "세발자전거",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("3-card tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("3장 이하 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for PerfectPotteryUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Perfect Pottery",
            Language::Korean => "완벽한 도자기",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("No-reroll tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("리롤 안한 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for SingleChopstickUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Single Chopstick",
            Language::Korean => "젓가락",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Odd-card tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("홀수 카드 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for PairChopsticksUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Pair Chopsticks",
            Language::Korean => "젓가락 세트",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Even-card tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("짝수 카드 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for FountainPenUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Fountain Pen",
            Language::Korean => "만년필",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Number-card tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("숫자 카드 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for BrushUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Brush",
            Language::Korean => "붓",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Face-card tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("그림 카드 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}

impl UpgradeKindL10n for BrokenPotteryUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Broken Pottery",
            Language::Korean => "깨진 도자기",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => builder
                .static_text("Rerolled tower ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
            Language::Korean => builder
                .static_text("리롤한 타워 ")
                .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier)),
        };
    }
}
