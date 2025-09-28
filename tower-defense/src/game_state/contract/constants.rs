//! Shared numeric balance constants & range tables for contract generation.
//! Centralizing these makes balancing and tweaking easier.

/// Stage duration weighted distribution (for stages 2..=10).
/// Interpreted by `WeightedIndex` in the same order.
pub const STAGE_DURATION_WEIGHTS: [u32; 9] = [8, 9, 5, 4, 3, 2, 1, 1, 1];

// Reward side ranges --------------------------------------------------------
pub const REWARD_HEAL_ON_SIGN: [(f32, f32); 4] = [(10.0, 15.0), (20.0, 25.0), (30.0, 35.0), (40.0, 46.0)];
pub const REWARD_EARN_GOLD: [(f32, f32); 4] = [(225.0, 251.0), (500.0, 551.0), (1000.0, 1251.0), (2000.0, 2501.0)];
pub const REWARD_INCREASE_TOWER_DAMAGE: [(f32, f32); 4] = [
    (1.01, 1.06),
    (1.05, 1.11),
    (1.10, 1.26),
    (1.25, 1.76),
];
pub const REWARD_INCREASE_TOWER_RANGE: [(f32, f32); 4] = [
    (1.01, 1.06),
    (1.05, 1.11),
    (1.10, 1.26),
    (1.25, 1.51),
];
pub const REWARD_DECREASE_INCOMING_DAMAGE: [(f32, f32); 4] = [
    (0.9, 0.95),
    (0.8, 0.9),
    (0.65, 0.8),
    (0.5, 0.65),
];
pub const REWARD_INCREASE_GOLD_GAIN: [(f32, f32); 4] = [
    (1.25, 1.35),
    (1.35, 1.5),
    (1.5, 1.75),
    (1.75, 2.25),
];

// Risk side ranges ----------------------------------------------------------
pub const RISK_LOSE_HEALTH: [(f32, f32); 4] = [(5.0, 10.0), (10.0, 15.0), (15.0, 20.0), (20.0, 26.0)];
pub const RISK_LOSE_GOLD: [(f32, f32); 4] = [(125.0, 151.0), (250.0, 301.0), (500.0, 751.0), (1000.0, 1501.0)];
pub const RISK_STAGE_LOSE_GOLD: [(f32, f32); 4] = [(10.0, 20.0), (20.0, 30.0), (30.0, 40.0), (40.0, 50.0)];
pub const RISK_DECREASE_TOWER_DAMAGE: (f32, f32) = (0.75, 0.95); // single range (not rarity scaled)
pub const RISK_INCREASE_INCOMING_DAMAGE: (f32, f32) = (1.1, 2.0);
pub const RISK_DECREASE_GOLD_GAIN_PERCENT: (f32, f32) = (0.1, 0.5); // reduction percentage
pub const RISK_REROLL_HEALTH_COST: (u32, u32) = (1, 5);
pub const RISK_DECREASE_ENEMY_HEALTH_PERCENT: f32 = 10.0; // currently fixed
