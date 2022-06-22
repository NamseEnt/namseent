mod history;
use downcast_rs::{impl_downcast, Downcast};
use history::History;
use std::error::Error;

pub trait Act<TState>: Downcast {
    fn act(&self, state: &TState) -> Result<TState, Box<dyn Error>>;
}
impl_downcast!(Act<TState>);

pub struct HistorySystem<TState> {
    action: Option<Box<dyn Act<TState>>>,
    history: History<TState>,
}

#[derive(Debug)]
pub enum ActError {
    ActionNotExists,
    ActionFailToRun(Box<dyn Error>),
}

impl<TState: 'static> HistorySystem<TState> {
    pub fn new(initial_state: TState) -> Self {
        HistorySystem {
            action: None,
            history: History::new(initial_state),
        }
    }
    pub fn get_state(&self) -> &TState {
        self.history.get()
    }
    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }
    pub fn set_action(&mut self, action: impl Act<TState>) {
        // let boxed: Box<dyn Act<TState>> = Box::new(action);
        self.action = Some(Box::new(action));
    }
    pub fn with_action<TAction: Act<TState>>(&mut self, callback: impl FnOnce(&mut TAction)) {
        if let Some(action) = &mut self.action {
            if let Some(action) = action.downcast_mut::<TAction>() {
                callback(action);
            }
        }
    }
    pub fn undo(&mut self) -> Option<()> {
        self.history.undo()
    }
    pub fn redo(&mut self) -> Option<()> {
        self.history.redo()
    }
    pub fn act(&mut self) -> Result<&TState, ActError> {
        if self.action.is_none() {
            return Err(ActError::ActionNotExists);
        }
        let action = self.action.take().unwrap();

        let state = self.get_state();
        let result = action.act(state);

        match result {
            Ok(state) => {
                self.history.push(state);
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
        history_system.with_action(|action: &mut RightAction| {
            action.value = 2;
            is_called = true;
        });

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
        history_system.with_action(|action: &mut WrongAction| {
            action.value = 2;
            is_called = true;
        });

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

        assert_eq!(history_system.act().unwrap(), &3);
    }
}
