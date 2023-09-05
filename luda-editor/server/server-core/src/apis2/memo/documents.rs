#[document_macro::document]
pub struct MemoDocument {
    #[pk]
    pub sequence_id: rpc::Uuid,
    #[sk]
    pub memo_id: rpc::Uuid,
    pub content: String,
    pub cut_id: rpc::Uuid,
    pub user_id: rpc::Uuid,
    pub user_name: String,
}

impl Into<rpc::data::Memo> for MemoDocument {
    fn into(self) -> rpc::data::Memo {
        rpc::data::Memo {
            id: self.memo_id,
            content: self.content,
            cut_id: self.cut_id,
            user_id: self.user_id,
            user_name: self.user_name,
        }
    }
}
