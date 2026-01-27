use crate::icon::IconKind;
use crate::theme::palette;
use crate::theme::typography::TypographyBuilder;

/// Typography Builder extension trait for rich text helpers
pub trait RichTextHelpers<'a> {
    fn with_range<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_attack_damage_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_percentage_increase<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_percentage_decrease<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_value_increase<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_multiplier<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_attack_speed_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_attack_range_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_gold_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_card_rank<S: Into<String>>(self, rank: S) -> TypographyBuilder<'a>;
    fn with_heal_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_shield_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_special_item_text<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_suit_color<S: Into<String>>(
        self,
        text: S,
        suit: crate::card::Suit,
    ) -> TypographyBuilder<'a>;
    fn with_attack_damage_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a>;
    fn with_attack_speed_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a>;
    fn with_attack_range_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a>;
    fn with_movement_speed_debuff_text<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_movement_speed_debuff_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_reduction_percentage<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
    fn with_contract_risk<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_contract_reward<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_contract_duration<S: Into<String>>(self, text: S) -> TypographyBuilder<'a>;
    fn with_time_duration<S: Into<String>>(self, value: S) -> TypographyBuilder<'a>;
}

impl<'a> RichTextHelpers<'a> for TypographyBuilder<'a> {
    fn with_range<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::BLUE)
    }

    fn with_attack_damage_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackDamage)
            .text(value.into())
            .color(palette::RED)
    }

    fn with_percentage_increase<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("+{}%", value.into()))
            .color(palette::COMMON)
    }

    fn with_percentage_decrease<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("-{}%", value.into())).color(palette::RED)
    }

    fn with_value_increase<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("+{}", value.into()))
            .color(palette::COMMON)
    }

    fn with_multiplier<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("x{}", value.into())).color(palette::BLUE)
    }

    fn with_attack_speed_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackSpeed)
            .text(value.into())
            .color(palette::YELLOW)
    }

    fn with_attack_range_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::AttackRange)
            .text(value.into())
            .color(palette::BLUE)
    }

    fn with_gold_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::Gold)
            .text(value.into())
            .color(palette::YELLOW)
    }

    fn with_card_rank<S: Into<String>>(self, rank: S) -> TypographyBuilder<'a> {
        self.text(rank.into()).color(palette::EPIC)
    }

    fn with_heal_icon<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.icon::<()>(IconKind::Gold)
            .text(value.into())
            .color(palette::YELLOW)
    }

    fn with_shield_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::BLUE)
    }

    fn with_special_item_text<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::BLUE)
    }

    fn with_suit_color<S: Into<String>>(
        self,
        text: S,
        suit: crate::card::Suit,
    ) -> TypographyBuilder<'a> {
        use crate::card::Suit;
        let color = match suit {
            Suit::Spades | Suit::Clubs => palette::COMMON,
            Suit::Hearts | Suit::Diamonds => palette::RED,
        };
        self.text(text.into()).color(color)
    }

    fn with_attack_damage_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a> {
        self.text(stat_name.into()).color(palette::RED)
    }

    fn with_attack_speed_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a> {
        self.text(stat_name.into()).color(palette::YELLOW)
    }

    fn with_attack_range_stat<S: Into<String>>(self, stat_name: S) -> TypographyBuilder<'a> {
        self.text(stat_name.into()).color(palette::BLUE)
    }

    fn with_movement_speed_debuff_text<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::RED)
    }

    fn with_movement_speed_debuff_value<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::RED)
    }

    fn with_reduction_percentage<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(format!("{}%", value.into()))
            .color(palette::COMMON)
    }

    fn with_contract_risk<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::RED)
    }

    fn with_contract_reward<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::BLUE)
    }

    fn with_contract_duration<S: Into<String>>(self, text: S) -> TypographyBuilder<'a> {
        self.text(text.into()).color(palette::YELLOW)
    }

    fn with_time_duration<S: Into<String>>(self, value: S) -> TypographyBuilder<'a> {
        self.text(value.into()).color(palette::YELLOW)
    }
}

// === 일부 String 반환 헬퍼 함수들 (UI preview 등 특수 용도) ===

/// 증가값 포맷팅 (UI preview에서만 사용)
pub fn additive_value<T: std::fmt::Display>(value: T) -> String {
    format!("+{}", value)
}

/// 배수값 포맷팅 (UI preview에서만 사용)
pub fn multiplier_value<T: std::fmt::Display>(value: T) -> String {
    format!("x{}", value)
}

// === 레거시 String 함수들 (tower_skill.rs, quest.rs 등에서 사용) ===
// 이 함수들은 점진적으로 trait 메서드로 대체될 예정입니다.

/// 범위/거리 표시
pub fn range<T: std::fmt::Display>(value: T) -> String {
    format!("{value}")
}

/// 백분율 증가
pub fn percentage_increase<T: std::fmt::Display>(value: T) -> String {
    format!("+{}%", value)
}

/// 백분율 감소
pub fn percentage_decrease<T: std::fmt::Display>(value: T) -> String {
    format!("-{}%", value)
}

/// 절대값 증가
pub fn value_increase<T: std::fmt::Display>(value: T) -> String {
    format!("+{}", value)
}

/// 공격력 아이콘 (작은 버전)
pub fn attack_damage_icon_small<T: std::fmt::Display>(value: T) -> String {
    format!("⚔ {}", value)
}

/// 공격 속도 아이콘 (작은 버전)
pub fn attack_speed_icon_small<T: std::fmt::Display>(value: T) -> String {
    format!("⚡ {}", value)
}

/// 사정거리 아이콘 (작은 버전)
pub fn attack_range_icon_small<T: std::fmt::Display>(value: T) -> String {
    format!("🎯 {}", value)
}

/// 골드 아이콘 (작은 버전)
pub fn gold_icon_small<T: std::fmt::Display>(value: T) -> String {
    format!("💰 {}", value)
}

/// 카드 랭크
pub fn card_rank<T: std::fmt::Display>(rank: T) -> String {
    format!("{}", rank)
}

/// 문양 아이콘
pub fn suit_icon(suit: crate::card::Suit) -> String {
    format!("{:?}", suit)
}

/// 방어막 값
pub fn shield_value<T: std::fmt::Display>(value: T) -> String {
    format!("{}", value)
}

/// 특수 아이템 텍스트
pub fn special_item_text<T: std::fmt::Display>(text: T) -> String {
    format!("{}", text)
}

/// 체력 아이콘
pub fn heal_icon<T: std::fmt::Display>(value: T) -> String {
    format!("❤ {}", value)
}

/// 공격력 아이콘
pub fn attack_damage_icon<T: std::fmt::Display>(value: T) -> String {
    format!("⚔ {}", value)
}

/// 공격 속도 아이콘
pub fn attack_speed_icon<T: std::fmt::Display>(value: T) -> String {
    format!("⚡ {}", value)
}

/// 사정거리 아이콘
pub fn attack_range_icon<T: std::fmt::Display>(value: T) -> String {
    format!("🎯 {}", value)
}

/// 공격력 스탯 명칭
pub fn attack_damage_stat(stat_name: &str) -> String {
    format!("{}", stat_name)
}

/// 공격 속도 스탯 명칭
pub fn attack_speed_stat(stat_name: &str) -> String {
    format!("{}", stat_name)
}

/// 사정거리 스탯 명칭
pub fn attack_range_stat(stat_name: &str) -> String {
    format!("{}", stat_name)
}

/// 감소 백분율
pub fn reduction_percentage<T: std::fmt::Display>(value: T) -> String {
    format!("{}%", value)
}

/// 시간 표시
pub fn time_duration<T: std::fmt::Display>(value: T) -> String {
    format!("{}", value)
}

/// 상점 아이콘
pub fn shop_icon() -> String {
    "[Shop]".to_string()
}

/// 새로고침 아이콘
pub fn refresh_icon() -> String {
    "[Refresh]".to_string()
}

/// 골드 아이콘
pub fn gold_icon<T: std::fmt::Display>(value: T) -> String {
    format!("💰 {}", value)
}
