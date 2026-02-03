pub mod render;
mod skill;

use super::{upgrade::TowerUpgradeState, *};
use crate::card::{Rank, Suit};
use crate::game_state::attack::AttackType;
use crate::l10n::tower::TowerKindText;
use namui::*;
use render::Animation;
pub use render::{AnimationKind, tower_animation_tick};
pub use skill::*;
use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

const PROJECTILE_SPEED: Velocity = Per::new(16.0, Duration::from_secs(1));

#[derive(Clone, PartialEq, State)]
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

    pub fn shoot_projectile(
        &mut self,
        target_indicator: ProjectileTargetIndicator,
        tower_upgrade_states: &[TowerUpgradeState],
        contract_multiplier: f32,
        now: Instant,
    ) -> Projectile {
        self.cooldown = self.shoot_interval;
        self.animation.transition(AnimationKind::Attack, now);

        Projectile::new(
            self.left_top.map(|t| t as f32 + 0.5),
            ProjectileKind::random_trash(),
            PROJECTILE_SPEED,
            target_indicator,
            self.calculate_projectile_damage(tower_upgrade_states, contract_multiplier),
        )
    }

    /// 레이저 공격 수행 - 즉시 데미지를 주고 LaserBeam 반환
    pub fn shoot_laser(
        &mut self,
        target_xy: (f32, f32),
        tower_upgrade_states: &[TowerUpgradeState],
        contract_multiplier: f32,
        now: Instant,
    ) -> (attack::laser::LaserBeam, f32) {
        self.cooldown = self.shoot_interval;
        self.animation.transition(AnimationKind::Attack, now);

        let damage = self.calculate_projectile_damage(tower_upgrade_states, contract_multiplier);

        let laser = attack::laser::LaserBeam::new(
            (self.left_top.x as f32 + 0.5, self.left_top.y as f32 + 0.5),
            target_xy,
            now,
            damage,
        );

        (laser, damage)
    }

    /// 즉시 이펙트 공격 수행 - 즉시 데미지를 주고 이펙트들 반환
    pub fn shoot_instant_effect(
        &mut self,
        target_xy: (f32, f32),
        tower_upgrade_states: &[TowerUpgradeState],
        contract_multiplier: f32,
        now: Instant,
    ) -> (
        attack::instant_effect::TowerEmitEffect,
        attack::instant_effect::TargetHitEffect,
        f32,
    ) {
        self.cooldown = self.shoot_interval;
        self.animation.transition(AnimationKind::Attack, now);

        let damage = self.calculate_projectile_damage(tower_upgrade_states, contract_multiplier);

        let tower_xy = (self.left_top.x as f32 + 0.5, self.left_top.y as f32 + 0.5);
        let effect_kind = attack::instant_effect::InstantEffectKind::Explosion;

        let emit_effect =
            attack::instant_effect::TowerEmitEffect::new(tower_xy, target_xy, now, effect_kind);

        let hit_effect =
            attack::instant_effect::TargetHitEffect::new(target_xy, now, effect_kind, 1.0);

        (emit_effect, hit_effect, damage)
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
    pub fn rank(&self) -> Rank {
        self.template.rank
    }
    pub fn suit(&self) -> Suit {
        self.template.suit
    }

    pub fn calculate_projectile_damage(
        &self,
        tower_upgrade_states: &[TowerUpgradeState],
        contract_multiplier: f32,
    ) -> f32 {
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

        tower_upgrade_states.iter().for_each(|tower_upgrade_state| {
            damage *= tower_upgrade_state.damage_multiplier;
        });

        // Apply contract damage multiplier
        damage *= contract_multiplier;

        damage
    }

    pub(crate) fn attack_range_radius(
        &self,
        _tower_upgrade_states: &[TowerUpgradeState],
        contract_range_multiplier: f32,
    ) -> f32 {
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
        ) * contract_range_multiplier
    }
}
impl Deref for Tower {
    type Target = TowerTemplate;

    fn deref(&self) -> &Self::Target {
        &self.template
    }
}

#[derive(Debug, Clone, PartialEq, State)]
pub struct TowerTemplate {
    pub kind: TowerKind,
    pub shoot_interval: Duration,
    pub default_attack_range_radius: f32,
    pub default_damage: f32,
    pub suit: Suit,
    pub rank: Rank,
    pub skill_templates: Vec<TowerSkillTemplate>,
    pub default_status_effects: Vec<TowerStatusEffect>,
    pub attack_type: AttackType,
}
impl TowerTemplate {
    pub fn new(kind: TowerKind, suit: Suit, rank: Rank) -> Self {
        Self {
            kind,
            shoot_interval: kind.shoot_interval(),
            default_attack_range_radius: kind.default_attack_range_radius(),
            default_damage: kind.default_damage(),
            suit,
            rank,
            skill_templates: kind.skill_templates(),
            default_status_effects: vec![],
            attack_type: kind.attack_type(),
        }
    }
    pub fn barricade() -> Self {
        Self::new(TowerKind::Barricade, Suit::Spades, Rank::Ace)
    }

    /// Calculate tower power rating based on damage
    pub fn calculate_rating(&self, damage_multiplier: f32) -> f32 {
        (self.default_damage + self.rank.bonus_damage() as f32) * damage_multiplier
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
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
    pub fn shoot_interval(&self) -> Duration {
        match self {
            Self::Barricade => 1.sec(),
            Self::High => 1.sec(),
            Self::OnePair => 1.sec(),
            Self::TwoPair => 1.sec(),
            Self::ThreeOfAKind => 1.sec(),
            Self::Straight => 0.5.sec(),
            Self::Flush => 1.sec(),
            Self::FullHouse => 1.sec(),
            Self::FourOfAKind => 1.sec(),
            Self::StraightFlush => 0.5.sec(),
            Self::RoyalFlush => (1.0 / 3.0).sec(),
        }
    }
    pub fn default_attack_range_radius(&self) -> f32 {
        match self {
            Self::Barricade => 4.0,
            Self::High => 4.0,
            Self::OnePair => 5.0,
            Self::TwoPair => 6.0,
            Self::ThreeOfAKind => 7.0,
            Self::Straight => 9.0,
            Self::Flush => 9.0,
            Self::FullHouse => 11.0,
            Self::FourOfAKind => 11.0,
            Self::StraightFlush => 14.0,
            Self::RoyalFlush => 15.0,
        }
    }
    pub fn default_damage(&self) -> f32 {
        match self {
            Self::Barricade => 0.0,
            Self::High => 5.0,
            Self::OnePair => 6.0,
            Self::TwoPair => 10.0,
            Self::ThreeOfAKind => 12.0,
            Self::Straight => 14.0,
            Self::Flush => 32.0,
            Self::FullHouse => 50.0,
            Self::FourOfAKind => 100.0,
            Self::StraightFlush => 250.0,
            Self::RoyalFlush => 400.0,
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

    pub fn attack_type(&self) -> AttackType {
        match self {
            Self::Barricade => AttackType::Projectile,
            Self::High => AttackType::Projectile,
            Self::OnePair => AttackType::Projectile,
            Self::TwoPair => AttackType::Projectile,
            Self::ThreeOfAKind => AttackType::Projectile,
            Self::Straight => AttackType::Projectile,
            Self::Flush => AttackType::Laser,
            Self::FullHouse => AttackType::Projectile,
            Self::FourOfAKind => AttackType::InstantEffect,
            Self::StraightFlush => AttackType::Laser,
            Self::RoyalFlush => AttackType::Laser,
        }
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
    let attack_speed_multiplier = game_state.stage_modifiers.get_attack_speed_multiplier();

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

        // Apply contract attack speed multiplier
        time_multiple *= attack_speed_multiplier;

        let cooldown_sub = dt * time_multiple;

        if tower.cooldown < cooldown_sub {
            tower.cooldown = Duration::from_secs(0);
        } else {
            tower.cooldown -= cooldown_sub;
        }
    });
}
