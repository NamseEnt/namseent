pub mod render;
mod royal_straight_flush;
mod skill;

use super::*;
use crate::game_state::attack::{AttackType, ProjectileGroup};
use crate::l10n::tower::TowerKindText;
use crate::{AttackId, MonsterId, SimTick, TowerId, WorldCoord, WorldDistance, WorldVec};
use namui::*;
use render::Animation;
pub use render::{AnimationKind, tower_animation_tick};
use royal_straight_flush::RoyalStraightFlushVisual;
pub use royal_straight_flush::royal_straight_flush_hit_delay;
pub use royal_straight_flush::tick_royal_straight_flush_visuals;
pub use skill::*;
use std::ops::Deref;

const PROJECTILE_SPEED: Velocity = WorldSpeed::from_raw(12 * crate::world::WORLD_UNITS_PER_TILE);
const FAST_PROJECTILE_SPEED: Velocity =
    WorldSpeed::from_raw(16 * crate::world::WORLD_UNITS_PER_TILE);

#[derive(Clone, PartialEq, State)]
pub struct Tower {
    id: Option<TowerId>,
    pub left_top: MapCoord,
    cooldown: SimTickSpan,
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
    pub damage: Damage,
}

pub struct ShootProjectileParams {
    pub id: AttackId,
    pub target_indicator: ProjectileTargetIndicator,
    pub key: u64,
    pub speed: Velocity,
    pub trail: ProjectileTrail,
    pub projectile_group: ProjectileGroup,
    pub hit_effect: attack::ProjectileHitEffect,
    pub damage: Damage,
    pub sim_tick: SimTick,
    pub source_tower: Option<attack::TowerInfo>,
}

pub struct AttackTypeParams {
    pub target_xy: WorldCoord,
    pub sim_tick: SimTick,
}

impl Tower {
    pub fn new(template: &TowerTemplate, left_top: MapCoord, sim_tick: SimTick) -> Self {
        Self {
            id: None,
            left_top,
            cooldown: SimTickSpan::ZERO,
            template: template.clone(),
            status_effects: template.default_status_effects.clone(),
            skills: template
                .skill_templates
                .iter()
                .cloned()
                .map(|skill_template| TowerSkill::new(skill_template, sim_tick))
                .collect(),
            cached_upgrade: CachedTowerUpgradeDamage {
                revision: 0,
                bonuses: Vec::new(),
                damage: template.default_damage,
            },
            animation: Animation::new(sim_tick),
            royal_straight_flush_visual: None,
        }
    }

    pub(crate) fn assign_id(&mut self, id: TowerId) {
        assert!(self.id.is_none(), "tower ID must be assigned exactly once");
        self.id = Some(id);
    }

    pub fn in_cooltime(&self) -> bool {
        self.cooldown > SimTickSpan::ZERO
    }

    pub fn shoot_projectile(&mut self, params: ShootProjectileParams) -> attack::InFlightAttack {
        self.mark_fired(params.sim_tick);

        attack::InFlightAttack::new_spatial(
            params.id,
            attack::SpatialAttack::new_direct(
                self.head_world_xy(),
                params.target_indicator,
                params.key,
                params.projectile_group.kind_for(params.key),
                params.speed,
                params.trail,
                params.hit_effect,
            ),
            params.damage,
            params.source_tower,
        )
        .with_on_hit_splashes(self.engraving_modifier().on_hit_splashes)
    }

    pub fn shoot_laser(
        &mut self,
        id: AttackId,
        target_xy: WorldCoord,
        target_monster_id: MonsterId,
        damage: Damage,
        sim_tick: SimTick,
        source_tower: Option<attack::TowerInfo>,
    ) -> attack::InFlightAttack {
        self.mark_fired(sim_tick);

        let beam = attack::laser::LaserBeam::new(
            self.head_world_xy(),
            target_xy,
            sim_tick,
            target_monster_id,
        );
        attack::InFlightAttack::new_laser(id, beam, damage, source_tower)
            .with_on_hit_splashes(self.engraving_modifier().on_hit_splashes)
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
            self.calculate_projectile_damage(&self.cached_upgrade.bonuses, FixedRatio::ONE);
        self.cached_upgrade.revision = revision;
    }

    pub fn cached_upgrade_damage(&self) -> Damage {
        self.cached_upgrade.damage
    }

    pub fn attack_type(&self, params: AttackTypeParams) -> AttackType {
        match self.kind {
            TowerKind::RubberCone => AttackType::Projectile {
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
                let head_xy = self.head_world_xy();
                AttackType::FullHouseRain { tower_xy: head_xy }
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
    pub fn mark_fired(&mut self, sim_tick: SimTick) {
        self.cooldown = self.effective_shoot_interval();
        self.animation.transition(AnimationKind::Attack, sim_tick);
    }

    fn center_xy(&self) -> MapCoord {
        self.left_top + MapCoord::new(1, 1)
    }
    pub fn center_xy_f32(&self) -> MapCoordF32 {
        self.center_world_xy().as_map_coord_f32()
    }

    pub fn center_world_xy(&self) -> WorldCoord {
        let center = self.center_xy();
        WorldCoord::from_tile_center(center.x as i64, center.y as i64)
    }

    pub fn head_xy_tile(&self) -> MapCoordF32 {
        self.head_world_xy().as_map_coord_f32()
    }

    pub fn head_world_xy(&self) -> WorldCoord {
        self.center_world_xy() + WorldVec::new(0, -crate::world::WORLD_UNITS_PER_TILE / 2)
    }

    pub fn id(&self) -> TowerId {
        self.id.expect("placed tower must have an ID")
    }

    pub fn rank(&self) -> Option<Rank> {
        self.template.rank
    }

    pub fn suit(&self) -> Option<Suit> {
        self.template.suit
    }

    pub fn rerolled_count(&self) -> usize {
        self.template.rerolled_count
    }

    pub fn calculate_projectile_damage(
        &self,
        tower_upgrade_bonuses: &[crate::game_state::upgrade::TowerUpgradeDamageBonus],
        stage_damage_multiplier: FixedRatio,
    ) -> Damage {
        let mut damage = self.default_damage;

        self.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::DamageAdd { add } = status_effect.kind {
                damage = damage.saturating_add_delta(add);
            }
        });

        if damage.is_zero() {
            return Damage::ZERO;
        }

        let mut ratios = RatioProduct::one();
        self.status_effects.iter().for_each(|status_effect| {
            if let TowerStatusEffectKind::DamageMul { mul } = status_effect.kind {
                ratios = ratios.clone().with(mul);
            }
        });

        let bonus_sum = tower_upgrade_bonuses
            .iter()
            .map(|upgrade_bonus| upgrade_bonus.effective_bonus_pct_for_tower(self).raw())
            .fold(0_i64, i64::saturating_add)
            .saturating_add(self.card_polish_pct().raw());
        ratios = ratios.with(FixedRatio::from_raw(
            FixedRatio::ONE.raw().saturating_add(bonus_sum),
        ));
        ratios = ratios.with(stage_damage_multiplier);

        Damage::from_raw(ratios.apply_raw(damage.raw()))
    }

    pub(crate) fn attack_range_radius(&self) -> WorldDistance {
        if self.kind == TowerKind::RubberCone {
            return WorldDistance::ZERO;
        }
        self.template.attack_range_radius()
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
    pub shoot_interval: SimTickSpan,
    pub default_attack_range_radius: WorldDistance,
    pub default_damage: Damage,
    pub suit: Option<Suit>,
    pub rank: Option<Rank>,
    pub skill_templates: Vec<TowerSkillTemplate>,
    pub default_status_effects: Vec<TowerStatusEffect>,
    used_cards: Vec<Card>,
    engraving: crate::card::TowerEngravingModifier,
}
impl TowerTemplate {
    pub fn new(kind: TowerKind, suit: Suit, rank: Rank) -> Self {
        Self::new_optional(kind, Some(suit), Some(rank))
    }

    pub fn new_optional(kind: TowerKind, suit: Option<Suit>, rank: Option<Rank>) -> Self {
        Self::new_optional_with_used_cards(kind, suit, rank, Vec::new())
    }

    pub fn new_optional_with_used_cards(
        kind: TowerKind,
        suit: Option<Suit>,
        rank: Option<Rank>,
        used_cards: Vec<Card>,
    ) -> Self {
        let mut template = Self {
            kind,
            rerolled_count: 0,
            shoot_interval: kind.shoot_interval(),
            default_attack_range_radius: kind.default_attack_range_radius(),
            default_damage: kind.default_damage(),
            suit,
            rank,
            skill_templates: kind.skill_templates(),
            default_status_effects: vec![],
            used_cards: Vec::new(),
            engraving: crate::card::TowerEngravingModifier::NONE,
        };
        template.set_used_cards(used_cards);
        template
    }

    pub fn used_cards(&self) -> &[Card] {
        &self.used_cards
    }

    pub fn set_used_cards(&mut self, used_cards: Vec<Card>) {
        self.engraving = used_cards
            .iter()
            .filter_map(|card| card.engraving())
            .map(|engraving| engraving.tower_modifier())
            .fold(
                crate::card::TowerEngravingModifier::NONE,
                crate::card::TowerEngravingModifier::combine,
            );
        self.used_cards = used_cards;
    }

    pub fn rubber_cone() -> Self {
        Self::new_optional(TowerKind::RubberCone, None, None)
    }

    pub fn suit(&self) -> Option<Suit> {
        self.suit
    }

    pub fn rank(&self) -> Option<Rank> {
        self.rank
    }

    pub fn calculate_rating(&self, damage_multiplier: FixedRatio) -> Damage {
        self.default_damage.scaled_by(damage_multiplier)
    }

    pub fn card_polish_pct(&self) -> FixedRatio {
        self.used_cards
            .iter()
            .map(|card| card.polish_pct())
            .fold(FixedRatio::ZERO, |sum, value| {
                FixedRatio::from_raw(sum.raw().saturating_add(value.raw()))
            })
    }

    pub fn engraving_modifier(&self) -> crate::card::TowerEngravingModifier {
        self.engraving.clone()
    }

    pub(crate) fn attack_range_radius(&self) -> WorldDistance {
        self.engraving
            .apply_attack_range(self.default_attack_range_radius)
    }

    pub fn effective_shoot_interval(&self) -> SimTickSpan {
        self.engraving.apply_shoot_interval(self.shoot_interval)
    }

    pub fn attack_power_with_upgrade_bonuses(
        &self,
        tower_upgrade_bonuses: &[crate::game_state::upgrade::TowerUpgradeDamageBonus],
    ) -> Damage {
        let upgrade_bonus_sum = tower_upgrade_bonuses
            .iter()
            .map(|bonus| bonus.effective_bonus_pct_for_tower_template(self).raw())
            .fold(0_i64, i64::saturating_add)
            .saturating_add(self.card_polish_pct().raw());
        let damage_multiplier =
            FixedRatio::from_raw(FixedRatio::ONE.raw().saturating_add(upgrade_bonus_sum));
        self.calculate_rating(damage_multiplier)
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

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    State,
)]
pub enum TowerKind {
    RubberCone,
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
    pub fn shoot_interval(&self) -> SimTickSpan {
        match self {
            Self::RubberCone => SimTickSpan::from_millis_ceil(1_000),
            Self::High => SimTickSpan::from_millis_ceil(1_000),
            Self::OnePair => SimTickSpan::from_millis_ceil(1_000),
            Self::TwoPair => SimTickSpan::from_millis_ceil(1_000),
            Self::ThreeOfAKind => SimTickSpan::from_millis_ceil(1_000),
            Self::Straight => SimTickSpan::from_millis_ceil(500),
            Self::Flush => SimTickSpan::from_millis_ceil(1_000),
            Self::FullHouse => SimTickSpan::from_millis_ceil(1_000),
            Self::FourOfAKind => SimTickSpan::from_millis_ceil(1_000),
            Self::StraightFlush => SimTickSpan::from_millis_ceil(500),
            Self::RoyalFlush => SimTickSpan::from_millis_ceil(1_000),
        }
    }
    pub fn default_attack_range_radius(&self) -> WorldDistance {
        match self {
            Self::RubberCone => WorldDistance::from_tiles(4),
            Self::High => WorldDistance::from_tiles(4),
            Self::OnePair => WorldDistance::from_tiles(5),
            Self::TwoPair => WorldDistance::from_tiles(6),
            Self::ThreeOfAKind => WorldDistance::from_tiles(7),
            Self::Straight => WorldDistance::from_tiles(9),
            Self::Flush => WorldDistance::from_tiles(9),
            Self::FullHouse => WorldDistance::from_tiles(11),
            Self::FourOfAKind => WorldDistance::from_tiles(11),
            Self::StraightFlush => WorldDistance::from_tiles(14),
            Self::RoyalFlush => WorldDistance::from_tiles(15),
        }
    }
    pub fn default_damage(&self) -> Damage {
        match self {
            Self::RubberCone => Damage::from_integer(0),
            Self::High => Damage::from_integer(5),
            Self::OnePair => Damage::from_integer(6),
            Self::TwoPair => Damage::from_integer(10),
            Self::ThreeOfAKind => Damage::from_integer(12),
            Self::Straight => Damage::from_integer(14),
            Self::Flush => Damage::from_integer(32),
            Self::FullHouse => Damage::from_integer(50),
            Self::FourOfAKind => Damage::from_integer(100),
            Self::StraightFlush => Damage::from_integer(250),
            Self::RoyalFlush => Damage::from_integer(1200),
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
            Self::RubberCone => TowerKindText::RubberCone,
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

pub fn tower_cooldown_tick(game_state: &mut GameState) {
    game_state.towers.iter_mut().for_each(|tower| {
        if tower.cooldown == SimTickSpan::ZERO {
            return;
        }

        if tower.cooldown == SimTickSpan::ONE {
            tower.cooldown = SimTickSpan::ZERO;
        } else {
            tower.cooldown -= SimTickSpan::ONE;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn template_with_cards(used_cards: Vec<Card>) -> TowerTemplate {
        TowerTemplate::new_optional_with_used_cards(
            TowerKind::OnePair,
            Some(Suit::Hearts),
            Some(Rank::Three),
            used_cards,
        )
    }

    #[test]
    fn engraving_modifier_folds_every_used_card() {
        let mut engraved = Card::new(Rank::Ace, Suit::Spades);
        engraved.effects.engraving = Some(crate::card::Engraving::Magnet);
        let plain = Card::new(Rank::Two, Suit::Hearts);

        let template = template_with_cards(vec![engraved, plain]);

        assert_eq!(
            template.engraving_modifier(),
            crate::card::TowerEngravingModifier::NONE
        );
    }

    #[test]
    fn attack_range_radius_goes_through_the_engraving_modifier() {
        let template = template_with_cards(vec![Card::new(Rank::Two, Suit::Hearts)]);

        assert_eq!(
            template.attack_range_radius(),
            template.default_attack_range_radius
        );
    }

    fn card_with_engraving(engraving: crate::card::Engraving) -> Card {
        let mut card = Card::new(Rank::Ace, Suit::Spades);
        card.effects.engraving = Some(engraving);
        card
    }

    #[test]
    fn overcharge_shortens_the_cooldown_after_firing() {
        let sim_tick = SimTick::ZERO;
        let plain = TowerTemplate::new(TowerKind::OnePair, Suit::Hearts, Rank::Three);
        let overcharged = template_with_cards(vec![card_with_engraving(
            crate::card::Engraving::Overcharge,
        )]);

        let mut plain_tower = Tower::new(&plain, MapCoord::new(0, 0), sim_tick);
        let mut overcharged_tower = Tower::new(&overcharged, MapCoord::new(0, 0), sim_tick);
        plain_tower.mark_fired(sim_tick);
        overcharged_tower.mark_fired(sim_tick);

        let expected = plain
            .shoot_interval
            .scale_ratio_ceil(overcharged.engraving_modifier().shoot_interval_mul);
        assert_eq!(overcharged_tower.cooldown, expected);
        assert!(overcharged_tower.cooldown < plain_tower.cooldown);
    }

    #[test]
    fn set_used_cards_re_resolves_the_engraving_modifier() {
        let mut template = TowerTemplate::new(TowerKind::OnePair, Suit::Hearts, Rank::Three);
        assert_eq!(
            template.engraving_modifier(),
            crate::card::TowerEngravingModifier::NONE
        );

        template.set_used_cards(vec![card_with_engraving(
            crate::card::Engraving::Overcharge,
        )]);
        assert!(template.engraving_modifier().shoot_interval_mul < FixedRatio::ONE);

        template.set_used_cards(vec![Card::new(Rank::Two, Suit::Hearts)]);
        assert_eq!(
            template.engraving_modifier(),
            crate::card::TowerEngravingModifier::NONE
        );
    }

    #[test]
    fn effective_shoot_interval_reflects_overcharge() {
        let plain = TowerTemplate::new(TowerKind::OnePair, Suit::Hearts, Rank::Three);
        let overcharged = template_with_cards(vec![card_with_engraving(
            crate::card::Engraving::Overcharge,
        )]);

        assert_eq!(plain.effective_shoot_interval(), plain.shoot_interval);
        let expected = plain
            .shoot_interval
            .scale_ratio_ceil(overcharged.engraving_modifier().shoot_interval_mul);
        assert_eq!(overcharged.effective_shoot_interval(), expected);
    }

    #[test]
    fn stacked_overcharge_cards_multiply_the_attack_speed() {
        let one = template_with_cards(vec![card_with_engraving(
            crate::card::Engraving::Overcharge,
        )]);
        let two = template_with_cards(vec![
            card_with_engraving(crate::card::Engraving::Overcharge),
            card_with_engraving(crate::card::Engraving::Overcharge),
        ]);

        let one_mul = one.engraving_modifier().shoot_interval_mul;
        let two_mul = two.engraving_modifier().shoot_interval_mul;

        assert_eq!(
            two_mul.raw(),
            crate::RatioProduct::one()
                .with(one_mul)
                .with(one_mul)
                .apply_raw(crate::combat_number::RATIO_SCALE)
        );
    }

    #[test]
    fn tower_new_applies_template_skills() {
        let now = SimTick::ZERO;
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
        let now = SimTick::ZERO;
        let mut tower = Tower::new(
            &TowerTemplate::new(TowerKind::RubberCone, Suit::Spades, Rank::Two),
            MapCoord::new(0, 0),
            now,
        );

        tower.cached_upgrade.revision = 1;
        tower.cached_upgrade.bonuses = vec![crate::game_state::upgrade::TowerUpgradeDamageBonus {
            target: crate::game_state::upgrade::TowerUpgradeTarget::Global,
            bonus_pct: FixedRatio::ZERO,
        }];
        tower.cached_upgrade.damage =
            tower.calculate_projectile_damage(&tower.cached_upgrade.bonuses, FixedRatio::ONE);

        let new_upgrade_bonuses = vec![crate::game_state::upgrade::TowerUpgradeDamageBonus {
            target: crate::game_state::upgrade::TowerUpgradeTarget::Suit { suit: Suit::Hearts },
            bonus_pct: FixedRatio::ONE,
        }];

        tower.refresh_cached_upgrade_damage(1, &new_upgrade_bonuses);

        assert_eq!(tower.cached_upgrade.revision, 1);
        assert_eq!(tower.cached_upgrade.bonuses.len(), 1);
        assert_eq!(
            tower.cached_upgrade.bonuses,
            vec![crate::game_state::upgrade::TowerUpgradeDamageBonus {
                target: crate::game_state::upgrade::TowerUpgradeTarget::Global,
                bonus_pct: FixedRatio::ZERO,
            }]
        );
    }
}
