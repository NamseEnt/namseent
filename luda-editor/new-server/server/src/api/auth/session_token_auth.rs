use super::refresh_session_token_ttl;
use crate::*;
use database::schema::*;
use luda_rpc::auth::session_token_auth::*;

pub async fn session_token_auth(
    Request { session_token }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    if session.logged_in() {
        bail!(Error::AlreadyLoggedIn)
    };

    let Some(doc) = db.get(SessionTokenDocGet { session_token }).await? else {
        bail!(Error::SessionTokenNotExists)
    };

    session.login(doc.user_id);
    refresh_session_token_ttl(db, &doc).await?;

    Ok(Response {})
}
