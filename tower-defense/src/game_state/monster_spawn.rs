use super::*;

pub enum MonsterSpawnState {
    Idle,
    Spawning {
        left_spawn_count: NonZeroUsize,
        next_spawn_time: Instant,
        spawn_interval: Duration,
        monster_kind: MonsterKind,
        velocity: Velocity,
    },
}

/// This won't immediately spawn a monster or update game_state,
/// but it just requests to start spawning a monster.
pub fn start_spawn(
    monster_kind: MonsterKind,
    monster_velocity: Velocity,
    spawn_count: NonZeroUsize,
    spawn_interval: Duration,
) {
    crate::game_state::mutate_game_state(move |game_state| {
        if !matches!(game_state.monster_spawn_state, MonsterSpawnState::Idle) {
            return;
        }

        game_state.monster_spawn_state = MonsterSpawnState::Spawning {
            left_spawn_count: spawn_count,
            next_spawn_time: Instant::now(),
            spawn_interval,
            monster_kind,
            velocity: monster_velocity,
        };
    });
}

pub fn spawn_tick(game_state: &mut GameState, now: Instant) {
    let MonsterSpawnState::Spawning {
        left_spawn_count,
        next_spawn_time,
        spawn_interval,
        monster_kind,
        velocity,
    } = &mut game_state.monster_spawn_state
    else {
        return;
    };

    if now < *next_spawn_time {
        return;
    }

    game_state.monsters.push(Monster {
        move_on_route: MoveOnRoute::new(game_state.route.clone(), *velocity),
        kind: *monster_kind,
        projectile_target_indicator: ProjectileTargetIndicator::new(),
    });

    if left_spawn_count.get() == 1 {
        game_state.monster_spawn_state = MonsterSpawnState::Idle;
        return;
    }

    *left_spawn_count = NonZeroUsize::new(left_spawn_count.get() - 1).unwrap();
    *next_spawn_time = now + *spawn_interval;
}
