use super::previous;

#[document_macro::document(no_serialize, no_deserialize)]
#[migration::version(1)]
pub struct UserDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub name: String,
}

impl UserDocument {
    pub fn migrate(previous: previous::v0::UserDocument) -> Self {
        Self {
            id: previous.id,
            name: format!("anonymous"),
        }
    }
}
