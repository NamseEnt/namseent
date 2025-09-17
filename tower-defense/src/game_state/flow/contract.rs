use crate::game_state::{
    GameState,
    contract::{Contract, ContractEvent},
    effect::run_effect,
    flow::GameFlow,
};
use namui::*;

const STANDBY_DURATION: Duration = Duration::from_millis(250);
const ACTIVE_DURATION: Duration = Duration::from_millis(750);

#[derive(Clone, Debug)]
pub struct ContractFlow {
    pub contract_event_queue: Vec<ContractEvent>,
    state: ContractFlowState,
}
impl ContractFlow {
    /// Creates a new ContractFlow with the given events.
    pub fn new(events: Vec<ContractEvent>) -> Self {
        ContractFlow {
            contract_event_queue: events,
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

#[derive(Clone, Debug, Default)]
pub enum ContractFlowState {
    #[default]
    Unset,
    Standby {
        /// game_now
        end_at: Instant,
    },
    Active {
        /// game_now
        end_at: Instant,
    },
}

pub fn update_contract_flow(game_state: &mut GameState) {
    let game_now = game_state.now();
    let GameFlow::Contract(contract_flow) = &mut game_state.flow else {
        return;
    };

    match &mut contract_flow.state {
        ContractFlowState::Unset => {
            if contract_flow.contract_event_queue.is_empty() {
                game_state.goto_selecting_tower();
                return;
            }
            contract_flow.state = ContractFlowState::Standby {
                end_at: game_now + STANDBY_DURATION,
            };
        }
        ContractFlowState::Standby { end_at, .. } => {
            if game_now < *end_at {
                return;
            }
            if let Some(contract_event) = contract_flow.contract_event_queue.pop() {
                contract_flow.state = ContractFlowState::Active {
                    end_at: game_now + ACTIVE_DURATION,
                };

                run_effect(game_state, &contract_event.effect);
                return;
            }

            println!("Contract event queue empty");
            contract_flow.state = ContractFlowState::Unset;
        }
        ContractFlowState::Active { end_at, .. } => {
            if game_now < *end_at {
                return;
            }
            contract_flow.state = ContractFlowState::Unset;
        }
    }
}
