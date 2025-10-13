use super::*;
use std::{collections::VecDeque, iter, vec};

#[derive(State)]
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
    game_state.monsters.push(Monster::new(
        &next_monster_template,
        game_state.route.clone(),
        now,
        health_multiplier,
    ));

    *next_spawn_time = now + *spawn_interval;
}

fn monster_queue_table(stage: usize) -> (VecDeque<MonsterKind>, Duration) {
    let spawn_interval =
        Duration::from_millis((10000.0 / (26.0 * (stage as f32 / 50.0) + 4.0)) as i64);

    let monster_queue = match stage {
        1 => vec![(MonsterKind::Mob01, 4)],
        2 => vec![(MonsterKind::Mob02, 1), (MonsterKind::Mob01, 3)],
        3 => vec![(MonsterKind::Mob02, 4)],
        4 => vec![(MonsterKind::Mob03, 1), (MonsterKind::Mob02, 3)],
        5 => vec![(MonsterKind::Named01, 1), (MonsterKind::Mob01, 3)],
        6 => vec![(MonsterKind::Mob04, 1), (MonsterKind::Mob03, 3)],
        7 => vec![(MonsterKind::Mob04, 4)],
        8 => vec![
            (MonsterKind::Named02, 1),
            (MonsterKind::Mob05, 1),
            (MonsterKind::Mob04, 4),
        ],
        9 => vec![(MonsterKind::Mob05, 4), (MonsterKind::Mob02, 2)],
        10 => vec![
            (MonsterKind::Mob06, 1),
            (MonsterKind::Mob05, 2),
            (MonsterKind::Mob02, 3),
        ],
        11 => vec![
            (MonsterKind::Named03, 1),
            (MonsterKind::Mob06, 2),
            (MonsterKind::Mob05, 5),
        ],
        12 => vec![(MonsterKind::Mob06, 5), (MonsterKind::Mob03, 3)],
        13 => vec![
            (MonsterKind::Mob07, 1),
            (MonsterKind::Mob06, 3),
            (MonsterKind::Mob03, 4),
        ],
        14 => vec![
            (MonsterKind::Named04, 1),
            (MonsterKind::Mob07, 3),
            (MonsterKind::Mob06, 6),
        ],
        15 => vec![
            (MonsterKind::Boss01, 1),
            (MonsterKind::Mob07, 5),
            (MonsterKind::Mob04, 4),
        ],
        16 => vec![
            (MonsterKind::Mob08, 1),
            (MonsterKind::Mob07, 4),
            (MonsterKind::Mob04, 5),
        ],
        17 => vec![
            (MonsterKind::Named05, 1),
            (MonsterKind::Mob08, 4),
            (MonsterKind::Mob07, 7),
        ],
        18 => vec![(MonsterKind::Mob08, 7), (MonsterKind::Mob05, 5)],
        19 => vec![
            (MonsterKind::Mob09, 1),
            (MonsterKind::Mob08, 5),
            (MonsterKind::Mob05, 6),
        ],
        20 => vec![
            (MonsterKind::Named06, 1),
            (MonsterKind::Mob09, 5),
            (MonsterKind::Mob08, 8),
        ],
        21 => vec![(MonsterKind::Mob09, 8), (MonsterKind::Mob08, 6)],
        22 => vec![
            (MonsterKind::Mob10, 1),
            (MonsterKind::Mob09, 6),
            (MonsterKind::Mob06, 7),
        ],
        23 => vec![
            (MonsterKind::Named07, 1),
            (MonsterKind::Mob10, 6),
            (MonsterKind::Mob09, 9),
        ],
        24 => vec![(MonsterKind::Mob10, 9), (MonsterKind::Mob09, 7)],
        25 => vec![
            (MonsterKind::Boss02, 1),
            (MonsterKind::Mob11, 1),
            (MonsterKind::Mob10, 6),
            (MonsterKind::Mob07, 8),
        ],
        26 => vec![
            (MonsterKind::Named08, 1),
            (MonsterKind::Mob11, 7),
            (MonsterKind::Mob10, 10),
        ],
        27 => vec![(MonsterKind::Mob11, 10), (MonsterKind::Mob10, 8)],
        28 => vec![
            (MonsterKind::Mob12, 1),
            (MonsterKind::Mob11, 8),
            (MonsterKind::Mob08, 9),
        ],
        29 => vec![
            (MonsterKind::Named09, 1),
            (MonsterKind::Mob12, 8),
            (MonsterKind::Mob11, 11),
        ],
        30 => vec![
            (MonsterKind::Boss03, 1),
            (MonsterKind::Mob12, 10),
            (MonsterKind::Mob11, 9),
        ],
        31 => vec![
            (MonsterKind::Mob13, 1),
            (MonsterKind::Mob12, 9),
            (MonsterKind::Mob09, 10),
        ],
        32 => vec![
            (MonsterKind::Named10, 1),
            (MonsterKind::Mob13, 9),
            (MonsterKind::Mob12, 12),
        ],
        33 => vec![(MonsterKind::Mob13, 12), (MonsterKind::Mob12, 10)],
        34 => vec![
            (MonsterKind::Mob14, 1),
            (MonsterKind::Mob13, 10),
            (MonsterKind::Mob10, 11),
        ],
        35 => vec![
            (MonsterKind::Boss04, 1),
            (MonsterKind::Named11, 1),
            (MonsterKind::Mob14, 9),
            (MonsterKind::Mob13, 13),
        ],
        36 => vec![(MonsterKind::Mob14, 13), (MonsterKind::Mob13, 11)],
        37 => vec![
            (MonsterKind::Mob15, 1),
            (MonsterKind::Mob14, 11),
            (MonsterKind::Mob11, 12),
        ],
        38 => vec![
            (MonsterKind::Named12, 1),
            (MonsterKind::Mob15, 11),
            (MonsterKind::Mob14, 14),
        ],
        39 => vec![(MonsterKind::Mob15, 14), (MonsterKind::Mob14, 12)],
        40 => vec![
            (MonsterKind::Boss05, 1),
            (MonsterKind::Mob15, 12),
            (MonsterKind::Mob12, 13),
        ],
        41 => vec![
            (MonsterKind::Named13, 1),
            (MonsterKind::Mob15, 20),
            (MonsterKind::Mob14, 7),
        ],
        42 => vec![(MonsterKind::Mob15, 16), (MonsterKind::Mob14, 12)],
        43 => vec![
            (MonsterKind::Named11, 1),
            (MonsterKind::Named10, 1),
            (MonsterKind::Mob15, 12),
            (MonsterKind::Mob14, 14),
        ],
        44 => vec![
            (MonsterKind::Named14, 1),
            (MonsterKind::Named11, 1),
            (MonsterKind::Mob15, 18),
            (MonsterKind::Mob14, 10),
        ],
        45 => vec![
            (MonsterKind::Boss06, 1),
            (MonsterKind::Named11, 1),
            (MonsterKind::Named09, 1),
            (MonsterKind::Mob15, 20),
            (MonsterKind::Mob14, 7),
        ],
        46 => vec![
            (MonsterKind::Boss07, 1),
            (MonsterKind::Named11, 1),
            (MonsterKind::Named09, 1),
            (MonsterKind::Mob15, 24),
            (MonsterKind::Mob14, 3),
        ],
        47 => vec![
            (MonsterKind::Boss08, 1),
            (MonsterKind::Named15, 1),
            (MonsterKind::Named13, 1),
            (MonsterKind::Named12, 1),
            (MonsterKind::Mob15, 24),
            (MonsterKind::Mob14, 4),
        ],
        48 => vec![
            (MonsterKind::Boss09, 1),
            (MonsterKind::Named14, 1),
            (MonsterKind::Named13, 1),
            (MonsterKind::Named12, 1),
            (MonsterKind::Mob15, 28),
        ],
        49 => vec![
            (MonsterKind::Boss10, 1),
            (MonsterKind::Named15, 1),
            (MonsterKind::Named14, 1),
            (MonsterKind::Named13, 1),
            (MonsterKind::Mob15, 28),
        ],
        50 => vec![
            (MonsterKind::Boss11, 1),
            (MonsterKind::Named16, 1),
            (MonsterKind::Named15, 1),
            (MonsterKind::Named14, 1),
            (MonsterKind::Mob15, 30),
        ],
        _ => unimplemented!(),
    }
    .iter()
    .flat_map(|(kind, count)| iter::repeat_n(*kind, *count))
    .collect::<VecDeque<_>>();

    (monster_queue, spawn_interval)
}
