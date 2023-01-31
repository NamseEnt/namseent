use super::previous;
use crate::storage::dynamo_db::Document;

#[migration::version(1)]
#[derive(Clone)]
pub struct UserDocument {
    pub id: rpc::Uuid,
    pub name: String,
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

impl UserDocument {
    pub fn migrate(previous: previous::v0::UserDocument) -> Self {
        Self {
            id: previous.id,
            name: format!("anonymous"),
        }
    }
}
