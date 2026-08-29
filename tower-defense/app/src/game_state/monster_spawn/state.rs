use crate::game_state::*;
use std::collections::VecDeque;

#[derive(State, Clone)]
pub struct MonsterSpawnState {
    pub monster_queue: VecDeque<Monster>,
    pub next_spawn_tick: Option<SimTick>,
    pub spawn_interval: SimTickSpan,
}

impl MonsterSpawnState {
    pub(crate) fn to_core_state(&self) -> td_core::MonsterSpawnState {
        td_core::MonsterSpawnState {
            monster_queue: self
                .monster_queue
                .iter()
                .map(crate::game_state::Monster::to_core_state)
                .collect(),
            next_spawn_tick: self.next_spawn_tick.map(|tick| tick.ticks()),
            spawn_interval_ticks: self.spawn_interval.ticks(),
        }
    }

    pub(crate) fn from_core_state(
        state: td_core::MonsterSpawnState,
        presentation_source: Option<&Self>,
    ) -> Option<Self> {
        let mut ids = Vec::with_capacity(state.monster_queue.len());
        let monster_queue = state
            .monster_queue
            .into_iter()
            .map(|snapshot| {
                let mut monster = crate::game_state::Monster::from_core_state(snapshot)?;
                if ids.contains(&monster.id()) {
                    return None;
                }
                ids.push(monster.id());
                if let Some(previous) = presentation_source.and_then(|source| {
                    source
                        .monster_queue
                        .iter()
                        .find(|candidate| candidate.id() == monster.id())
                }) {
                    monster.restore_presentation_state(previous.presentation_state());
                }
                Some(monster)
            })
            .collect::<Option<VecDeque<_>>>()?;
        Some(Self {
            monster_queue,
            next_spawn_tick: state.next_spawn_tick.map(SimTick::from_ticks),
            spawn_interval: SimTickSpan::from_ticks(state.spawn_interval_ticks),
        })
    }

    #[cfg(test)]
    pub fn idle() -> Self {
        Self {
            monster_queue: VecDeque::new(),
            next_spawn_tick: None,
            spawn_interval: SimTickSpan::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_state_round_trip_preserves_spawn_timing() {
        let state = MonsterSpawnState {
            monster_queue: VecDeque::new(),
            next_spawn_tick: Some(SimTick::from_ticks(12)),
            spawn_interval: SimTickSpan::from_ticks(7),
        };

        let raw = state.to_core_state();
        let restored = MonsterSpawnState::from_core_state(raw.clone(), None)
            .expect("valid monster spawn state");

        assert_eq!(restored.to_core_state(), raw);
    }

    #[test]
    fn raw_state_round_trip_preserves_queued_monster() {
        let game_state = crate::game_state::create_game_state_with_seed(23);
        let template = crate::game_state::MonsterTemplate::new(
            crate::game_state::MonsterKind::Mob01,
            &game_state.config,
        );
        let monster = crate::game_state::Monster::new_with_id(
            &template,
            game_state.route.clone(),
            SimTick::ZERO,
            &crate::RatioProduct::one(),
            crate::MonsterId::from_raw(77),
        );
        let state = MonsterSpawnState {
            monster_queue: VecDeque::from([monster]),
            next_spawn_tick: Some(SimTick::from_ticks(4)),
            spawn_interval: SimTickSpan::from_ticks(9),
        };

        let raw = state.to_core_state();
        let restored = MonsterSpawnState::from_core_state(raw.clone(), Some(&state))
            .expect("valid monster spawn state");

        assert_eq!(restored.to_core_state(), raw);
        assert_eq!(
            restored.monster_queue.front().unwrap().id(),
            crate::MonsterId::from_raw(77)
        );
    }
}
