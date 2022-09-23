use crate::storage::dynamo_db::Document;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SequenceDocument {
    pub id: rpc::Uuid,
    pub project_id: rpc::Uuid,
    pub name: String,
    pub json: String,
    /// Timestamp in nano seconds.
    pub last_modified: Option<i64>,
}

impl Document for SequenceDocument {
    fn partition_key_prefix() -> &'static str {
        "sequence"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.id.to_string()
    }

    fn sort_key(&self) -> Option<String> {
        None
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectSequenceDocument {
    pub project_id: rpc::Uuid,
    pub sequence_id: rpc::Uuid,
}

impl Document for ProjectSequenceDocument {
    fn partition_key_prefix() -> &'static str {
        "project_sequence"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.project_id.to_string()
    }

    fn sort_key(&self) -> Option<String> {
        Some(self.sequence_id.to_string())
    }
}
