use super::Upgrade;
use crate::game_state::{GameState, upgrade::UpgradeWithId};

pub fn generate_boss_reward_upgrade(game_state: &mut GameState) -> Upgrade {
    let mut raw = game_state.raw_core.state().clone();
    let upgrade = UpgradeWithId::from_core_state(td_core::generate_boss_reward_option(&mut raw))
        .expect("raw boss reward must convert to a headed upgrade")
        .upgrade;
    game_state
        .restore_raw_core_projection(raw)
        .expect("raw boss reward generation must be restorable in headed adapter");
    upgrade
}
