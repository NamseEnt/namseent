#[document_macro::document]
pub struct SessionDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub user_id: rpc::Uuid,
}
