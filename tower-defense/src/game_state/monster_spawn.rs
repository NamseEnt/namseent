use super::*;
use std::{collections::VecDeque, iter, vec};

#[derive(State, Clone)]
pub enum MonsterSpawnState {
    Idle,
    Spawning {
        monster_queue: VecDeque<MonsterKind>,
        next_spawn_time: Instant,
        spawn_interval: Duration,
    },
}

/// This won't immediately spawn a monster or update game_state,
/// but it just requests to start spawning a monster.
pub fn start_spawn(game_state: &mut GameState) {
    if !matches!(game_state.monster_spawn_state, MonsterSpawnState::Idle) {
        return;
    }

    let (monster_queue, spawn_interval) = monster_queue_table(game_state.stage);

    game_state.monster_spawn_state = MonsterSpawnState::Spawning {
        monster_queue,
        next_spawn_time: game_state.now(),
        spawn_interval,
    };
}

pub fn tick(game_state: &mut GameState, now: Instant) {
    let MonsterSpawnState::Spawning {
        monster_queue,
        next_spawn_time,
        spawn_interval,
    } = &mut game_state.monster_spawn_state
    else {
        return;
    };

    if now < *next_spawn_time {
        return;
    }

    let Some(next_monster_kind) = monster_queue.pop_front() else {
        game_state.monster_spawn_state = MonsterSpawnState::Idle;
        return;
    };

    let next_monster_template = MonsterTemplate::new(next_monster_kind);
    let health_multiplier = game_state.stage_modifiers.get_enemy_health_multiplier();
    #[allow(unused_mut)]
    let mut monster = Monster::new(
        &next_monster_template,
        game_state.route.clone(),
        now,
        health_multiplier,
    );

    #[cfg(feature = "debug-tools")]
    {
        let hp_offset = crate::game_state::debug_tools::monster_hp_balance::get_hp_offset();
        monster.max_hp += hp_offset;
        monster.hp = monster.max_hp;
    }

    game_state.monsters.push(monster);

    *next_spawn_time = now + *spawn_interval;
}

pub fn monster_queue_table(stage: usize) -> (VecDeque<MonsterKind>, Duration) {
    let spawn_interval =
        Duration::from_millis((10000.0 / (26.0 * (stage as f32 / 50.0) + 4.0)) as i64);

    let monster_queue = match stage {
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
        15 => vec![(MonsterKind::Mob15, 9)],
        16 => vec![(MonsterKind::Mob16, 10)],
        17 => vec![(MonsterKind::Mob17, 10)],
        18 => vec![(MonsterKind::Mob18, 10)],
        19 => vec![(MonsterKind::Mob19, 10)],
        20 => vec![(MonsterKind::Mob20, 10)],
        21 => vec![(MonsterKind::Mob21, 11)],
        22 => vec![(MonsterKind::Mob22, 11)],
        23 => vec![(MonsterKind::Mob23, 11)],
        24 => vec![(MonsterKind::Mob24, 11)],
        25 => vec![(MonsterKind::Mob25, 11)],
        26 => vec![(MonsterKind::Mob26, 11)],
        27 => vec![(MonsterKind::Mob27, 11)],
        28 => vec![(MonsterKind::Mob28, 11)],
        29 => vec![(MonsterKind::Mob29, 11)],
        30 => vec![(MonsterKind::Mob30, 11)],
        31 => vec![(MonsterKind::Mob31, 12)],
        32 => vec![(MonsterKind::Mob32, 12)],
        33 => vec![(MonsterKind::Mob33, 12)],
        34 => vec![(MonsterKind::Mob34, 12)],
        35 => vec![(MonsterKind::Mob35, 12)],
        36 => vec![(MonsterKind::Mob36, 13)],
        37 => vec![(MonsterKind::Mob37, 13)],
        38 => vec![(MonsterKind::Mob38, 13)],
        39 => vec![(MonsterKind::Mob39, 13)],
        40 => vec![(MonsterKind::Mob40, 13)],
        41 => vec![(MonsterKind::Mob41, 14)],
        42 => vec![(MonsterKind::Mob42, 14)],
        43 => vec![(MonsterKind::Mob43, 14)],
        44 => vec![(MonsterKind::Mob44, 14)],
        45 => vec![(MonsterKind::Mob45, 14)],
        46 => vec![(MonsterKind::Mob46, 15)],
        47 => vec![(MonsterKind::Mob47, 15)],
        48 => vec![(MonsterKind::Mob48, 15)],
        49 => vec![(MonsterKind::Mob49, 15)],
        50 => vec![(MonsterKind::Mob50, 15)],
        _ => unimplemented!(),
    }
    .iter()
    .flat_map(|(kind, count)| iter::repeat_n(*kind, *count))
    .collect::<VecDeque<_>>();

    (monster_queue, spawn_interval)
}
