use namui_type::serde;

pub type SerErr = serde_json::Error;
pub type Result<T> = std::result::Result<T, SerErr>;
pub fn serialize<T>(value: &T) -> Result<Vec<u8>>
where
    T: serde::Serialize,
{
    serde_json::to_vec(value)
}

pub fn deserialize<T>(bytes: &[u8]) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_slice(bytes)
}
