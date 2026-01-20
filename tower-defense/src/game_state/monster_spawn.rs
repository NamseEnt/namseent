use super::*;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::{HashSet, VecDeque};
use std::{iter, vec};

#[derive(State, Clone)]
pub struct MonsterSpawnState {
    pub monster_queue: VecDeque<MonsterKind>,
    pub next_spawn_time: Option<Instant>,
    pub spawn_interval: Duration,
    pub challenge_choices: [MonsterKind; 3],
    pub challenge_selected: [bool; 3],
}

impl MonsterSpawnState {
    pub fn idle() -> Self {
        Self {
            monster_queue: VecDeque::new(),
            next_spawn_time: None,
            // arbitrary default; real value set when spawning starts
            spawn_interval: Duration::from_millis(0),
            challenge_choices: [MonsterKind::Named01; 3],
            challenge_selected: [false; 3],
        }
    }

    pub fn is_spawning(&self) -> bool {
        self.next_spawn_time.is_some()
    }

    pub fn is_idle(&self) -> bool {
        self.next_spawn_time.is_none() && self.monster_queue.is_empty()
    }

    pub fn reset_challenge_selection(&mut self) {
        self.challenge_selected = [false; 3];
    }

    pub fn toggle_challenge_selection(&mut self, index: usize) {
        if index < 3 {
            self.challenge_selected[index] = !self.challenge_selected[index];
        }
    }
}

/// This won't immediately spawn a monster or update game_state,
/// but it just requests to start spawning a monster.
pub fn start_spawn(game_state: &mut GameState) {
    if game_state.monster_spawn_state.is_spawning() {
        return;
    }

    let (mut monster_queue, spawn_interval) = monster_queue_table(game_state.stage);

    // 선택된 named 몬스터들을 큐에 추가
    let selected_monsters: Vec<_> = game_state
        .monster_spawn_state
        .challenge_choices
        .iter()
        .zip(game_state.monster_spawn_state.challenge_selected.iter())
        .filter_map(|(kind, &selected)| selected.then_some(*kind))
        .collect();

    append_named_to_queue(&mut monster_queue, &selected_monsters);

    game_state.monster_spawn_state.monster_queue = monster_queue;
    game_state.monster_spawn_state.spawn_interval = spawn_interval;
    game_state.monster_spawn_state.next_spawn_time = Some(game_state.now());
}

pub fn tick(game_state: &mut GameState, now: Instant) {
    if let Some(next_time) = game_state.monster_spawn_state.next_spawn_time
        && now < next_time
    {
        return;
    }

    let Some(next_monster_kind) = game_state.monster_spawn_state.monster_queue.pop_front() else {
        game_state.monster_spawn_state.next_spawn_time = None;
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

    game_state.monster_spawn_state.next_spawn_time =
        Some(now + game_state.monster_spawn_state.spawn_interval);
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
    .flat_map(|(kind, count)| iter::repeat_n(*kind, *count))
    .collect::<VecDeque<_>>();

    (monster_queue, spawn_interval)
}

const NAMED_MONSTER_ORDER: [MonsterKind; 16] = [
    MonsterKind::Named01,
    MonsterKind::Named02,
    MonsterKind::Named03,
    MonsterKind::Named04,
    MonsterKind::Named05,
    MonsterKind::Named06,
    MonsterKind::Named07,
    MonsterKind::Named08,
    MonsterKind::Named09,
    MonsterKind::Named10,
    MonsterKind::Named11,
    MonsterKind::Named12,
    MonsterKind::Named13,
    MonsterKind::Named14,
    MonsterKind::Named15,
    MonsterKind::Named16,
];

fn append_named_to_queue(queue: &mut VecDeque<MonsterKind>, extras: &[MonsterKind]) {
    if extras.is_empty() {
        return;
    }

    let mut seen: HashSet<MonsterKind> = queue.iter().copied().collect();
    for kind in extras {
        if seen.insert(*kind) {
            queue.push_back(*kind);
        }
    }
}

pub fn named_candidate_pool_for_stage(stage: usize) -> Vec<MonsterKind> {
    let window = stage.saturating_sub(1) / 5; // 0-based window per 5 levels
    let start = 1 + window;
    let end = (5 + window).min(NAMED_MONSTER_ORDER.len());

    NAMED_MONSTER_ORDER
        .iter()
        .copied()
        .skip(start.saturating_sub(1))
        .take(end.saturating_sub(start.saturating_sub(1)))
        .collect()
}

pub fn pick_challenge_named_choices(stage: usize) -> [MonsterKind; 3] {
    let pool = named_candidate_pool_for_stage(stage);
    let mut rng = thread_rng();

    // Pick up to 3 unique monsters randomly
    let picks: Vec<_> = pool.choose_multiple(&mut rng, 3).copied().collect();

    // Fallback: if pool is smaller than 3, reuse the first element to fill
    let mut result = [pool[0]; 3];
    for (i, kind) in picks.into_iter().enumerate() {
        result[i] = kind;
    }
    result
}
