mod render;
mod skill;

use super::*;
use crate::card::{Rank, Suit};
use namui::*;
use render::Animation;
pub use render::{AnimationKind, tower_animation_tick, tower_image_resource_location};
pub use skill::*;
use std::{
    fmt::Display,
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

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
    pub fn new(template: &TowerTemplate, left_top: MapCoord) -> Self {
        const ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            left_top,
            cooldown: Duration::from_secs(0),
            template: template.clone(),
            status_effects: vec![],
            skills: vec![],
            animation: Animation::new(),
        }
    }
    pub fn in_cooltime(&self) -> bool {
        self.cooldown > Duration::from_secs(0)
    }

    pub fn shoot(&mut self, target_indicator: ProjectileTargetIndicator) -> Projectile {
        self.cooldown = self.shoot_interval;
        self.animation.transition(AnimationKind::Attack);

        Projectile {
            kind: self.projectile_kind,
            xy: self.left_top.map(|t| t as f32 + 0.5),
            velocity: self.projectile_speed,
            target_indicator,
            damage: self.calculate_projectile_damage(),
        }
    }

    fn center_xy(&self) -> MapCoord {
        self.left_top + MapCoord::new(1, 1)
    }
    fn center_xy_f32(&self) -> MapCoordF32 {
        self.center_xy().map(|t| t as f32)
    }

    fn calculate_projectile_damage(&self) -> f32 {
        let mut damage = self.default_damage;

        self.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::DamageAdd { add } = status_effect.kind {
                damage += add;
            }
        });

        if damage < 0.0 {
            return 0.0;
        }

        self.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::DamageMul { mul } = status_effect.kind {
                damage *= mul;
            }
        });

        damage
    }

    pub(crate) fn attack_range_radius(&self) -> f32 {
        self.status_effects.iter().fold(
            self.default_attack_range_radius,
            |attack_range_radius, status_effect| {
                if let TowerStatusEffectKind::AttackRangeAdd { add } = status_effect.kind {
                    attack_range_radius + add
                } else {
                    attack_range_radius
                }
            },
        )
    }
}
impl Deref for Tower {
    type Target = TowerTemplate;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

#[derive(Clone)]
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            Self::High => 5.0,
            Self::OnePair => 5.0,
            Self::TwoPair => 5.0,
            Self::ThreeOfAKind => 5.0,
            Self::Straight => 10.0,
            Self::Flush => 5.0,
            Self::FullHouse => 5.0,
            Self::FourOfAKind => 5.0,
            Self::StraightFlush => 10.0,
            Self::RoyalFlush => 15.0,
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
}
impl Display for TowerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Barricade => "Barricade",
                Self::High => "High",
                Self::OnePair => "One Pair",
                Self::TwoPair => "Two Pair",
                Self::ThreeOfAKind => "Three of a Kind",
                Self::Straight => "Straight",
                Self::Flush => "Flush",
                Self::FullHouse => "Full House",
                Self::FourOfAKind => "Four of a Kind",
                Self::StraightFlush => "Straight Flush",
                Self::RoyalFlush => "Royal Flush",
            }
        )
    }
}

pub fn tower_cooldown_tick(game_state: &mut GameState, dt: Duration) {
    game_state.towers.iter_mut().for_each(|tower| {
        if tower.cooldown == Duration::from_secs(0) {
            return;
        }

        let mut time_multiple = 1.0;

        tower.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::AttackSpeedAdd { add } = status_effect.kind {
                time_multiple += add;
            }
        });
        if time_multiple == 0.0 {
            return;
        }

        tower.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::AttackSpeedMul { mul } = status_effect.kind {
                time_multiple *= mul;
            }
        });

        let cooldown_sub = dt * time_multiple;

        if tower.cooldown < cooldown_sub {
            tower.cooldown = Duration::from_secs(0);
        } else {
            tower.cooldown -= cooldown_sub;
        }
    });
}
