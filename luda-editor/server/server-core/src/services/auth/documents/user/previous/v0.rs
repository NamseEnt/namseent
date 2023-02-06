use crate::storage::dynamo_db::Document;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDocument {
    pub id: rpc::Uuid,
    // TODO: Add User Name
}

impl Document for UserDocument {
    fn partition_key_prefix() -> &'static str {
        "user"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.id.to_string()
    }

    fn sort_key(&self) -> Option<String> {
        None
    }
}
