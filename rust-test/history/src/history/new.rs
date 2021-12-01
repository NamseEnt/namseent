use crate::History;

pub fn new<TState>(state: TState) -> History<TState>
where
    TState: Copy,
{
    History {
        current_index: 0,
        states: vec![state],
    }
}
pub fn test() {}

#[cfg(test)]
mod tests {}
