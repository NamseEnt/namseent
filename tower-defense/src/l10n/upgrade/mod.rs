mod upgrade_kind;

use super::{Language, Locale, LocalizedText};

pub enum UpgradeKindText<'a> {
    Name(&'a crate::game_state::upgrade::UpgradeKind),
    Description(&'a crate::game_state::upgrade::UpgradeKind),
}

impl LocalizedText for UpgradeKindText<'_> {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

// 기존 Template 구조체들은 하위 호환성을 위해 유지
#[derive(Debug, Clone, Copy)]
pub enum TowerUpgradeTarget {
    Tower(crate::game_state::upgrade::TowerUpgradeTarget),
    TowerSelect(crate::game_state::upgrade::TowerSelectUpgradeTarget),
}

#[derive(Debug, Clone, Copy)]
pub enum WhatUpgrade {
    Damage,
    Speed,
    Range,
}

#[derive(Debug, Clone, Copy)]
pub enum AddOrMultiply {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub enum Template {
    TowerUpgrade {
        target: TowerUpgradeTarget,
        what_upgrade: WhatUpgrade,
        add_or_multiply: AddOrMultiply,
        how_much: f32,
    },
}

impl LocalizedText for Template {
    fn localized_text(&self, locale: &Locale) -> String {
        match locale.language {
            Language::Korean => self.to_korean(),
            Language::English => self.to_english(),
        }
    }
}

impl Template {
    pub fn to_korean(&self) -> String {
        "레거시 템플릿".to_string() // 간단한 구현
    }

    pub fn to_english(&self) -> String {
        "Legacy template".to_string()
    }

    pub fn from_kind(_kind: &crate::game_state::upgrade::UpgradeKind, _is_name: bool) -> Self {
        // 간단한 기본값 반환
        Template::TowerUpgrade {
            target: TowerUpgradeTarget::Tower(
                crate::game_state::upgrade::TowerUpgradeTarget::Rank {
                    rank: crate::card::Rank::Ace,
                },
            ),
            what_upgrade: WhatUpgrade::Damage,
            add_or_multiply: AddOrMultiply::Add,
            how_much: 0.0,
        }
    }
}
