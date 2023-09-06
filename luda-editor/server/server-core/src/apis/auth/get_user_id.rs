use crate::documents::*;
use rpc::get_user_id::{Error, Request, Response};

pub async fn get_user_id(
    session: Option<SessionDocument>,
    _req: Request,
) -> rpc::get_user_id::Result {
    match session {
        Some(session) => Ok(Response {
            user_id: session.user_id,
        }),
        None => Err(Error::InvalidSession),
    }
}
