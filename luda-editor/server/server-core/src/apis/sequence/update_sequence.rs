use super::shared::*;
use crate::documents::*;
use rpc::update_sequence::{Error, Request, Response};

pub async fn update_sequence(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::update_sequence::Result {
    todo!()
}
