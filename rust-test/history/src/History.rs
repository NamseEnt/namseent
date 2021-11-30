pub struct History<TState>
where
    TState: Copy,
{
    pub current_index: usize,
    pub states: Vec<TState>,
}
