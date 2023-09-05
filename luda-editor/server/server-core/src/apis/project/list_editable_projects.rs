use super::shared::*;
use crate::documents::*;
use rpc::list_editable_projects::{Error, Request, Response};

pub async fn list_editable_projects(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::list_editable_projects::Result {
    todo!()
}
