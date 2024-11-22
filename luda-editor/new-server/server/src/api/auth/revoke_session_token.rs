use crate::*;
use database::schema::*;
use luda_rpc::auth::revoke_session_token::*;

pub async fn revoke_session_token(
    &ArchivedRequest { session_token }: &ArchivedRequest,
    db: &Database,
    session: Session,
) -> Result<Response> {
    let user_id = session.user_id().ok_or(Error::Unauthorized)?;

    let Some(doc) = db.get(SessionTokenDocGet { session_token }).await? else {
        return Ok(Response {});
    };

    if doc.user_id != user_id {
        bail!(Error::Unauthorized)
    }

    db.transact::<()>(SessionTokenDocDelete { session_token })
        .await?;

    Ok(Response {})
}
