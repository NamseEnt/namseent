use super::*;

pub enum MonsterSpawnState {
    Idle,
    Spawning {
        left_spawn_count: NonZeroUsize,
        next_spawn_time: Instant,
        spawn_interval: Duration,
        template: MonsterTemplate,
    },
}

/// This won't immediately spawn a monster or update game_state,
/// but it just requests to start spawning a monster.
pub fn start_spawn(template: MonsterTemplate, spawn_count: NonZeroUsize, spawn_interval: Duration) {
    crate::game_state::mutate_game_state(move |game_state| {
        if !matches!(game_state.monster_spawn_state, MonsterSpawnState::Idle) {
            return;
        }

        game_state.monster_spawn_state = MonsterSpawnState::Spawning {
            left_spawn_count: spawn_count,
            next_spawn_time: Instant::now(),
            spawn_interval,
            template,
        };
    });
}

pub fn tick(game_state: &mut GameState, now: Instant) {
    let MonsterSpawnState::Spawning {
        left_spawn_count,
        next_spawn_time,
        spawn_interval,
        template,
    } = &mut game_state.monster_spawn_state
    else {
        return;
    };

    if now < *next_spawn_time {
        return;
    }

    game_state
        .monsters
        .push(Monster::new(template, game_state.route.clone()));

    if left_spawn_count.get() == 1 {
        game_state.monster_spawn_state = MonsterSpawnState::Idle;
        return;
    }

    *left_spawn_count = NonZeroUsize::new(left_spawn_count.get() - 1).unwrap();
    *next_spawn_time = now + *spawn_interval;
}
