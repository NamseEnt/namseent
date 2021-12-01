use crate::History;

impl<TState> History<TState>
where
    TState: Copy,
{
    pub fn get_current_state(&self) -> &TState {
        &self.states[self.current_index]
    }
}

#[cfg(test)]
mod tests {
    use crate as history;

    #[test]
    fn current_state_of_just_created_history_should_be_what_you_pass_on_creating() {
        let state = 1;
        let history = history::new(state);
        let next_state = history.get_current_state();
        assert_eq!(&state, next_state);
    }

    #[test]
    fn commit_should_change_current_state_as_committed_one() {
        #[derive(Copy, Clone)]
        struct State {
            value: i32,
        }
        let state = State { value: 1 };
        let mut history = history::new(state);

        let result = history.commit::<()>(|state| {
            state.value += 1;
            Result::Ok(())
        });
        assert_eq!(result, Result::Ok(()));

        let next_state = history.get_current_state();
        assert_eq!(state.value + 1, next_state.value);
    }

    #[test]
    fn undo_should_change_current_state_as_previous_one() {
        #[derive(Copy, Clone)]
        struct State {
            value: i32,
        }
        let state = State { value: 1 };
        let mut history = history::new(state);

        let commit_result = history.commit::<()>(|state| {
            state.value += 1;
            Result::Ok(())
        });
        assert_eq!(commit_result, Result::Ok(()));

        assert_eq!(history.undo(1).is_ok(), true);

        let next_state = history.get_current_state();
        assert_eq!(state.value, next_state.value);
    }

    #[test]
    fn redo_should_change_current_state_as_next_one() {
        #[derive(Copy, Clone)]
        struct State {
            value: i32,
        }
        let state = State { value: 1 };
        let mut history = history::new(state);

        let commit_result = history.commit::<()>(|state| {
            state.value += 1;
            Result::Ok(())
        });
        assert_eq!(commit_result, Result::Ok(()));

        assert_eq!(history.undo(1), Ok(()));
        assert_eq!(history.redo(1), Ok(()));

        assert_eq!(history.get_current_state().value, 2);
    }
}
