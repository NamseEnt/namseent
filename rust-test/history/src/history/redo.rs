use crate::History;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct RedoError;

impl fmt::Display for RedoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fail to redo")
    }
}

impl<TState> History<TState>
where
    TState: Copy,
{
    pub fn redoable_count(&self) -> usize {
        self.states.len() - self.current_index - 1
    }

    pub fn redo(&mut self, count: usize) -> Result<(), RedoError> {
        if self.redoable_count() < count {
            return Err(RedoError);
        }

        self.current_index += count;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate as history;

    #[test]
    fn should_throw_error_if_redo_more_than_redoables() {
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

        assert_eq!(history.undo(1), Result::Ok(()));
        assert_eq!(history.redo(2), Err(history::RedoError));
    }

    #[test]
    fn should_not_throw_error_if_redo_less_than_redoables() {
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

        assert_eq!(history.undo(1), Result::Ok(()));
        assert_eq!(history.redo(1), Result::Ok(()));
    }
}
