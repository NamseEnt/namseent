use super::shared::*;
use crate::documents::*;
use rpc::get_sequence_and_project_shared_data::{Error, Request, Response};

pub async fn get_sequence_and_project_shared_data(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::get_sequence_and_project_shared_data::Result {
    todo!()
}
