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

impl From<MemoDocument> for rpc::data::Memo {
    fn from(val: MemoDocument) -> Self {
        rpc::data::Memo {
            id: val.memo_id,
            content: val.content,
            cut_id: val.cut_id,
            user_id: val.user_id,
            user_name: val.user_name,
        }
    }
}
