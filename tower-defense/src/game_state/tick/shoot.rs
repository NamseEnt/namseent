use super::*;
use crate::game_state::attack::{InFlightAttack, SpatialAttack, TowerInfo};
use crate::game_state::entity_id::EntityIdAllocator;
use crate::game_state::tower::{
    AttackTypeParams, ShootProjectileParams, royal_straight_flush_hit_delay,
};
use crate::{WorldCoord, WorldDistance};
use std::cmp::Reverse;

fn allocate_attack_id(allocator: &mut EntityIdAllocator) -> crate::AttackId {
    allocator.allocate_attack_id()
}

fn target_priority(
    route_progress: WorldDistance,
    distance_squared: u128,
    entity_id: crate::MonsterId,
) -> (WorldDistance, Reverse<u128>, Reverse<crate::MonsterId>) {
    (
        route_progress,
        Reverse(distance_squared),
        Reverse(entity_id),
    )
}

fn select_target_index(
    monsters: &[crate::game_state::monster::Monster],
    tower_center: WorldCoord,
    range: WorldDistance,
) -> Option<usize> {
    let range_squared = (range.raw() as u128).saturating_mul(range.raw() as u128);
    monsters
        .iter()
        .enumerate()
        .filter(|(_, monster)| {
            (monster.center_world_xy() - tower_center).length_squared() <= range_squared
        })
        .max_by_key(|(_, monster)| {
            target_priority(
                monster.move_on_route.route_progress(),
                (monster.center_world_xy() - tower_center).length_squared(),
                monster.id(),
            )
        })
        .map(|(index, _)| index)
}

pub fn shoot_attacks(game_state: &mut GameState, presentation_instant: crate::PresentationInstant) {
    use crate::game_state::attack::AttackType;

    let sim_tick = game_state.sim_tick();
    let mut new_attacks: Vec<InFlightAttack> = Vec::new();
    let mut area_damage_events = Vec::new();
    let mut next_entity_id = game_state.next_entity_id;

    {
        let towers = &mut game_state.towers;
        let stage_modifiers = &game_state.stage_modifiers;
        let monsters = &game_state.monsters;
        let black_smoke_sources = &mut game_state.black_smoke_sources;

        for tower in towers.iter_mut() {
            if tower.in_cooltime() {
                continue;
            }
            if tower
                .rank()
                .is_some_and(|rank| stage_modifiers.get_disabled_ranks().contains(&rank))
            {
                continue;
            }
            if tower
                .suit()
                .is_some_and(|suit| stage_modifiers.get_disabled_suits().contains(&suit))
            {
                continue;
            }

            let attack_range_radius = tower.attack_range_radius();
            let tower_center_world = tower.center_world_xy();
            let target_idx = select_target_index(monsters, tower_center_world, attack_range_radius);
            let Some(target_idx) = target_idx else {
                continue;
            };

            let target_xy = monsters[target_idx].center_world_xy();
            let damage = tower.cached_upgrade_damage();
            let engraving_modifier = tower.engraving_modifier();
            let source_tower = TowerInfo {
                id: tower.id(),
                kind: tower.kind,
                rank: tower.rank(),
                suit: tower.suit(),
            };
            if !engraving_modifier.on_attack_splashes.is_empty() {
                area_damage_events.push(super::AreaDamageEvent {
                    center: tower_center_world,
                    damage,
                    source_tower,
                    splashes: engraving_modifier.on_attack_splashes.clone(),
                });
            }
            let attack_type = tower.attack_type(AttackTypeParams {
                target_xy,
                sim_tick,
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
                        id: allocate_attack_id(&mut next_entity_id),
                        target_indicator,
                        key: ((tower.id().raw()) << 32)
                            ^ sim_tick.ticks()
                            ^ monsters[target_idx].id().raw(),
                        speed,
                        trail,
                        projectile_group,
                        hit_effect,
                        damage,
                        sim_tick,
                        source_tower: Some(source_tower),
                    }));
                }
                AttackType::Laser => {
                    let target_monster_id = monsters[target_idx].id();
                    new_attacks.push(tower.shoot_laser(
                        allocate_attack_id(&mut next_entity_id),
                        target_xy,
                        target_monster_id,
                        damage,
                        sim_tick,
                        Some(source_tower),
                    ));
                }
                AttackType::FullHouseRain { tower_xy } => {
                    let target_indicator = monsters[target_idx].projectile_target_indicator;
                    tower.mark_fired(sim_tick);
                    for (projectile_index, damage_per_projectile) in
                        damage.split_evenly(4).into_iter().enumerate()
                    {
                        let projectile_key = ((tower.id().raw()) << 32)
                            ^ sim_tick.ticks()
                            ^ monsters[target_idx].id().raw()
                            ^ projectile_index as u64;
                        new_attacks.push(
                            InFlightAttack::new_spatial(
                                allocate_attack_id(&mut next_entity_id),
                                SpatialAttack::new_homing(
                                    tower_xy,
                                    target_indicator,
                                    projectile_key,
                                    crate::game_state::projectile::ProjectileKind::deterministic_trash(projectile_key),
                                    crate::game_state::projectile::ProjectileTrail::Burning,
                                    crate::game_state::attack::ProjectileHitEffect::TrashBounce,
                                ),
                                damage_per_projectile,
                                Some(source_tower),
                            )
                            .with_on_hit_splashes(engraving_modifier.on_hit_splashes.clone()),
                        );
                    }
                }
                AttackType::RoyalStraightFlush { target_xy } => {
                    let target_monster_id = monsters[target_idx].id();
                    tower.mark_fired(sim_tick);
                    tower.spawn_royal_straight_flush_visual(
                        &mut game_state.effect_events,
                        {
                            let xy = target_xy.as_map_coord_f32();
                            (xy.x, xy.y)
                        },
                        target_monster_id,
                        sim_tick,
                        presentation_instant,
                        black_smoke_sources,
                    );
                    new_attacks.push(
                        InFlightAttack::new_timed(
                            allocate_attack_id(&mut next_entity_id),
                            target_monster_id,
                            sim_tick + royal_straight_flush_hit_delay(),
                            damage,
                            Some(source_tower),
                        )
                        .with_on_hit_splashes(engraving_modifier.on_hit_splashes.clone()),
                    );
                }
            }
        }
    }

    game_state.next_entity_id = next_entity_id;
    area_damage_events.sort_by_key(|event| event.source_tower.id);
    new_attacks.sort_by_key(|attack| attack.id);
    game_state.in_flight_attacks.extend(new_attacks);
    game_state.in_flight_attacks.sort_by_key(|attack| attack.id);
    super::resolve::apply_area_damage_events(game_state, area_damage_events, presentation_instant);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn monster_with_id(id: crate::MonsterId) -> crate::game_state::monster::Monster {
        let game_state = crate::game_state::create_game_state_with_seed(7);
        let template = crate::game_state::monster::MonsterTemplate::new(
            crate::game_state::monster::MonsterKind::Mob01,
            &game_state.config,
        );
        crate::game_state::monster::Monster::new_with_id(
            &template,
            game_state.route,
            crate::SimTick::ZERO,
            &crate::RatioProduct::one(),
            id,
        )
    }

    #[test]
    fn tie_breaking_is_independent_of_insertion_order() {
        let first_order = vec![
            monster_with_id(crate::MonsterId::from_raw(7)),
            monster_with_id(crate::MonsterId::from_raw(2)),
        ];
        let reversed_order = vec![
            monster_with_id(crate::MonsterId::from_raw(2)),
            monster_with_id(crate::MonsterId::from_raw(7)),
        ];
        let center = first_order[0].center_world_xy();
        let first = select_target_index(&first_order, center, WorldDistance::ZERO)
            .map(|index| first_order[index].id());
        let reversed = select_target_index(&reversed_order, center, WorldDistance::ZERO)
            .map(|index| reversed_order[index].id());
        assert_eq!(first, Some(crate::MonsterId::from_raw(2)));
        assert_eq!(reversed, Some(crate::MonsterId::from_raw(2)));
    }

    #[test]
    fn range_boundary_is_inclusive() {
        let monsters = vec![monster_with_id(crate::MonsterId::from_raw(1))];
        let target = monsters[0].center_world_xy();
        let range = WorldDistance::from_tiles(2);
        let on_boundary = target - crate::WorldVec::new(range.raw(), 0);
        let outside = target - crate::WorldVec::new(range.raw().saturating_add(1), 0);

        assert_eq!(select_target_index(&monsters, on_boundary, range), Some(0));
        assert_eq!(select_target_index(&monsters, outside, range), None);
    }
}
