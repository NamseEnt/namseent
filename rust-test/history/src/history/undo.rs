use crate::History;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct UndoError;

impl fmt::Display for UndoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fail to undo")
    }
}

impl<TState> History<TState>
where
    TState: Copy,
{
    pub fn undoable_count(&self) -> usize {
        self.current_index
    }

    pub fn undo(&mut self, count: usize) -> Result<(), UndoError> {
        if self.undoable_count() < count {
            return Err(UndoError);
        }

        self.current_index -= count;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate as history;

    #[test]
    fn should_throw_error_if_undo_more_than_undoables() {
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

        let undo_result = history.undo(2);
        assert_eq!(undo_result, Err(history::UndoError));
    }

    #[test]
    fn should_not_throw_error_if_undo_less_than_undoables() {
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

        assert_eq!(history.undo(1), Ok(()));
    }
}
