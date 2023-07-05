pub use migration_macro::version;

pub trait Migration {
    fn migration_version() -> u64;
    fn deserialize(bytes: &[u8], version: u64) -> Result<Self, bincode::Error>
    where
        Self: Sized;
}
