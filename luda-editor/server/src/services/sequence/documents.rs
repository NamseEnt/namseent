use crate::storage::dynamo_db::Document;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SequenceDocument {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub yrs_update_v2_base64: Option<String>,
    /// Timestamp in nano seconds.
    pub last_modified: Option<i64>,
}

impl Document for SequenceDocument {
    fn partition_key_prefix() -> &'static str {
        "sequence"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.id.clone()
    }

    fn sort_key(&self) -> Option<&str> {
        None
    }
}

impl SequenceDocument {
    pub fn e_tag(&self) -> Option<String> {
        self.last_modified
            .map(|last_modified| last_modified.to_string())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectSequenceDocument {
    pub project_id: String,
    pub sequence_id: String,
}

impl Document for ProjectSequenceDocument {
    fn partition_key_prefix() -> &'static str {
        "project_sequence"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.project_id.clone()
    }

    fn sort_key(&self) -> Option<&str> {
        Some(&self.sequence_id)
    }
}
