pub mod render;
mod royal_straight_flush;
mod skill;

use super::*;
use crate::card::{Rank, Suit};
use crate::game_state::attack::{AttackType, ProjectileGroup};
use crate::l10n::tower::TowerKindText;
use namui::*;
use render::Animation;
pub use render::{AnimationKind, tower_animation_tick};
use royal_straight_flush::RoyalStraightFlushVisual;
pub use royal_straight_flush::royal_straight_flush_hit_delay;
pub use royal_straight_flush::tick_royal_straight_flush_visuals;
pub use skill::*;
use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

const PROJECTILE_SPEED: Velocity = Per::new(12.0, Duration::from_secs(1));
const FAST_PROJECTILE_SPEED: Velocity = Per::new(16.0, Duration::from_secs(1));

#[derive(Clone, PartialEq, State)]
pub struct Tower {
    id: usize,
    pub left_top: MapCoord,
    cooldown: Duration,
    pub template: TowerTemplate,
    pub status_effects: Vec<TowerStatusEffect>,
    pub skills: Vec<TowerSkill>,
    cached_upgrade: CachedTowerUpgradeDamage,
    pub(in crate::game_state::tower) animation: Animation,
    pub(self) royal_straight_flush_visual: Option<RoyalStraightFlushVisual>,
}

#[derive(Clone, Debug, PartialEq, State)]
pub struct CachedTowerUpgradeDamage {
    pub revision: usize,
    pub bonuses: Vec<crate::game_state::upgrade::TowerUpgradeDamageBonus>,
    pub damage: f32,
}

pub struct ShootProjectileParams {
    pub target_indicator: ProjectileTargetIndicator,
    pub speed: Velocity,
    pub trail: ProjectileTrail,
    pub projectile_group: ProjectileGroup,
    pub hit_effect: attack::ProjectileHitEffect,
    pub damage: f32,
    pub now: Instant,
    pub source_tower: Option<attack::TowerInfo>,
}

pub struct AttackTypeParams {
    pub target_xy: (f32, f32),
    pub now: Instant,
}

impl Tower {
    pub fn new(template: &TowerTemplate, left_top: MapCoord, now: Instant) -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            left_top,
            cooldown: Duration::from_secs(0),
            template: template.clone(),
            status_effects: template.default_status_effects.clone(),
            skills: template
                .skill_templates
                .iter()
                .cloned()
                .map(|skill_template| TowerSkill::new(skill_template, now))
                .collect(),
            cached_upgrade: CachedTowerUpgradeDamage {
                revision: 0,
                bonuses: Vec::new(),
                damage: template.default_damage,
            },
            animation: Animation::new(now),
            royal_straight_flush_visual: None,
        }
    }
    pub fn in_cooltime(&self) -> bool {
        self.cooldown > Duration::from_secs(0)
    }

    pub fn shoot_projectile(&mut self, params: ShootProjectileParams) -> attack::InFlightAttack {
        self.mark_fired(params.now);

        attack::InFlightAttack::new_spatial(
            attack::SpatialAttack::new_direct(
                self.head_xy_tile(),
                params.target_indicator,
                params.projectile_group.random_kind(),
                params.speed,
                params.trail,
                params.hit_effect,
            ),
            params.damage,
            params.source_tower,
        )
    }

    pub fn shoot_laser(
        &mut self,
        target_xy: (f32, f32),
        target_monster_id: usize,
        damage: f32,
        now: Instant,
        source_tower: Option<attack::TowerInfo>,
    ) -> attack::InFlightAttack {
        self.mark_fired(now);

        let head_xy = self.head_xy_tile();
        let beam = attack::laser::LaserBeam::new(
            (head_xy.x, head_xy.y),
            target_xy,
            now,
            target_monster_id,
        );
        attack::InFlightAttack::new_laser(beam, damage, source_tower)
    }

    pub fn refresh_cached_upgrade_damage(
        &mut self,
        revision: usize,
        upgrade_bonuses: &[crate::game_state::upgrade::TowerUpgradeDamageBonus],
    ) {
        if self.cached_upgrade.revision != revision {
            self.cached_upgrade.bonuses = upgrade_bonuses.to_vec();
        }
        self.cached_upgrade.damage =
            self.calculate_projectile_damage(&self.cached_upgrade.bonuses, 1.0);
        self.cached_upgrade.revision = revision;
    }

    pub fn cached_upgrade_damage(&self) -> f32 {
        self.cached_upgrade.damage
    }

    pub fn attack_type(&self, params: AttackTypeParams) -> AttackType {
        match self.kind {
            TowerKind::Barricade => AttackType::Projectile {
                speed: PROJECTILE_SPEED,
                trail: ProjectileTrail::None,
                projectile_group: ProjectileGroup::Trash,
                hit_effect: attack::ProjectileHitEffect::TrashBounce,
            },
            TowerKind::High => AttackType::Projectile {
                speed: PROJECTILE_SPEED,
                trail: ProjectileTrail::None,
                projectile_group: ProjectileGroup::Trash,
                hit_effect: attack::ProjectileHitEffect::TrashBounce,
            },
            TowerKind::OnePair => AttackType::Projectile {
                speed: PROJECTILE_SPEED,
                trail: ProjectileTrail::None,
                projectile_group: ProjectileGroup::Trash,
                hit_effect: attack::ProjectileHitEffect::TrashBounce,
            },
            TowerKind::TwoPair => AttackType::Projectile {
                speed: PROJECTILE_SPEED,
                trail: ProjectileTrail::None,
                projectile_group: ProjectileGroup::Trash,
                hit_effect: attack::ProjectileHitEffect::TrashBounce,
            },
            TowerKind::ThreeOfAKind => AttackType::Projectile {
                speed: FAST_PROJECTILE_SPEED,
                trail: ProjectileTrail::Burning,
                projectile_group: ProjectileGroup::Trash,
                hit_effect: attack::ProjectileHitEffect::TrashBounce,
            },
            TowerKind::Straight => AttackType::Laser,
            TowerKind::RoyalFlush => AttackType::RoyalStraightFlush {
                target_xy: params.target_xy,
            },
            TowerKind::StraightFlush => AttackType::Projectile {
                speed: FAST_PROJECTILE_SPEED,
                trail: ProjectileTrail::LightningSparkle,
                projectile_group: ProjectileGroup::Heart,
                hit_effect: attack::ProjectileHitEffect::HeartBurst,
            },
            TowerKind::Flush => AttackType::Projectile {
                speed: FAST_PROJECTILE_SPEED,
                trail: ProjectileTrail::Sparkle,
                projectile_group: ProjectileGroup::Girl,
                hit_effect: attack::ProjectileHitEffect::SparkleBurst,
            },
            TowerKind::FullHouse => {
                let head_xy = self.head_xy_tile();
                AttackType::FullHouseRain {
                    tower_xy: (head_xy.x, head_xy.y),
                }
            }
            TowerKind::FourOfAKind => AttackType::Projectile {
                speed: FAST_PROJECTILE_SPEED,
                trail: ProjectileTrail::WindCurve,
                projectile_group: ProjectileGroup::Cards,
                hit_effect: attack::ProjectileHitEffect::CardBurst,
            },
        }
    }

    /// cooldown과 animation을 한 번에 설정. shoot_projectile/shoot_laser와 달리
    /// FullHouse/RSF처럼 별도 shoot_* 메서드가 없는 공격 타입이 호출한다.
    pub fn mark_fired(&mut self, now: Instant) {
        self.cooldown = self.shoot_interval;
        self.animation.transition(AnimationKind::Attack, now);
    }

    fn center_xy(&self) -> MapCoord {
        self.left_top + MapCoord::new(1, 1)
    }
    pub fn center_xy_f32(&self) -> MapCoordF32 {
        self.center_xy().map(|t| t as f32)
    }

    pub fn head_xy_tile(&self) -> MapCoordF32 {
        let center = self.center_xy_f32();
        MapCoordF32::new(center.x, center.y - 0.5)
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

    pub fn rerolled_count(&self) -> usize {
        self.template.rerolled_count
    }

    pub fn calculate_projectile_damage(
        &self,
        tower_upgrade_bonuses: &[crate::game_state::upgrade::TowerUpgradeDamageBonus],
        stage_damage_multiplier: f32,
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

        let bonus_sum: f32 = tower_upgrade_bonuses
            .iter()
            .map(|upgrade_bonus| upgrade_bonus.effective_bonus_pct_for_tower(self))
            .sum();

        damage *= 1.0 + bonus_sum;

        damage *= stage_damage_multiplier;

        damage
    }

    pub(crate) fn attack_range_radius(&self, contract_range_multiplier: f32) -> f32 {
        if self.kind == TowerKind::Barricade {
            return 0.0;
        }
        self.default_attack_range_radius * contract_range_multiplier
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
    pub rerolled_count: usize,
    pub shoot_interval: Duration,
    pub default_attack_range_radius: f32,
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
            rerolled_count: 0,
            shoot_interval: kind.shoot_interval(),
            default_attack_range_radius: kind.default_attack_range_radius(),
            default_damage: kind.default_damage(),
            suit,
            rank,
            skill_templates: kind.skill_templates(),
            default_status_effects: vec![],
        }
    }

    pub fn new_with_config(
        kind: TowerKind,
        suit: Suit,
        rank: Rank,
        config: &crate::config::GameConfig,
    ) -> Self {
        let stats = config
            .towers
            .stats
            .get(&kind)
            .expect("missing tower stats for kind");
        Self {
            kind,
            rerolled_count: 0,
            shoot_interval: Duration::from_millis(stats.cooldown_ms as i64),
            default_attack_range_radius: stats.range,
            default_damage: stats.damage,
            suit,
            rank,
            skill_templates: kind.skill_templates(),
            default_status_effects: vec![],
        }
    }

    pub fn barricade() -> Self {
        Self::new(TowerKind::Barricade, Suit::Spades, Rank::Ace)
    }

    pub fn calculate_rating(&self, damage_multiplier: f32, rank_bonus: usize) -> f32 {
        (self.default_damage + rank_bonus as f32) * damage_multiplier
    }
}
impl PartialOrd for TowerTemplate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.kind
                .cmp(&other.kind)
                .then_with(|| self.suit.cmp(&other.suit))
                .then_with(|| self.rank.cmp(&other.rank)),
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, State)]
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
            Self::RoyalFlush => 1.sec(),
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
            Self::RoyalFlush => 1200.0,
        }
    }
    pub fn skill_templates(&self) -> Vec<TowerSkillTemplate> {
        vec![]
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

        if tower.cooldown < dt {
            tower.cooldown = Duration::from_secs(0);
        } else {
            tower.cooldown -= dt;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_new_applies_template_skills() {
        let now = Instant::now();
        let template = TowerTemplate::new(TowerKind::OnePair, Suit::Hearts, Rank::Three);
        let tower = Tower::new(&template, MapCoord::new(0, 0), now);

        assert_eq!(tower.skills.len(), template.skill_templates.len());
        assert!(tower.skills.iter().all(|skill| {
            template
                .skill_templates
                .iter()
                .any(|template_skill| template_skill == &skill.template)
        }));
    }

    #[test]
    fn refresh_cached_upgrade_damage_preserves_cached_bonuses_when_revision_unchanged() {
        let now = Instant::now();
        let mut tower = Tower::new(
            &TowerTemplate::new(TowerKind::Barricade, Suit::Spades, Rank::Two),
            MapCoord::new(0, 0),
            now,
        );

        tower.cached_upgrade.revision = 1;
        tower.cached_upgrade.bonuses = vec![crate::game_state::upgrade::TowerUpgradeDamageBonus {
            target: crate::game_state::upgrade::TowerUpgradeTarget::Global,
            bonus_pct: 0.0,
        }];
        tower.cached_upgrade.damage =
            tower.calculate_projectile_damage(&tower.cached_upgrade.bonuses, 1.0);

        let new_upgrade_bonuses = vec![crate::game_state::upgrade::TowerUpgradeDamageBonus {
            target: crate::game_state::upgrade::TowerUpgradeTarget::Suit { suit: Suit::Hearts },
            bonus_pct: 1.0,
        }];

        tower.refresh_cached_upgrade_damage(1, &new_upgrade_bonuses);

        assert_eq!(tower.cached_upgrade.revision, 1);
        assert_eq!(tower.cached_upgrade.bonuses.len(), 1);
        assert_eq!(
            tower.cached_upgrade.bonuses,
            vec![crate::game_state::upgrade::TowerUpgradeDamageBonus {
                target: crate::game_state::upgrade::TowerUpgradeTarget::Global,
                bonus_pct: 0.0,
            }]
        );
    }
}
