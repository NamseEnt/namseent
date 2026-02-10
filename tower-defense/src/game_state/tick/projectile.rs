use super::*;

pub fn move_projectiles(game_state: &mut GameState, dt: Duration, now: Instant) {
    let GameState {
        projectiles,
        monsters,
        ..
    } = game_state;

    let mut total_earn_gold = 0;

    projectiles.retain_mut(|projectile| {
        let start_xy = projectile.xy;

        let Some(monster_index) = monsters
            .iter()
            .position(|monster| monster.projectile_target_indicator == projectile.target_indicator)
        else {
            field_particle::PROJECTILES.spawn(
                field_particle::ProjectileParticle::new(
                    projectile.xy,
                    projectile.kind,
                    projectile.rotation,
                    projectile.rotation_speed,
                    projectile.velocity,
                    now,
                    Duration::from_millis(300),
                ),
            );
            return false;
        };

        let monster = &mut monsters[monster_index];
        let monster_xy = monster.center_xy_tile();

        let step_distance = match projectile.behavior {
            ProjectileBehavior::Direct => projectile.velocity.length() * dt.as_secs_f32(),
            ProjectileBehavior::Homing { velocity, .. } => velocity.length() * dt.as_secs_f32(),
        };

        if (monster_xy - start_xy).length() > step_distance {
            match projectile.behavior {
                ProjectileBehavior::Direct => projectile.move_by(dt, monster_xy),
                ProjectileBehavior::Homing { .. } => projectile.move_homing(dt, monster_xy),
            }

            if projectile.trail == ProjectileTrail::Burning {
                field_particle::emitter::spawn_burning_trail(
                    start_xy,
                    projectile.xy,
                    dt,
                    now,
                );
            }

            return true;
        }

        let damage = projectile.damage;
        monster.get_damage(damage);
        if damage > 0.0 {
            field_particle::DAMAGE_TEXTS.spawn(
                field_particle::DamageTextParticle::new(monster_xy, damage, now),
            );
        }

        let bounce_particles = field_particle::emitter::create_bounce_particles(
            projectile.kind,
            (start_xy.x, start_xy.y),
            (monster_xy.x, monster_xy.y),
            now,
        );
        for p in bounce_particles {
            field_particle::TRASHES.spawn(p);
        }

        if monster.dead() {
            if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
                defense_flow.stage_progress.processed_hp += monster.max_hp;
            }
            let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
            let earn =
                (earn as f32 * game_state.stage_modifiers.get_gold_gain_multiplier()) as usize;
            total_earn_gold += earn;

            let monster_kind = monster.kind;
            let rotation = monster.animation.rotation;
            let wh = monster::monster_wh(monster_kind);

            let tile_base_xy = TILE_PX_SIZE.to_xy() * monster_xy;
            let monster_center_offset = Xy::new(
                TILE_PX_SIZE.width * 0.5,
                TILE_PX_SIZE.height - wh.height * 0.5
                    + TILE_PX_SIZE.height * monster.animation.y_offset,
            );
            let pixel_xy = tile_base_xy + monster_center_offset;

            field_particle::MONSTER_SOULS.spawn(
                field_particle::MonsterSoulParticle::new(pixel_xy, now, rotation),
            );

            field_particle::MONSTER_CORPSES.spawn(
                field_particle::MonsterCorpseParticle::new(
                    pixel_xy, now, rotation, monster_kind, wh,
                ),
            );

            monsters.swap_remove(monster_index);
        }

        false
    });

    if total_earn_gold > 0 {
        game_state.earn_gold(total_earn_gold);
    }
}
