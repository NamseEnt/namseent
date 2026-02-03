use super::*;

pub fn move_projectiles(game_state: &mut GameState, dt: Duration, now: Instant) {
    let GameState {
        projectiles,
        monsters,
        ..
    } = game_state;

    let mut total_earn_gold = 0;
    let mut damage_emitters = Vec::new();
    let mut monster_death_emitters = Vec::new();

    projectiles.retain_mut(|projectile| {
        let start_xy = projectile.xy;

        let Some(monster_index) = monsters
            .iter()
            .position(|monster| monster.projectile_target_indicator == projectile.target_indicator)
        else {
            return false;
        };

        let monster = &mut monsters[monster_index];
        let monster_xy = monster.move_on_route.xy();

        if (monster_xy - start_xy).length() > projectile.velocity * dt {
            projectile.move_by(dt, monster_xy);
            return true;
        }

        let damage = projectile.damage;
        monster.get_damage(damage);
        if damage > 0.0 {
            damage_emitters.push(field_particle::emitter::DamageTextEmitter::new(
                monster_xy, damage,
            ));
        }

        if monster.dead() {
            if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
                defense_flow.stage_progress.processed_hp += monster.max_hp;
            }
            let earn = monster.reward + game_state.upgrade_state.gold_earn_plus;
            let earn =
                (earn as f32 * game_state.stage_modifiers.get_gold_gain_multiplier()) as usize;
            total_earn_gold += earn;

            // Emit monster death particle (soul)
            monster_death_emitters.push(field_particle::emitter::MonsterDeathEmitter::new(
                monster_xy,
            ));

            // Emit monster corpse particle
            let monster_kind = monster.kind;
            let rotation = monster.animation.rotation;
            let wh = monster::monster_wh(monster_kind);

            // Calculate the pixel position where the monster is rendered
            let tile_base_xy = TILE_PX_SIZE.to_xy() * monster_xy;
            let monster_center_offset = Xy::new(
                TILE_PX_SIZE.width * 0.5,
                TILE_PX_SIZE.height - wh.height * 0.5
                    + TILE_PX_SIZE.height * monster.animation.y_offset,
            );
            let pixel_xy = tile_base_xy + monster_center_offset;

            let corpse_particle = field_particle::MonsterCorpseParticle::new(
                pixel_xy,
                now,
                rotation,
                monster_kind,
                wh,
            );
            game_state.field_particle_system_manager.add_emitters(vec![
                field_particle::FieldParticleEmitter::MonsterCorpse {
                    emitter: field_particle::TempParticleEmitter::new(vec![
                        field_particle::FieldParticle::MonsterCorpse {
                            particle: corpse_particle,
                        },
                    ]),
                },
            ]);

            monsters.swap_remove(monster_index);
        }

        false
    });

    super::particle_emit::emit_damage_text_particles(game_state, damage_emitters);
    super::particle_emit::emit_monster_death_particles(game_state, monster_death_emitters);

    if total_earn_gold > 0 {
        game_state.earn_gold(total_earn_gold);
    }
}
