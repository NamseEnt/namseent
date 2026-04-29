use crate::game_state::upgrade::*;
use crate::icon::IconKind;
use crate::l10n::Locale;
use crate::l10n::locale::Language;
use crate::l10n::rich_text_helpers::RichTextHelpers;
use crate::theme::typography::TypographyBuilder;

use super::UpgradeKindL10n;

impl UpgradeKindL10n for TrophyUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Trophy",
            Language::Korean => "트로피",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Perfect clears stack to increase global damage",
            Language::Korean => "완전 클리어가 쌓일수록 전역 피해가 증가합니다",
        });
    }
}

impl UpgradeKindL10n for CrockUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Crock",
            Language::Korean => "항아리",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Damage increases with your gold",
            Language::Korean => "골드가 많을수록 피해가 증가합니다",
        });
    }
}

impl UpgradeKindL10n for DemolitionHammerUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Demolition Hammer",
            Language::Korean => "파괴 망치",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Removing a tower boosts damage on the next stage",
            Language::Korean => "타워를 제거하면 다음 스테이지 피해가 증가합니다",
        });
    }
}

impl UpgradeKindL10n for MetronomeUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Metronome",
            Language::Korean => "메트로놈",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Gain 1 extra dice every 2 stages",
            Language::Korean => "2스테이지마다 주사위 +1을 얻습니다",
        });
    }
}

impl UpgradeKindL10n for TapeUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Tape",
            Language::Korean => "테이프",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Slow enemies every 4 waves after acquisition",
            Language::Korean => "획득 후 4웨이브마다 적 속도가 느려집니다",
        });
    }
}

impl UpgradeKindL10n for NameTagUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Name Tag",
            Language::Korean => "이름표",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "The next placed tower gains bonus damage",
            Language::Korean => "다음 배치한 타워가 추가 피해를 얻습니다",
        });
    }
}

impl UpgradeKindL10n for ShoppingBagUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Shopping Bag",
            Language::Korean => "쇼핑백",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Purchased items increase global damage",
            Language::Korean => "구매한 아이템마다 전역 피해가 증가합니다",
        });
    }
}

impl UpgradeKindL10n for ResolutionUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Resolution",
            Language::Korean => "결심",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Remaining rerolls add damage to the next tower",
            Language::Korean => "남은 리롤 수만큼 다음 타워 피해가 증가합니다",
        });
    }
}

impl UpgradeKindL10n for MirrorUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Mirror",
            Language::Korean => "거울",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Duplicate the next acquired tower",
            Language::Korean => "다음 획득한 타워를 복제합니다",
        });
    }
}

impl UpgradeKindL10n for IceCreamUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Ice Cream",
            Language::Korean => "아이스크림",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => {
                let waves_text =
                    Box::leak(format!("{} waves", self.waves_remaining).into_boxed_str());
                builder
                    .static_text("Damage ")
                    .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier))
                    .static_text(" for ")
                    .static_text(waves_text)
            }
            Language::Korean => {
                let waves_text =
                    Box::leak(format!("{}웨이브", self.waves_remaining).into_boxed_str());
                builder
                    .static_text("다음 ")
                    .static_text(waves_text)
                    .static_text(" 동안 피해 ")
                    .with_icon_bold(IconKind::Damage, format!("X{:.1}", self.damage_multiplier))
            }
        };
    }
}

impl UpgradeKindL10n for SpannerUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Spanner",
            Language::Korean => "스패너",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Keep shield across stage transitions",
            Language::Korean => "스테이지 전환 시 방패를 유지합니다",
        });
    }
}

impl UpgradeKindL10n for PeaUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Pea",
            Language::Korean => "완두콩",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Increase max HP by 10 and heal to full",
            Language::Korean => "최대 체력이 10 증가하고 즉시 회복합니다",
        });
    }
}

impl UpgradeKindL10n for SlotMachineUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Slot Machine",
            Language::Korean => "슬롯머신",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => {
                let dice_text =
                    Box::leak(format!("{} extra dice", self.next_round_dice).into_boxed_str());
                builder
                    .static_text("Gain ")
                    .static_text(dice_text)
                    .static_text(" next stage")
            }
            Language::Korean => {
                let dice_text = Box::leak(format!("{}개", self.next_round_dice).into_boxed_str());
                builder
                    .static_text("다음 스테이지에 주사위 ")
                    .static_text(dice_text)
                    .static_text("를 얻습니다")
            }
        };
    }
}

impl UpgradeKindL10n for PiggyBankUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Piggy Bank",
            Language::Korean => "돼지저금통",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "If you have 500 gold, earn 50 gold after each stage",
            Language::Korean => "골드가 500 이상일 때 스테이지 종료 후 50골드를 얻습니다",
        });
    }
}

impl UpgradeKindL10n for CameraUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Camera",
            Language::Korean => "카메라",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Gain 50 gold when placing a face tower",
            Language::Korean => "페이스 타워를 배치하면 50골드를 얻습니다",
        });
    }
}

impl UpgradeKindL10n for GiftBoxUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Gift Box",
            Language::Korean => "선물 상자",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Earn 10 gold per item at the end of each stage",
            Language::Korean => "각 아이템마다 스테이지 종료 시 10골드를 얻습니다",
        });
    }
}

impl UpgradeKindL10n for FangUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Fang",
            Language::Korean => "송곳니",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Recover 1 HP when a monster dies",
            Language::Korean => "몬스터가 죽을 때마다 1HP를 회복합니다",
        });
    }
}

impl UpgradeKindL10n for PopcornUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Popcorn",
            Language::Korean => "팝콘",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match locale.language {
            Language::English => {
                let duration_text = Box::leak(format!("{} waves", self.duration).into_boxed_str());
                builder
                    .static_text("Damage boost lasts for ")
                    .static_text(duration_text)
                    .static_text(" and decreases each wave")
            }
            Language::Korean => {
                let duration_text =
                    Box::leak(format!("{}웨이브 동안 피해 증가", self.duration).into_boxed_str());
                builder
                    .static_text(duration_text)
                    .static_text("가 매 웨이브마다 감소합니다")
            }
        };
    }
}

impl UpgradeKindL10n for MembershipCardUpgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Membership Card",
            Language::Korean => "멤버십 카드",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        builder.static_text(match locale.language {
            Language::English => "Get a free shop this stage",
            Language::Korean => "이번 스테이지 상점이 무료가 됩니다",
        });
    }
}
