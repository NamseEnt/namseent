use super::append_named_to_queue;
use crate::game_state::*;
use crate::route::Route;
use std::collections::VecDeque;
use std::sync::Arc;

pub fn start_spawn(game_state: &mut GameState) {
    if game_state.monster_spawn_state.is_spawning() {
        return;
    }

    let health_multiplier = game_state.stage_modifiers.get_enemy_health_multiplier();
    let now = game_state.now();
    let (mut monster_queue, spawn_interval) = monster_queue_table(
        game_state.stage,
        game_state.route.clone(),
        now,
        health_multiplier,
    );

    // 선택된 named 몬스터들을 큐에 추가
    let selected_monsters: Vec<MonsterTemplate> = game_state
        .monster_spawn_state
        .challenge_choices
        .iter()
        .zip(game_state.monster_spawn_state.challenge_selected.iter())
        .filter_map(|(template, &selected)| selected.then_some(template.clone()))
        .collect();

    append_named_to_queue(
        &mut monster_queue,
        &selected_monsters[..],
        game_state.route.clone(),
        now,
        health_multiplier,
    );

    game_state.monster_spawn_state.monster_queue = monster_queue;
    game_state.monster_spawn_state.spawn_interval = spawn_interval;
    game_state.monster_spawn_state.next_spawn_time = Some(now);
}

pub fn tick(game_state: &mut GameState, now: namui::Instant) {
    if let Some(next_time) = game_state.monster_spawn_state.next_spawn_time
        && now < next_time
    {
        return;
    }

    let Some(mut next_monster) = game_state.monster_spawn_state.monster_queue.pop_front() else {
        game_state.monster_spawn_state.next_spawn_time = None;
        return;
    };

    for skill in next_monster.skills.iter_mut() {
        skill.last_used_at = now;
    }

    #[cfg(feature = "debug-tools")]
    {
        let hp_offset = crate::game_state::debug_tools::monster_hp_balance::get_hp_offset();
        next_monster.max_hp += hp_offset;
        next_monster.hp = next_monster.max_hp;
    }

    game_state.monsters.push(next_monster);

    game_state.monster_spawn_state.next_spawn_time =
        Some(now + game_state.monster_spawn_state.spawn_interval);
}

pub fn monster_queue_table(
    stage: usize,
    route: Arc<Route>,
    now: namui::Instant,
    health_multiplier: f32,
) -> (VecDeque<Monster>, namui::Duration) {
    let (template_queue, spawn_interval) = monster_template_queue_table(stage);

    let monster_queue = template_queue
        .into_iter()
        .map(|template| Monster::new(&template, route.clone(), now, health_multiplier))
        .collect();

    (monster_queue, spawn_interval)
}

pub fn monster_template_queue_table(stage: usize) -> (VecDeque<MonsterTemplate>, namui::Duration) {
    let spawn_interval =
        namui::Duration::from_millis((10000.0 / (26.0 * (stage as f32 / 50.0) + 4.0)) as i64);

    let template_queue = match stage {
        1 => vec![(MonsterKind::Mob01, 5)],
        2 => vec![(MonsterKind::Mob02, 5)],
        3 => vec![(MonsterKind::Mob03, 5)],
        4 => vec![(MonsterKind::Mob04, 5)],
        5 => vec![(MonsterKind::Mob05, 5)],
        6 => vec![(MonsterKind::Mob06, 7)],
        7 => vec![(MonsterKind::Mob07, 7)],
        8 => vec![(MonsterKind::Mob08, 7)],
        9 => vec![(MonsterKind::Mob09, 7)],
        10 => vec![(MonsterKind::Mob10, 7)],
        11 => vec![(MonsterKind::Mob11, 9)],
        12 => vec![(MonsterKind::Mob12, 9)],
        13 => vec![(MonsterKind::Mob13, 9)],
        14 => vec![(MonsterKind::Mob14, 9)],
        15 => vec![(MonsterKind::Mob15, 8), (MonsterKind::Boss01, 1)],
        16 => vec![(MonsterKind::Mob16, 10)],
        17 => vec![(MonsterKind::Mob17, 10)],
        18 => vec![(MonsterKind::Mob18, 10)],
        19 => vec![(MonsterKind::Mob19, 10)],
        20 => vec![(MonsterKind::Mob20, 10)],
        21 => vec![(MonsterKind::Mob21, 11)],
        22 => vec![(MonsterKind::Mob22, 11)],
        23 => vec![(MonsterKind::Mob23, 11)],
        24 => vec![(MonsterKind::Mob24, 11)],
        25 => vec![(MonsterKind::Mob25, 10), (MonsterKind::Boss02, 1)],
        26 => vec![(MonsterKind::Mob26, 11)],
        27 => vec![(MonsterKind::Mob27, 11)],
        28 => vec![(MonsterKind::Mob28, 11)],
        29 => vec![(MonsterKind::Mob29, 11)],
        30 => vec![(MonsterKind::Mob30, 10), (MonsterKind::Boss03, 1)],
        31 => vec![(MonsterKind::Mob31, 12)],
        32 => vec![(MonsterKind::Mob32, 12)],
        33 => vec![(MonsterKind::Mob33, 12)],
        34 => vec![(MonsterKind::Mob34, 12)],
        35 => vec![(MonsterKind::Mob35, 11), (MonsterKind::Boss04, 1)],
        36 => vec![(MonsterKind::Mob36, 13)],
        37 => vec![(MonsterKind::Mob37, 13)],
        38 => vec![(MonsterKind::Mob38, 13)],
        39 => vec![(MonsterKind::Mob39, 13)],
        40 => vec![(MonsterKind::Mob40, 12), (MonsterKind::Boss05, 1)],
        41 => vec![(MonsterKind::Mob41, 14)],
        42 => vec![(MonsterKind::Mob42, 14)],
        43 => vec![(MonsterKind::Mob43, 14)],
        44 => vec![(MonsterKind::Mob44, 14)],
        45 => vec![(MonsterKind::Mob45, 13), (MonsterKind::Boss06, 1)],
        46 => vec![(MonsterKind::Mob46, 14), (MonsterKind::Boss07, 1)],
        47 => vec![(MonsterKind::Mob47, 14), (MonsterKind::Boss08, 1)],
        48 => vec![(MonsterKind::Mob48, 14), (MonsterKind::Boss09, 1)],
        49 => vec![(MonsterKind::Mob49, 14), (MonsterKind::Boss10, 1)],
        50 => vec![(MonsterKind::Mob50, 14), (MonsterKind::Boss11, 1)],
        _ => unimplemented!(),
    }
    .iter()
    .flat_map(|(kind, count)| std::iter::repeat_n(*kind, *count))
    .map(MonsterTemplate::new)
    .collect::<VecDeque<_>>();

    (template_queue, spawn_interval)
}
