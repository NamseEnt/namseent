use super::*;
use std::{array, collections::VecDeque};

pub enum MonsterSpawnState {
    Idle,
    Spawning {
        monster_queue: VecDeque<MonsterTemplate>,
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
        next_spawn_time: Instant::now(),
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

    let Some(next_monster_template) = monster_queue.pop_front() else {
        game_state.monster_spawn_state = MonsterSpawnState::Idle;
        return;
    };

    game_state.monsters.push(Monster::new(
        &next_monster_template,
        game_state.route.clone(),
    ));

    *next_spawn_time = now + *spawn_interval;
}

fn monster_queue_table(stage: usize) -> (VecDeque<MonsterTemplate>, Duration) {
    let spawn_interval = Duration::from_millis(match stage {
        0..5 => 2000,
        5..15 => 1500,
        15..35 => 1000,
        _ => 750,
    });

    let monster_queue = match stage {
        1 => VecDeque::from_iter(
            array::from_fn::<_, 5, _>(|_| MonsterTemplate::new_mob_01()).into_iter(),
        ),
        2 => VecDeque::from_iter(
            array::from_fn::<_, 10, _>(|_| MonsterTemplate::new_mob_01()).into_iter(),
        ),
        _ => unimplemented!(),
    };

    (monster_queue, spawn_interval)
}
