use crate::rarity::Rarity;
use rand::{Rng, RngCore};

/// 고정 rarity 값에 따른 숫자 선택
pub(crate) fn rarity_based_amount(
    rarity: Rarity,
    common: f32,
    rare: f32,
    epic: f32,
    legendary: f32,
) -> f32 {
    match rarity {
        Rarity::Common => common,
        Rarity::Rare => rare,
        Rarity::Epic => epic,
        Rarity::Legendary => legendary,
    }
}

/// 간결화를 위한 헬퍼: (f32,f32) 튜플 4개짜리 rarity 테이블에서 해당 rarity 구간을 뽑아 난수.
/// 기존 `rarity_based_random_amount_with_rng` 가 4개의 Range 를 직접 받는 패턴 중복을 줄이기 위해 추가.
pub(crate) fn rarity_table_random(
    rng: &mut dyn RngCore,
    rarity: Rarity,
    table: &[(f32, f32); 4],
) -> f32 {
    let (lo, hi) = match rarity {
        Rarity::Common => table[0],
        Rarity::Rare => table[1],
        Rarity::Epic => table[2],
        Rarity::Legendary => table[3],
    };
    rng.gen_range(lo..hi)
}

/// Stage 단위로 total 값을 duration 에 나누어 (0.8x, 1.2x) 범위로 정수 경계 보정.
/// 동일 패턴이 reward / risk stage start 생성 코드에 3회 반복되어 추출.
/// 분포 범위를 조정하려면 아래 상수만 수정하면 된다.
pub(crate) const STAGE_DISTRIBUTION_MIN_MULTIPLIER: f32 = 0.8; // 하한 배율
pub(crate) const STAGE_DISTRIBUTION_MAX_MULTIPLIER: f32 = 1.2; // 상한 배율
pub(crate) fn distribute_per_stage(total: f32, duration_stages: usize) -> (f32, f32) {
    let base = (total / duration_stages as f32).max(1.0);
    let min_amount = (base * STAGE_DISTRIBUTION_MIN_MULTIPLIER).floor();
    let max_amount = (base * STAGE_DISTRIBUTION_MAX_MULTIPLIER).ceil();
    (min_amount, max_amount)
}
