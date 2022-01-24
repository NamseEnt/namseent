pub struct History<TState> {
    undo_count: usize,
    initial_state: TState,
    states: Vec<TState>,
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
    pub fn undo(&mut self) {
        self.undo_count = (self.undo_count + 1).min(self.states.len());
    }
    pub fn get(&self) -> &TState {
        if self.states.len() <= self.undo_count {
            &self.initial_state
        } else {
            &self.states[self.states.len() - self.undo_count - 1]
        }
    }
    pub fn redo(&mut self) {
        if self.undo_count > 0 {
            self.undo_count -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::History;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn undo_should_works() {
        let mut history = History::<i32>::new(0);
        for i in 1..5 {
            history.push(i);
        }
        history.undo();
        history.undo();

        assert_eq!(history.get(), &2);
    }
    #[test]
    #[wasm_bindgen_test]
    fn push_should_reset_undo() {
        let mut history = History::<i32>::new(0);
        for i in 1..5 {
            history.push(i);
        }
        history.undo();
        history.undo();

        history.push(5);
        assert_eq!(history.get(), &5);

        history.redo();
        assert_eq!(history.get(), &5);
        history.redo();
        assert_eq!(history.get(), &5);

        history.undo();
        assert_eq!(history.get(), &2);
        history.undo();
        assert_eq!(history.get(), &1);
        history.undo();
        assert_eq!(history.get(), &0);
        history.undo();
        assert_eq!(history.get(), &0);
    }
    #[test]
    #[wasm_bindgen_test]
    fn redo_should_works() {
        let mut history = History::<i32>::new(0);
        for i in 1..5 {
            history.push(i);
        }
        history.undo();
        history.undo();

        assert_eq!(history.get(), &2);

        history.redo();
        assert_eq!(history.get(), &3);

        history.redo();
        assert_eq!(history.get(), &4);
    }
}
