use super::shared::*;
use crate::documents::*;
use rpc::list_sequence_memos::{Error, Request, Response};

pub async fn list_sequence_memos(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::list_sequence_memos::Result {
    todo!()
}
