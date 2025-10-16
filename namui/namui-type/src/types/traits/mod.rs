pub trait State: Send + bincode::Encode + bincode::Decode<()> + 'static {}

impl<T: Send + bincode::Encode + bincode::Decode<()> + 'static> State for T {}
