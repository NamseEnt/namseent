mod user;

use crate::storage::dynamo_db::Document;
pub use user::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IdentityDocument {
    pub id: String,
    pub user_id: rpc::Uuid,
}

impl Document for IdentityDocument {
    fn partition_key_prefix() -> &'static str {
        "identity"
    }

    fn partition_key_without_prefix(&self) -> String {
        self.id.clone()
    }

    fn sort_key(&self) -> Option<String> {
        None
    }
}
