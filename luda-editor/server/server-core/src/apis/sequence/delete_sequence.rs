use super::shared::*;
use crate::documents::*;
use rpc::delete_sequence::{Error, Request, Response};

pub async fn delete_sequence(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::delete_sequence::Result {
    todo!()
}
