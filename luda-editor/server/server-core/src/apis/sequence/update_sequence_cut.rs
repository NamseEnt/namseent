use super::shared::*;
use crate::documents::*;
use rpc::update_sequence_cut::{Error, Request, Response};

pub async fn update_sequence_cut(
    session: Option<SessionDocument>,
    Request {}: Request,
) -> rpc::update_sequence_cut::Result {
    todo!()
}
