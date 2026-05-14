use super::*;
use crate::game_state::attack::{HitSound, InFlightAttack, SpatialAttack, TowerInfo};
use crate::game_state::tower::{
    AttackTypeParams, ShootProjectileParams, royal_straight_flush_hit_delay,
};

pub fn shoot_attacks(game_state: &mut GameState) {
    use crate::game_state::attack::AttackType;

    let now = game_state.now();
    let mut new_attacks: Vec<InFlightAttack> = Vec::new();

    {
        let towers = &mut game_state.towers;
        let stage_modifiers = &game_state.stage_modifiers;
        let monsters = &game_state.monsters;
        let black_smoke_sources = &mut game_state.black_smoke_sources;

        for tower in towers.iter_mut() {
            if tower.in_cooltime() {
                continue;
            }
            if stage_modifiers.get_disabled_ranks().contains(&tower.rank()) {
                continue;
            }
            if stage_modifiers.get_disabled_suits().contains(&tower.suit()) {
                continue;
            }

            let attack_range_radius = tower.attack_range_radius(1.0);
            let tower_center = tower.center_xy_f32();
            let target_idx = monsters.iter().position(|monster| {
                (monster.center_xy_tile() - tower_center).length() < attack_range_radius
            });
            let Some(target_idx) = target_idx else {
                continue;
            };

            let target_xy = monsters[target_idx].center_xy_tile();
            let damage = tower.cached_upgrade_damage();
            let source_tower = TowerInfo {
                id: tower.id(),
                kind: tower.kind,
                rank: tower.rank(),
                suit: tower.suit(),
            };
            let attack_type = tower.attack_type(AttackTypeParams {
                target_xy: (target_xy.x, target_xy.y),
                now,
            });

            match attack_type {
                AttackType::Projectile {
                    speed,
                    trail,
                    projectile_group,
                    hit_effect,
                } => {
                    let target_indicator = monsters[target_idx].projectile_target_indicator;
                    new_attacks.push(tower.shoot_projectile(ShootProjectileParams {
                        target_indicator,
                        speed,
                        trail,
                        projectile_group,
                        hit_effect,
                        damage,
                        now,
                        source_tower: Some(source_tower),
                    }));
                }
                AttackType::Laser => {
                    let target_monster_id = monsters[target_idx].id();
                    new_attacks.push(tower.shoot_laser(
                        (target_xy.x, target_xy.y),
                        target_monster_id,
                        damage,
                        now,
                        Some(source_tower),
                    ));
                }
                AttackType::FullHouseRain { tower_xy } => {
                    let target_indicator = monsters[target_idx].projectile_target_indicator;
                    let damage_per_projectile = damage / 4.0;
                    tower.mark_fired(now);
                    for _ in 0..4 {
                        new_attacks.push(InFlightAttack::new_spatial(
                            SpatialAttack::new_homing(
                                crate::MapCoordF32::new(tower_xy.0, tower_xy.1),
                                target_indicator,
                                crate::game_state::projectile::ProjectileKind::random_trash(),
                                crate::game_state::projectile::ProjectileTrail::Burning,
                                crate::game_state::attack::ProjectileHitEffect::TrashBounce,
                            ),
                            damage_per_projectile,
                            Some(source_tower),
                        ));
                    }
                }
                AttackType::RoyalStraightFlush { target_xy } => {
                    let target_monster_id = monsters[target_idx].id();
                    tower.mark_fired(now);
                    tower.spawn_royal_straight_flush_visual(
                        &mut game_state.effect_events,
                        target_xy,
                        target_monster_id,
                        now,
                        black_smoke_sources,
                    );
                    new_attacks.push(InFlightAttack::new_timed(
                        target_monster_id,
                        now + royal_straight_flush_hit_delay(),
                        damage,
                        Some(source_tower),
                        HitSound::KnifeSlash,
                    ));
                }
            }
        }
    }

    game_state.in_flight_attacks.extend(new_attacks);
}
