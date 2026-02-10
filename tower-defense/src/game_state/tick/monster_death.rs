use super::*;

pub fn handle_monster_death(
    game_state: &mut GameState,
    target_idx: usize,
    target_xy: Xy<f32>,
    now: Instant,
) {
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

    let wh = monster::monster_wh(monster_kind);

    let tile_base_xy = TILE_PX_SIZE.to_xy() * target_xy;
    let monster_center_offset = Xy::new(
        TILE_PX_SIZE.width * 0.5,
        TILE_PX_SIZE.height - wh.height * 0.5 + TILE_PX_SIZE.height * y_offset,
    );
    let pixel_xy = tile_base_xy + monster_center_offset;

    field_particle::MONSTER_SOULS.spawn(
        field_particle::MonsterSoulParticle::new(pixel_xy, now, rotation),
    );

    field_particle::MONSTER_CORPSES.spawn(
        field_particle::MonsterCorpseParticle::new(pixel_xy, now, rotation, monster_kind, wh),
    );

    game_state.earn_gold(earn);
    game_state.monsters.swap_remove(target_idx);
}
