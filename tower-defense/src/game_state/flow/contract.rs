use crate::game_state::{
    GameState,
    contract::{Contract, ContractEvent},
    effect::run_effect,
    flow::GameFlow,
    *,
};
use namui::*;
use std::collections::VecDeque;

const STANDBY_DURATION: Duration = Duration::from_millis(250);
const ACTIVE_DURATION: Duration = Duration::from_millis(750);

#[derive(Clone, Debug, State)]
pub struct ContractFlow {
    pub contract_event_queue: VecDeque<ContractEvent>,
    pub state: ContractFlowState,
}
impl ContractFlow {
    /// Creates a new ContractFlow with the given events.
    pub fn new(events: Vec<ContractEvent>) -> Self {
        ContractFlow {
            contract_event_queue: VecDeque::from(events),
            state: Default::default(),
        }
    }

    /// Creates an empty ContractFlow.
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    /// Steps all contracts to the next stage (Command - modifies state only).
    /// This follows Command-Query Separation by only modifying state.
    /// Events are stored internally in each contract.
    pub fn step_all_contracts(contracts: &mut [Contract]) {
        for contract in contracts.iter_mut() {
            contract.step_stage();
        }
    }

    /// Drains all pending events from all contracts (Query - returns data only).
    /// This follows Command-Query Separation by only reading data.
    /// Call this after step_all_contracts to collect generated events.
    pub fn drain_all_events(contracts: &mut [Contract]) -> Vec<ContractEvent> {
        contracts
            .iter_mut()
            .flat_map(|contract| contract.drain_events())
            .collect()
    }
}

#[derive(Clone, Debug, Default, State)]
pub enum ContractFlowState {
    #[default]
    Unset,
    Standby {
        /// game_now
        end_at: Instant,
        event: ContractEvent,
    },
    Active {
        /// game_now
        end_at: Instant,
        event: ContractEvent,
    },
}

pub fn update_contract_flow(game_state: &mut GameState) {
    let game_now = game_state.now();
    let GameFlow::Contract(contract_flow) = &mut game_state.flow else {
        return;
    };

    match &mut contract_flow.state {
        ContractFlowState::Unset => {
            let Some(contract_event) = contract_flow.contract_event_queue.pop_front() else {
                game_state.goto_selecting_tower();
                return;
            };
            contract_flow.state = ContractFlowState::Standby {
                end_at: game_now + STANDBY_DURATION,
                event: contract_event,
            };
        }
        ContractFlowState::Standby { end_at, event } => {
            if game_now < *end_at {
                return;
            }
            let effect = event.effect.clone();
            contract_flow.state = ContractFlowState::Active {
                end_at: game_now + ACTIVE_DURATION,
                event: event.clone(),
            };
            run_effect(game_state, &effect);
        }
        ContractFlowState::Active { end_at, .. } => {
            if game_now < *end_at {
                return;
            }
            contract_flow.state = ContractFlowState::Unset;
        }
    }
}
