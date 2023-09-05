use crate::documents::*;

pub async fn validate_session(
    session: Option<SessionDocument>,
    _req: rpc::validate_session::Request,
) -> rpc::validate_session::Result {
    match session {
        Some(_) => Ok(rpc::validate_session::Response {}),
        None => Err(rpc::validate_session::Error::InvalidSession),
    }
}
