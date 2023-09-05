use super::shared::*;
use crate::documents::*;
use rpc::list_project_sequences::{Error, Request, Response};

pub async fn list_project_sequences(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::list_project_sequences::Result {
    todo!()
}
