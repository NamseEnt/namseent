use super::shared::*;
use crate::documents::*;
use rpc::create_project::{Error, Request, Response};

pub async fn create_project(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::create_project::Result {
    todo!()
}
