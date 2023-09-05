use crate::documents::*;
use rpc::get_user_id::{Error, Request, Response};

pub async fn get_user_id(
    session: Option<SessionDocument>,
    _req: rpc::get_user_id::Request,
) -> rpc::get_user_id::Result {
    match session {
        Some(session) => Ok(rpc::get_user_id::Response {
            user_id: session.user_id,
        }),
        None => Err(rpc::get_user_id::Error::InvalidSession),
    }
}
