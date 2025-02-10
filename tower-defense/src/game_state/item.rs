use crate::card::{Rank, Suit};
use namui::*;

#[derive(Debug, Clone)]
pub enum Item {
    Heal {
        amount: f32,
    },
    TowerDamagePlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerDamageMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerSpeedPlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerSpeedMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    TowerRangePlus {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    WeakenMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    SlowdownMultiply {
        amount: f32,
        duration: Duration,
        radius: f32,
    },
    Attack {
        rank: Rank,
        suit: Suit,
        damage: f32,
        radius: f32,
    },
}
