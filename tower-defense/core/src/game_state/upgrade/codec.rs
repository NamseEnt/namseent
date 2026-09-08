pub use super::codec_impl::UpgradeCodecError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn encode_upgrade_entry<S>(
    entry: &super::UpgradeEntry,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    entry.to_raw().serialize(serializer)
}

pub fn decode_upgrade_entry<'de, D>(deserializer: D) -> Result<super::UpgradeEntry, D::Error>
where
    D: Deserializer<'de>,
{
    let wire = super::UpgradeWireEntry::deserialize(deserializer)?;
    super::UpgradeEntry::from_raw(wire)
        .map_err(|_| serde::de::Error::custom("invalid upgrade codec entry"))
}

pub fn encode_upgrade_collection<S>(
    collection: &super::UpgradeCollection,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeStruct;
    let mut state = serializer.serialize_struct("UpgradeCollection", 2)?;
    state.serialize_field("upgrades", &collection.upgrades)?;
    state.serialize_field("revision", &collection.revision)?;
    state.end()
}

pub fn decode_upgrade_collection<'de, D>(
    deserializer: D,
) -> Result<super::UpgradeCollection, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct UpgradeCollectionWire {
        upgrades: Vec<super::UpgradeEntry>,
        revision: usize,
    }
    let state = UpgradeCollectionWire::deserialize(deserializer)?;
    Ok(super::UpgradeCollection::from_entries(
        state.upgrades,
        state.revision,
    ))
}
