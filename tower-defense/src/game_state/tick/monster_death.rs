use super::*;
use crate::game_state::upgrade::UpgradeBehavior;

pub fn handle_monster_death(
    game_state: &mut GameState,
    target_idx: usize,
    target_xy: Xy<f32>,
    now: Instant,
) {
    if target_idx >= game_state.monsters.len() {
        return;
    }
    let monster_max_hp = game_state.monsters[target_idx].max_hp;
    let monster_reward = game_state.monsters[target_idx].reward;
    let monster_kind = game_state.monsters[target_idx].kind;
    let rotation = game_state.monsters[target_idx].animation.rotation;

    if let GameFlow::Defense(defense_flow) = &mut game_state.flow {
        defense_flow.stage_progress.processed_hp += monster_max_hp;
    }

    let earn = monster_reward + game_state.upgrade_state.gold_earn_plus();
    let earn = (earn as f32 * game_state.stage_modifiers.get_gold_gain_multiplier()) as usize;

    let wh = monster::monster_wh(monster_kind);

    let pixel_xy = TILE_PX_SIZE.to_xy() * target_xy;

    game_state
        .effect_events
        .push(GameEffectEvent::SpawnParticle(
            ParticleSpawnRequest::MonsterSoul(field_particle::MonsterSoulParticle::new(
                pixel_xy, now, rotation,
            )),
        ));

    game_state
        .effect_events
        .push(GameEffectEvent::SpawnParticle(
            ParticleSpawnRequest::MonsterCorpse(field_particle::MonsterCorpseParticle::new(
                pixel_xy,
                now,
                rotation,
                monster_kind,
                wh,
            )),
        ));

    game_state.action(crate::game_state::GameStateAction::EarnGold(earn));
    let mut recovered = false;
    let mut index = 0;
    while index < game_state.upgrade_state.upgrades.len() {
        let mut upgrade = game_state.upgrade_state.upgrades.remove(index);
        recovered |= upgrade.on_monster_death(game_state);
        game_state.upgrade_state.upgrades.insert(index, upgrade);
        index += 1;
    }
    if recovered {
        game_state.hp = (game_state.hp + 1.0).min(game_state.max_hp());
    }
    game_state.monsters.swap_remove(target_idx);
}
