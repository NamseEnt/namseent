#[document_macro::document]
#[derive(Clone)]
pub struct UserDocument {
    pub id: rpc::Uuid,
    // TODO: Add User Name
}
