pub struct History<TState> {
    undo_count: usize,
    initial_state: TState,
    states: Vec<TState>,
}

#[derive(Debug, PartialEq)]
pub enum UndoError {
    NothingToUndo,
}

#[derive(Debug, PartialEq)]
pub enum RedoError {
    NothingToRedo,
}

impl<TState> History<TState> {
    pub fn new(initial_state: TState) -> Self {
        Self {
            undo_count: 0,
            initial_state,
            states: Vec::new(),
        }
    }
    pub fn push(&mut self, state: TState) {
        for _ in 0..(self.undo_count.min(self.states.len())) {
            self.states.pop();
        }
        self.undo_count = 0;
        self.states.push(state);
    }
    pub fn undo(&mut self) -> Result<&TState, UndoError> {
        if self.undo_count < self.states.len() {
            self.undo_count += 1;
            Ok(self.get())
        } else {
            Err(UndoError::NothingToUndo)
        }
    }
    pub fn get(&self) -> &TState {
        if self.states.len() <= self.undo_count {
            &self.initial_state
        } else {
            &self.states[self.states.len() - self.undo_count - 1]
        }
    }
    pub fn redo(&mut self) -> Result<&TState, RedoError> {
        if self.undo_count > 0 {
            self.undo_count -= 1;
            Ok(self.get())
        } else {
            Err(RedoError::NothingToRedo)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn undo_should_works() {
        let mut history = History::<i32>::new(0);
        for i in 1..5 {
            history.push(i);
        }
        assert_eq!(history.undo(), Ok(&3));
        assert_eq!(history.undo(), Ok(&2));

        assert_eq!(history.get(), &2);
    }
    #[test]
    #[wasm_bindgen_test]
    fn push_should_reset_undo() {
        let mut history = History::<i32>::new(0);
        for i in 1..5 {
            history.push(i);
        }
        assert_eq!(history.undo(), Ok(&3));
        assert_eq!(history.undo(), Ok(&2));

        history.push(5);
        assert_eq!(history.get(), &5);

        assert_eq!(history.redo(), Err(RedoError::NothingToRedo));
        assert_eq!(history.get(), &5);

        assert_eq!(history.undo(), Ok(&2));
        assert_eq!(history.get(), &2);
        assert_eq!(history.undo(), Ok(&1));
        assert_eq!(history.get(), &1);
        assert_eq!(history.undo(), Ok(&0));
        assert_eq!(history.get(), &0);
        assert_eq!(history.undo(), Err(UndoError::NothingToUndo));
        assert_eq!(history.get(), &0);
    }
    #[test]
    #[wasm_bindgen_test]
    fn redo_should_works() {
        let mut history = History::<i32>::new(0);
        for i in 1..5 {
            history.push(i);
        }
        assert_eq!(history.undo(), Ok(&3));
        assert_eq!(history.undo(), Ok(&2));

        assert_eq!(history.get(), &2);

        assert_eq!(history.redo(), Ok(&3));
        assert_eq!(history.get(), &3);

        assert_eq!(history.redo(), Ok(&4));
        assert_eq!(history.get(), &4);
    }
}
