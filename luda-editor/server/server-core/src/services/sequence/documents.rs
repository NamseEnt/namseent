use anyhow::Result;
use std::io::read_to_string;

#[document_macro::document]
pub struct SequenceDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub project_id: rpc::Uuid,
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub json_brotli: Vec<u8>,
    /// Timestamp in nano seconds.
    pub last_modified: Option<i64>,
}

impl SequenceDocument {
    pub fn sequence_json_string(&self) -> Result<String> {
        let decompressor =
            brotli::Decompressor::new(self.json_brotli.as_slice(), self.json_brotli.len());
        let sequence_json_string = read_to_string(decompressor)?;
        Ok(sequence_json_string)
    }

    pub fn sequence<TSequence: serde::de::DeserializeOwned>(&self) -> Result<TSequence> {
        let decompressor =
            brotli::Decompressor::new(self.json_brotli.as_slice(), self.json_brotli.len());
        let sequence = serde_json::from_reader(decompressor)?;
        Ok(sequence)
    }
}

#[document_macro::document]
pub struct ProjectSequenceDocument {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub sequence_id: rpc::Uuid,
}
