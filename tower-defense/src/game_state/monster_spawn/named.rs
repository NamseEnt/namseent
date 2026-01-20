use crate::game_state::monster::{
    MonsterKind, MonsterSkillTemplate, MonsterTemplate, PrebuiltSkill,
};
use crate::route::Velocity;
use crate::*;
use namui::State;
use rand::{Rng, seq::SliceRandom, thread_rng};
use std::array::from_fn;

#[derive(State, Clone)]
pub struct NamedMonsterConfig {
    pub kind: MonsterKind,
    pub max_hp: f32,
    pub velocity: Velocity,
    pub reward: usize,
    pub skills: Vec<MonsterSkillTemplate>,
}

impl NamedMonsterConfig {
    pub fn from_kind(kind: MonsterKind) -> Self {
        let template = MonsterTemplate::new(kind);
        Self {
            kind,
            max_hp: template.max_hp,
            velocity: template.velocity,
            reward: template.reward,
            skills: template.skills,
        }
    }
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

const SKILL_ORDER: [PrebuiltSkill; 16] = [
    PrebuiltSkill::ImmuneSlow02,
    PrebuiltSkill::Invincible03,
    PrebuiltSkill::Speedmul03,
    PrebuiltSkill::ImmuneSlow03,
    PrebuiltSkill::Heal01,
    PrebuiltSkill::ImmuneSlow04,
    PrebuiltSkill::Heal02,
    PrebuiltSkill::Speedmul04,
    PrebuiltSkill::Invincible04,
    PrebuiltSkill::Heal03,
    PrebuiltSkill::Heal04,
    PrebuiltSkill::Speedmul01,
    PrebuiltSkill::ImmuneSlow01,
    PrebuiltSkill::Invincible01,
    PrebuiltSkill::Invincible02,
    PrebuiltSkill::ImmuneSlow02,
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

fn skill_for_choice(stage: usize, choice_idx: usize, rng: &mut impl Rng) -> PrebuiltSkill {
    let window = stage.saturating_sub(1) / 5;
    let base = window * 3;
    let ranges = [(0, 3), (3, 5), (5, 7)];
    let (start, end) = ranges[choice_idx];
    let span = end - start + 1;
    let pick = rng.gen_range(0..span);
    let idx = (base + start + pick) % SKILL_ORDER.len();
    SKILL_ORDER[idx]
}

fn base_monster_kind_for_stage(stage: usize) -> MonsterKind {
    match stage {
        1 => MonsterKind::Mob01,
        2 => MonsterKind::Mob02,
        3 => MonsterKind::Mob03,
        4 => MonsterKind::Mob04,
        5 => MonsterKind::Mob05,
        6 => MonsterKind::Mob06,
        7 => MonsterKind::Mob07,
        8 => MonsterKind::Mob08,
        9 => MonsterKind::Mob09,
        10 => MonsterKind::Mob10,
        11 => MonsterKind::Mob11,
        12 => MonsterKind::Mob12,
        13 => MonsterKind::Mob13,
        14 => MonsterKind::Mob14,
        15 => MonsterKind::Mob15,
        16 => MonsterKind::Mob16,
        17 => MonsterKind::Mob17,
        18 => MonsterKind::Mob18,
        19 => MonsterKind::Mob19,
        20 => MonsterKind::Mob20,
        21 => MonsterKind::Mob21,
        22 => MonsterKind::Mob22,
        23 => MonsterKind::Mob23,
        24 => MonsterKind::Mob24,
        25 => MonsterKind::Mob25,
        26 => MonsterKind::Mob26,
        27 => MonsterKind::Mob27,
        28 => MonsterKind::Mob28,
        29 => MonsterKind::Mob29,
        30 => MonsterKind::Mob30,
        31 => MonsterKind::Mob31,
        32 => MonsterKind::Mob32,
        33 => MonsterKind::Mob33,
        34 => MonsterKind::Mob34,
        35 => MonsterKind::Mob35,
        36 => MonsterKind::Mob36,
        37 => MonsterKind::Mob37,
        38 => MonsterKind::Mob38,
        39 => MonsterKind::Mob39,
        40 => MonsterKind::Mob40,
        41 => MonsterKind::Mob41,
        42 => MonsterKind::Mob42,
        43 => MonsterKind::Mob43,
        44 => MonsterKind::Mob44,
        45 => MonsterKind::Mob45,
        46 => MonsterKind::Mob46,
        47 => MonsterKind::Mob47,
        48 => MonsterKind::Mob48,
        49 => MonsterKind::Mob49,
        50 => MonsterKind::Mob50,
        _ => MonsterKind::Mob50,
    }
}

pub fn pick_challenge_named_choices(stage: usize) -> [NamedMonsterConfig; 3] {
    let pool = named_candidate_pool_for_stage(stage);
    let mut rng = thread_rng();

    let picks: Vec<_> = pool.choose_multiple(&mut rng, 3).copied().collect();

    let base_kind = base_monster_kind_for_stage(stage);
    let base_hp = MonsterTemplate::get_base_max_hp(base_kind);
    let base_reward = MonsterTemplate::new(base_kind).reward;

    let hp_multipliers = [1.25, 1.5, 2.0];
    let reward_multipliers = [2usize, 5, 10];

    let mut result = from_fn(|_| NamedMonsterConfig::from_kind(pool[0]));
    for i in 0..3 {
        let kind = picks.get(i).copied().unwrap_or(pool[0]);
        let mut config = NamedMonsterConfig::from_kind(kind);
        config.max_hp = base_hp * hp_multipliers[i];
        config.reward = base_reward * reward_multipliers[i];
        let skill = skill_for_choice(stage, i, &mut rng);
        config.skills = vec![skill.into()];
        result[i] = config;
    }
    result
}
