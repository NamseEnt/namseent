use super::*;

pub fn handle_monster_death(
    game_state: &mut GameState,
    target_idx: usize,
    target_xy: crate::WorldCoord,
    presentation_instant: crate::PresentationInstant,
) {
    if target_idx >= game_state.monsters.len() {
        return;
    }
    let monster_reward = game_state.monsters[target_idx].reward;
    let monster_kind = game_state.monsters[target_idx].kind;
    let rotation = game_state.monsters[target_idx].animation.rotation;

    if !game_state.monsters[target_idx].stage_progress_counted {
        let remaining_hp = game_state.monsters[target_idx].hp;
        if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
            defense_flow.stage_progress.processed_hp = defense_flow
                .stage_progress
                .processed_hp
                .saturating_add(remaining_hp);
        }
        game_state.monsters[target_idx].stage_progress_counted = true;
    }

    let earn = RatioProduct::one()
        .with_all(
            game_state
                .stage_modifiers
                .gold_gain_multipliers()
                .iter()
                .copied(),
        )
        .apply_usize(monster_reward);

    let wh = monster::monster_wh(monster_kind);

    let pixel_xy = TILE_PX_SIZE.to_xy() * target_xy.as_map_coord_f32();

    game_state
        .effect_events
        .push(GameEffectEvent::SpawnParticle(
            ParticleSpawnRequest::MonsterSoul(field_particle::MonsterSoulParticle::new(
                pixel_xy,
                presentation_instant.as_namui(),
                rotation,
            )),
        ));

    game_state
        .effect_events
        .push(GameEffectEvent::SpawnParticle(
            ParticleSpawnRequest::MonsterCorpse(field_particle::MonsterCorpseParticle::new(
                pixel_xy,
                presentation_instant.as_namui(),
                rotation,
                monster_kind,
                wh,
            )),
        ));

    game_state.action(crate::game_state::GameStateAction::EarnGold(earn));
    game_state.action(crate::game_state::GameStateAction::MonsterDeath);
    game_state.monsters.swap_remove(target_idx);
}
