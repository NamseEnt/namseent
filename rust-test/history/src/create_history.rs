use crate::History::History;

impl<TState> History<TState>
where
    TState: Copy,
{
    pub fn create_history(state: TState) -> History<TState> {
        History {
            current_index: 0,
            states: vec![state],
        }
    }
}

#[cfg(test)]
mod tests {}
