use crate::documents::*;
use rpc::validate_session::{Error, Request, Response};

pub async fn validate_session(
    session: Option<SessionDocument>,
    _req: Request,
) -> rpc::validate_session::Result {
    match session {
        Some(_) => Ok(Response {}),
        None => Err(Error::InvalidSession),
    }
}
