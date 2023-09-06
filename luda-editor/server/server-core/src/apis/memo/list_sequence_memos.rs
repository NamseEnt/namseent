use crate::documents::*;
use rpc::list_sequence_memos::{Error, Request, Response};

pub async fn list_sequence_memos(
    _session: Option<SessionDocument>,
    Request { sequence_id }: Request,
) -> rpc::list_sequence_memos::Result {
    let memo_query = MemoDocumentQuery {
        pk_sequence_id: sequence_id,
        last_sk: None, // TODO
    }
    .run()
    .await
    .map_err(|error| Error::Unknown(error.to_string()))?;

    let memos = memo_query
        .documents
        .into_iter()
        .map(|memo_document| memo_document.into())
        .collect();

    Ok(Response { memos })
}
