use crate::History;

impl<TState> History<TState>
where
    TState: Copy,
{
    pub fn commit<TError>(
        &mut self,
        update_state: fn(&mut TState) -> Result<(), TError>,
    ) -> Result<(), TError> {
        let current_state = self.get_current_state();
        let mut next_state = current_state.clone();
        let result = update_state(&mut next_state);

        if result.is_err() {
            return Result::Err(result.unwrap_err());
        }

        self.states.push(next_state);
        self.current_index += 1;
        Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate as history;

    #[test]
    fn should_be_failed_if_update_state_is_failed() {
        #[derive(Copy, Clone)]
        struct State {
            value: i32,
        }

        let state = State { value: 1 };
        let mut history = history::new(state);

        let result = history.commit::<String>(|state| {
            state.value = 2;
            Result::Err(String::from("failed"))
        });
        assert_eq!(result, Result::Err(String::from("failed")));
    }

    #[test]
    fn should_be_successful_if_update_state_is_successful() {
        #[derive(Copy, Clone)]
        struct State {
            value: i32,
        }

        let state = State { value: 1 };
        let mut history = history::new(state);

        let result = history.commit::<String>(|state| {
            state.value = 2;
            Result::Ok(())
        });
        assert_eq!(result, Result::Ok(()));
    }
}
