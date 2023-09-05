use super::shared::*;
use crate::documents::*;
use rpc::update_server_project_shared_data::{Error, Request, Response};

pub async fn update_server_project_shared_data(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::update_server_project_shared_data::Result {
    todo!()
}
