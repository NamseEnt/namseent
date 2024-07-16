use super::refresh_session_token_ttl;
use crate::*;
use database::schema::*;
use luda_rpc::auth::session_token_auth::*;

pub async fn session_token_auth(
    ArchivedRequest { session_token }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    if session.logged_in().await {
        bail!(Error::AlreadyLoggedIn)
    };

    let Some(doc) = db.get(SessionTokenDocGet { session_token }).await? else {
        bail!(Error::SessionTokenNotExist)
    };

    session.login(&doc.user_id).await;
    refresh_session_token_ttl(db, &doc).await?;

    Ok(Response {})
}
