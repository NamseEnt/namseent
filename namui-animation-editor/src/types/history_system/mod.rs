#![allow(dead_code)]

mod history;
use downcast_rs::{impl_downcast, Downcast};
use history::History;
use std::{error::Error, sync::Arc};

pub trait Act<TState>: Downcast {
    fn act(&self, state: &TState) -> Result<TState, Box<dyn Error>>;
}
impl_downcast!(Act<TState>);

pub struct HistorySystem<TState> {
    action: Option<Box<dyn Act<TState>>>,
    history: History<Arc<TState>>,
}

#[derive(Debug)]
pub enum ActError {
    ActionNotExists,
    ActionFailToRun(Box<dyn Error>),
}

#[derive(Debug, PartialEq)]
pub enum UpdateActionError {
    NoAction,
    WrongActionType,
}

impl<TState: 'static> HistorySystem<TState> {
    pub fn new(initial_state: TState) -> Self {
        HistorySystem {
            action: None,
            history: History::new(Arc::new(initial_state)),
        }
    }
    pub fn get_state(&self) -> Arc<TState> {
        self.history.get().clone()
    }
    pub fn get_preview(&self) -> Arc<TState> {
        if let Some(action) = &self.action {
            let state = self.get_state();
            if let Ok(preview) = action.act(&state) {
                return Arc::new(preview);
            }
        }
        self.get_state()
    }
    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }
    pub fn set_action(&mut self, action: impl Act<TState>) {
        self.action = Some(Box::new(action));
    }
    pub fn update_action<TAction: Act<TState>>(
        &mut self,
        update: impl FnOnce(&mut TAction),
    ) -> Result<(), UpdateActionError> {
        if let Some(action) = &mut self.action {
            if let Some(action) = action.downcast_mut::<TAction>() {
                update(action);
                Ok(())
            } else {
                Err(UpdateActionError::WrongActionType)
            }
        } else {
            Err(UpdateActionError::NoAction)
        }
    }
    pub fn undo(&mut self) -> Option<()> {
        self.history.undo()
    }
    pub fn redo(&mut self) -> Option<()> {
        self.history.redo()
    }
    pub fn act(&mut self) -> Result<Arc<TState>, ActError> {
        if self.action.is_none() {
            return Err(ActError::ActionNotExists);
        }
        let action = self.action.take().unwrap();

        let state = self.get_state();
        let result = action.act(&state);

        match result {
            Ok(state) => {
                self.history.push(Arc::new(state));
                Ok(self.get_state())
            }
            Err(error) => Err(ActError::ActionFailToRun(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn with_action_should_be_called_on_right_type() {
        let mut history_system = HistorySystem::new(0);

        struct RightAction {
            value: i32,
        }

        impl Act<i32> for RightAction {
            fn act(&self, state: &i32) -> Result<i32, Box<dyn Error>> {
                Ok(self.value + state)
            }
        }

        history_system.set_action(RightAction { value: 1 });

        let mut is_called = false;
        history_system
            .update_action(|action: &mut RightAction| {
                action.value = 2;
                is_called = true;
            })
            .unwrap();

        assert!(is_called);
    }

    #[test]
    #[wasm_bindgen_test]
    fn with_action_should_not_be_called_on_wrong_type() {
        let mut history_system = HistorySystem::new(0);

        struct RightAction {
            _value: i32,
        }

        impl Act<i32> for RightAction {
            fn act(&self, state: &i32) -> Result<i32, Box<dyn Error>> {
                Ok(self._value + state)
            }
        }

        history_system.set_action(RightAction { _value: 1 });

        struct WrongAction {
            value: i32,
        }

        impl Act<i32> for WrongAction {
            fn act(&self, state: &i32) -> Result<i32, Box<dyn Error>> {
                Ok(self.value + state)
            }
        }

        let mut is_called = false;
        assert_eq!(
            history_system.update_action(|action: &mut WrongAction| {
                action.value = 2;
                is_called = true;
            }),
            Err(UpdateActionError::WrongActionType)
        );

        assert_eq!(is_called, false);
    }

    #[test]
    #[wasm_bindgen_test]
    fn act_should_work() {
        let mut history_system = HistorySystem::new(1);

        struct RightAction {
            value: i32,
        }

        impl Act<i32> for RightAction {
            fn act(&self, state: &i32) -> Result<i32, Box<dyn Error>> {
                Ok(self.value + state)
            }
        }

        history_system.set_action(RightAction { value: 2 });

        assert_eq!(history_system.act().unwrap(), Arc::new(3));
    }
}
