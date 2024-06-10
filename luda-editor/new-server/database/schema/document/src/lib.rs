pub use anyhow::Result;

pub trait Document {
    fn name() -> &'static str;
    fn from_bytes(bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized;
    fn to_bytes(&self) -> Result<Vec<u8>>;
}
