use super::*;

#[document_macro::document(1)]
pub struct UserDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub name: String,
}

impl UserDocument {
    pub fn migrate(previous: v0::UserDocument) -> Self {
        Self {
            id: previous.id,
            name: "anonymous".to_string(),
        }
    }
}
