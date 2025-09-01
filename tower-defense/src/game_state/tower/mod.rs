mod render;
mod skill;

use super::{upgrade::TowerUpgradeState, *};
use crate::card::{Rank, Suit};
use crate::l10n::tower::TowerKindText;
use namui::*;
use render::Animation;
pub use render::{AnimationKind, tower_animation_tick};
pub use skill::*;
use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Clone, PartialEq)]
pub struct Tower {
    id: usize,
    pub left_top: MapCoord,
    cooldown: Duration,
    template: TowerTemplate,
    pub status_effects: Vec<TowerStatusEffect>,
    pub skills: Vec<TowerSkill>,
    pub(self) animation: Animation,
}
impl Tower {
    pub fn new(template: &TowerTemplate, left_top: MapCoord, now: Instant) -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            left_top,
            cooldown: Duration::from_secs(0),
            template: template.clone(),
            status_effects: vec![],
            skills: vec![],
            animation: Animation::new(now),
        }
    }
    pub fn in_cooltime(&self) -> bool {
        self.cooldown > Duration::from_secs(0)
    }

    pub fn shoot(
        &mut self,
        target_indicator: ProjectileTargetIndicator,
        tower_upgrade_states: &[TowerUpgradeState],
        now: Instant,
    ) -> Projectile {
        self.cooldown = self.shoot_interval;
        self.animation.transition(AnimationKind::Attack, now);

        Projectile {
            kind: self.projectile_kind,
            xy: self.left_top.map(|t| t as f32 + 0.5),
            velocity: self.projectile_speed,
            target_indicator,
            damage: self.calculate_projectile_damage(tower_upgrade_states),
        }
    }

    fn center_xy(&self) -> MapCoord {
        self.left_top + MapCoord::new(1, 1)
    }
    pub fn center_xy_f32(&self) -> MapCoordF32 {
        self.center_xy().map(|t| t as f32)
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn calculate_projectile_damage(&self, tower_upgrade_states: &[TowerUpgradeState]) -> f32 {
        let mut damage = self.default_damage;

        self.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::DamageAdd { add } = status_effect.kind {
                damage += add;
            }
        });

        tower_upgrade_states.iter().for_each(|tower_upgrade_state| {
            damage += tower_upgrade_state.damage_plus;
        });

        if damage < 0.0 {
            return 0.0;
        }

        self.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::DamageMul { mul } = status_effect.kind {
                damage *= mul;
            }
        });

        tower_upgrade_states.iter().for_each(|tower_upgrade_state| {
            damage *= tower_upgrade_state.damage_multiplier;
        });

        damage
    }

    pub(crate) fn attack_range_radius(&self, tower_upgrade_states: &[TowerUpgradeState]) -> f32 {
        if self.kind == TowerKind::Barricade {
            return 0.0;
        }
        self.status_effects.iter().fold(
            self.default_attack_range_radius,
            |attack_range_radius, status_effect| {
                if let TowerStatusEffectKind::AttackRangeAdd { add } = status_effect.kind {
                    attack_range_radius + add
                } else {
                    attack_range_radius
                }
            },
        ) + tower_upgrade_states
            .iter()
            .fold(0.0, |r, tower_upgrade_state| {
                r + tower_upgrade_state.range_plus
            })
    }
}
impl Deref for Tower {
    type Target = TowerTemplate;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TowerTemplate {
    pub kind: TowerKind,
    pub shoot_interval: Duration,
    pub default_attack_range_radius: f32,
    pub projectile_kind: ProjectileKind,
    pub projectile_speed: Velocity,
    pub default_damage: f32,
    pub suit: Suit,
    pub rank: Rank,
    pub skill_templates: Vec<TowerSkillTemplate>,
    pub default_status_effects: Vec<TowerStatusEffect>,
}
impl TowerTemplate {
    pub fn new(kind: TowerKind, suit: Suit, rank: Rank) -> Self {
        Self {
            kind,
            shoot_interval: kind.shoot_interval(),
            default_attack_range_radius: kind.default_attack_range_radius(),
            projectile_kind: ProjectileKind::Ball,
            projectile_speed: Per::new(48.0, 1.sec()),
            default_damage: kind.default_damage() as f32,
            suit,
            rank,
            skill_templates: kind.skill_templates(),
            default_status_effects: vec![],
        }
    }
    pub fn barricade() -> Self {
        Self::new(TowerKind::Barricade, Suit::Spades, Rank::Ace)
    }
}
impl PartialOrd for TowerTemplate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // 타워끼리는 kind(역순) -> suit -> rank 순으로 정렬
        Some(
            self.kind
                .cmp(&other.kind)
                .then_with(|| self.suit.cmp(&other.suit))
                .then_with(|| self.rank.cmp(&other.rank)),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TowerKind {
    Barricade,
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl TowerKind {
    pub fn asset_id(&self) -> &'static str {
        match self {
            TowerKind::Barricade => "barricade",
            TowerKind::High => "high",
            TowerKind::OnePair => "one_pair",
            TowerKind::TwoPair => "two_pair",
            TowerKind::ThreeOfAKind => "three_of_a_kind",
            TowerKind::Straight => "straight",
            TowerKind::Flush => "flush",
            TowerKind::FullHouse => "full_house",
            TowerKind::FourOfAKind => "four_of_a_kind",
            TowerKind::StraightFlush => "straight_flush",
            TowerKind::RoyalFlush => "royal_flush",
        }
    }
    pub fn shoot_interval(&self) -> Duration {
        match self {
            Self::Barricade => 1.sec(),
            Self::High => 1.sec(),
            Self::OnePair => 1.sec(),
            Self::TwoPair => 1.sec(),
            Self::ThreeOfAKind => 1.sec(),
            Self::Straight => 1.sec(),
            Self::Flush => 0.5.sec(),
            Self::FullHouse => 1.sec(),
            Self::FourOfAKind => 1.sec(),
            Self::StraightFlush => 0.5.sec(),
            Self::RoyalFlush => 0.33.sec(),
        }
    }
    pub fn default_attack_range_radius(&self) -> f32 {
        match self {
            Self::Barricade => 5.0,
            Self::High => 8.0,
            Self::OnePair => 8.0,
            Self::TwoPair => 10.0,
            Self::ThreeOfAKind => 10.0,
            Self::Straight => 12.0,
            Self::Flush => 13.0,
            Self::FullHouse => 15.0,
            Self::FourOfAKind => 15.0,
            Self::StraightFlush => 18.0,
            Self::RoyalFlush => 20.0,
        }
    }
    pub fn default_damage(&self) -> usize {
        match self {
            Self::Barricade => 0,
            Self::High => 5,
            Self::OnePair => 25,
            Self::TwoPair => 50,
            Self::ThreeOfAKind => 125,
            Self::Straight => 250,
            Self::Flush => 375,
            Self::FullHouse => 1000,
            Self::FourOfAKind => 1250,
            Self::StraightFlush => 7500,
            Self::RoyalFlush => 15000,
        }
    }
    pub fn skill_templates(&self) -> Vec<TowerSkillTemplate> {
        match self {
            Self::Barricade => vec![],
            Self::High => vec![],
            Self::OnePair => vec![TowerSkillTemplate::new_passive(
                TowerSkillKind::MoneyIncomeAdd { add: 1 },
            )],
            Self::TwoPair => vec![TowerSkillTemplate::new_passive(
                TowerSkillKind::MoneyIncomeAdd { add: 2 },
            )],
            Self::ThreeOfAKind => vec![TowerSkillTemplate::new_passive(
                TowerSkillKind::NearbyMonsterSpeedMul {
                    mul: 0.9,
                    range_radius: 5.0,
                },
            )],
            Self::Straight => vec![],
            Self::Flush => vec![],
            Self::FullHouse => vec![TowerSkillTemplate::new_passive(
                TowerSkillKind::NearbyTowerAttackSpeedMul {
                    mul: 2.0,
                    range_radius: 2.0,
                },
            )],
            Self::FourOfAKind => vec![TowerSkillTemplate::new_passive(
                TowerSkillKind::NearbyMonsterSpeedMul {
                    mul: 0.75,
                    range_radius: 4.0,
                },
            )],
            Self::StraightFlush => vec![],
            Self::RoyalFlush => vec![TowerSkillTemplate::new_passive(
                TowerSkillKind::NearbyTowerDamageMul {
                    mul: 2.0,
                    range_radius: 6.0,
                },
            )],
        }
    }
    pub fn is_low_card_tower(&self) -> bool {
        matches!(self, Self::High | Self::OnePair | Self::ThreeOfAKind)
    }

    pub fn to_text(self) -> TowerKindText {
        match self {
            Self::Barricade => TowerKindText::Barricade,
            Self::High => TowerKindText::High,
            Self::OnePair => TowerKindText::OnePair,
            Self::TwoPair => TowerKindText::TwoPair,
            Self::ThreeOfAKind => TowerKindText::ThreeOfAKind,
            Self::Straight => TowerKindText::Straight,
            Self::Flush => TowerKindText::Flush,
            Self::FullHouse => TowerKindText::FullHouse,
            Self::FourOfAKind => TowerKindText::FourOfAKind,
            Self::StraightFlush => TowerKindText::StraightFlush,
            Self::RoyalFlush => TowerKindText::RoyalFlush,
        }
    }
}

pub fn tower_cooldown_tick(game_state: &mut GameState, dt: Duration) {
    game_state.towers.iter_mut().for_each(|tower| {
        if tower.cooldown == Duration::from_secs(0) {
            return;
        }

        let tower_upgrades = game_state.upgrade_state.tower_upgrades(tower);

        let mut time_multiple = 1.0;

        tower.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::AttackSpeedAdd { add } = status_effect.kind {
                time_multiple += add;
            }
        });
        tower_upgrades.iter().for_each(|tower_upgrade_state| {
            time_multiple += tower_upgrade_state.speed_plus;
        });
        if time_multiple == 0.0 {
            return;
        }

        tower.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::AttackSpeedMul { mul } = status_effect.kind {
                time_multiple *= mul;
            }
        });
        tower_upgrades.iter().for_each(|tower_upgrade_state| {
            time_multiple *= tower_upgrade_state.speed_multiplier;
        });

        let cooldown_sub = dt * time_multiple;

        if tower.cooldown < cooldown_sub {
            tower.cooldown = Duration::from_secs(0);
        } else {
            tower.cooldown -= cooldown_sub;
        }
    });
}
