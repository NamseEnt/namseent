use super::*;

pub fn handle_monster_death(
    game_state: &mut GameState,
    target_idx: usize,
    target_xy: Xy<f32>,
    now: Instant,
    monster_death_emitters: &mut Vec<field_particle::emitter::MonsterDeathEmitter>,
) {
    // 몬스터 데이터 먼저 추출 (mutable borrow 충돌 회피)
    let monster_max_hp = game_state.monsters[target_idx].max_hp;
    let monster_reward = game_state.monsters[target_idx].reward;
    let monster_kind = game_state.monsters[target_idx].kind;
    let rotation = game_state.monsters[target_idx].animation.rotation;
    let y_offset = game_state.monsters[target_idx].animation.y_offset;

    if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
        defense_flow.stage_progress.processed_hp += monster_max_hp;
    }

    let earn = monster_reward + game_state.upgrade_state.gold_earn_plus;
    let earn = (earn as f32 * game_state.stage_modifiers.get_gold_gain_multiplier()) as usize;

    monster_death_emitters.push(field_particle::emitter::MonsterDeathEmitter::new(target_xy));

    let wh = monster::monster_wh(monster_kind);

    let tile_base_xy = TILE_PX_SIZE.to_xy() * target_xy;
    let monster_center_offset = Xy::new(
        TILE_PX_SIZE.width * 0.5,
        TILE_PX_SIZE.height - wh.height * 0.5 + TILE_PX_SIZE.height * y_offset,
    );
    let pixel_xy = tile_base_xy + monster_center_offset;

    let corpse_particle =
        field_particle::MonsterCorpseParticle::new(pixel_xy, now, rotation, monster_kind, wh);
    game_state.field_particle_system_manager.add_emitters(vec![
        field_particle::FieldParticleEmitter::MonsterCorpse {
            emitter: field_particle::TempParticleEmitter::new(vec![
                field_particle::FieldParticle::MonsterCorpse {
                    particle: corpse_particle,
                },
            ]),
        },
    ]);

    game_state.earn_gold(earn);
    game_state.monsters.swap_remove(target_idx);
}
