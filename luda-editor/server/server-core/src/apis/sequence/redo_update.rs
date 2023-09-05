use super::shared::*;
use crate::documents::*;
use rpc::redo_update::{Error, Request, Response};

pub async fn redo_update(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::redo_update::Result {
    todo!()
}
