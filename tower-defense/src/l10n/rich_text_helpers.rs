use crate::icon::{Icon, IconKind};

// 통합 Rich text 헬퍼 함수들 - 모든 l10n 모듈에서 공유
// 색상과 아이콘의 일관성을 위해 중앙화된 함수들

// === 범위/거리 관련 ===
pub fn range<T: std::fmt::Display>(value: T) -> String {
    format!("|blue|{value}|/blue|")
}

pub fn beam_thickness<T: std::fmt::Display>(value: T) -> String {
    format!("|blue|{value}|/blue|")
}

// === 시간 관련 ===
pub fn time_duration<T: std::fmt::Display>(value: T) -> String {
    format!("|yellow|{value}|/yellow|")
}

// === 골드 관련 ===
pub fn gold_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::Gold);
    format!("|gold_color|{}{value}|/gold_color|", icon.as_tag())
}

pub fn gold_icon_small<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::Gold).size(crate::icon::IconSize::Small);
    format!("|gold_color|{}{value}|/gold_color|", icon.as_tag())
}

// === 공격력 관련 ===
pub fn attack_damage_stat(stat_name: &str) -> String {
    let icon = Icon::new(IconKind::AttackDamage);
    format!(
        "{}|attack_damage_color|{stat_name}|/attack_damage_color|",
        icon.as_tag()
    )
}

// === 수치 증가/곱셈 표시 (색상은 +/× 구분에 따라) ===
pub fn additive_value<T: std::fmt::Display>(value: T) -> String {
    format!("|green|+{value}|/green|")
}

pub fn multiplier_value<T: std::fmt::Display>(value: T) -> String {
    format!("|blue|x{value}|/blue|")
}

// === 카드 랭크 표시 ===
pub fn card_rank<T: std::fmt::Display>(rank: T) -> String {
    format!("|purple|{rank}|/purple|")
}

// === 특수 아이템 텍스트 ===
pub fn special_item_text<T: std::fmt::Display>(text: T) -> String {
    format!("|blue|{text}|/blue|")
}

// === 이동 속도 디버프 ===
pub fn movement_speed_debuff_text(text: &str) -> String {
    format!("|red|{text}|/red|")
}

pub fn movement_speed_debuff_value<T: std::fmt::Display>(value: T) -> String {
    format!("|red|{value}|/red|")
}

// === 백분율 증가/감소 표시 ===
pub fn percentage_increase<T: std::fmt::Display>(value: T) -> String {
    format!("|green|+{value}%|/green|")
}

pub fn percentage_decrease<T: std::fmt::Display>(value: T) -> String {
    format!("|red|-{value}%|/red|")
}

// === 절대값 증가 ===
pub fn value_increase<T: std::fmt::Display>(value: T) -> String {
    format!("|green|+{value}|/green|")
}

// === 특수 용도 색상 함수들 ===
pub fn shield_value<T: std::fmt::Display>(value: T) -> String {
    format!("|blue|{value}|/blue|")
}

pub fn reduction_percentage<T: std::fmt::Display>(value: T) -> String {
    format!("|green|{value}%|/green|")
}

pub fn attack_damage_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackDamage);
    format!(
        "|attack_damage_color|{}{value}|/attack_damage_color|",
        icon.as_tag()
    )
}

pub fn attack_damage_icon_small<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackDamage).size(crate::icon::IconSize::Small);
    format!(
        "|attack_damage_color|{}{value}|/attack_damage_color|",
        icon.as_tag()
    )
}

// === 공격 속도 관련 ===
pub fn attack_speed_stat(stat_name: &str) -> String {
    let icon = Icon::new(IconKind::AttackSpeed);
    format!(
        "{}|attack_speed_color|{stat_name}|/attack_speed_color|",
        icon.as_tag()
    )
}

pub fn attack_speed_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackSpeed);
    format!(
        "|attack_speed_color|{}{value}|/attack_speed_color|",
        icon.as_tag()
    )
}

pub fn attack_speed_icon_small<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackSpeed).size(crate::icon::IconSize::Small);
    format!(
        "|attack_speed_color|{}{value}|/attack_speed_color|",
        icon.as_tag()
    )
}

// === 사정거리 관련 ===
pub fn attack_range_stat(stat_name: &str) -> String {
    let icon = Icon::new(IconKind::AttackRange);
    format!(
        "{}|attack_range_color|{stat_name}|/attack_range_color|",
        icon.as_tag()
    )
}

pub fn attack_range_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackRange);
    format!(
        "|attack_range_color|{}{value}|/attack_range_color|",
        icon.as_tag()
    )
}

pub fn attack_range_icon_small<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::AttackRange).size(crate::icon::IconSize::Small);
    format!(
        "|attack_range_color|{}{value}|/attack_range_color|",
        icon.as_tag()
    )
}

// === 체력 관련 ===
pub fn heal_icon<T: std::fmt::Display>(value: T) -> String {
    let icon = Icon::new(IconKind::Health);
    format!("|gold_color|{}{value}|/gold_color|", icon.as_tag())
}

// === 기본 아이콘들 ===
pub fn shop_icon() -> String {
    Icon::new(IconKind::Shop).as_tag()
}

pub fn quest_icon() -> String {
    Icon::new(IconKind::Contract).as_tag()
}

pub fn refresh_icon() -> String {
    Icon::new(IconKind::Refresh).as_tag()
}

// === 문양 아이콘 ===
pub fn suit_icon(suit: crate::card::Suit) -> String {
    Icon::new(IconKind::Suit { suit }).as_tag()
}

// === 문양 색상 헬퍼 ===
pub fn with_suit_color(text: String, suit: crate::card::Suit) -> String {
    use crate::card::Suit;
    let color_tag = match suit {
        Suit::Spades | Suit::Clubs => "black_suit_color",
        Suit::Hearts | Suit::Diamonds => "red_suit_color",
    };
    format!("|{color_tag}|{text}|/{color_tag}|")
}

// === 계약 텍스트 색상 ===
pub fn contract_risk<T: std::fmt::Display>(text: T) -> String {
    format!("|red|{text}|/red|")
}

pub fn contract_reward<T: std::fmt::Display>(text: T) -> String {
    format!("|blue|{text}|/blue|")
}

// === 계약 기간 텍스트 색상 ===
pub fn contract_duration<T: std::fmt::Display>(text: T) -> String {
    format!("|yellow|{text}|/yellow|")
}
