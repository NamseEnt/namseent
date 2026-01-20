use crate::game_state::MonsterKind;
use rand::{seq::SliceRandom, thread_rng};

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
