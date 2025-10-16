mod our_serde;

pub use our_serde::*;

pub trait State:
    Send + 'static + Serialize + Deserialize + bincode::Encode + bincode::Decode<()>
{
}

impl<T: Send + 'static + Serialize + Deserialize + bincode::Encode + bincode::Decode<()>> State
    for T
{
}
