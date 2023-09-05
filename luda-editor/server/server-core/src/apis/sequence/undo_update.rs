use super::shared::*;
use crate::documents::*;
use rpc::undo_update::{Error, Request, Response};

pub async fn undo_update(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::undo_update::Result {
    todo!()
}
