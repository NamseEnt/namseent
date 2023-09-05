use super::shared::*;
use crate::documents::*;
use rpc::create_sequence::{Error, Request, Response};

pub async fn create_sequence(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::create_sequence::Result {
    todo!()
}
