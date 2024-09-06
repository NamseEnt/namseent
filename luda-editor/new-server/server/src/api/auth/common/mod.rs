use database::{
    schema::{SessionTokenDoc, SessionTokenDocPut},
    Database,
};
use rkyv::Archived;
use std::time::Duration;

const SEVEN_DAYS: Duration = Duration::from_secs(3600 * 24 * 7);

pub async fn generate_session_token(db: &Database, user_id: &str) -> database::Result<String> {
    let session_token = randum::rand();
    db.transact::<()>(SessionTokenDocPut {
        session_token: &session_token,
        user_id,
        ttl: Some(SEVEN_DAYS),
    })
    .await?;

    Ok(session_token)
}

pub async fn refresh_session_token_ttl(
    db: &Database,
    session_token_doc: &Archived<SessionTokenDoc>,
) -> database::Result<()> {
    db.transact::<()>(SessionTokenDocPut {
        user_id: &session_token_doc.user_id,
        session_token: &session_token_doc.session_token,
        ttl: Some(SEVEN_DAYS),
    })
    .await?;

    Ok(())
}
