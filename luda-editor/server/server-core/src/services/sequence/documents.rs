#[document_macro::document]
pub struct SequenceDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub project_id: rpc::Uuid,
    pub name: String,
    pub json: String,
    /// Timestamp in nano seconds.
    pub last_modified: Option<i64>,
}

#[document_macro::document]
pub struct ProjectSequenceDocument {
    #[pk]
    pub project_id: rpc::Uuid,
    #[sk]
    pub sequence_id: rpc::Uuid,
}
