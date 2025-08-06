use crate::*;
use database::{
    Database,
    schema::{SessionTokenDoc, SessionTokenDocPut},
};
use namui_type::SystemTime;
use std::time::Duration;

const SEVEN_DAYS: Duration = Duration::from_secs(3600 * 24 * 7);

pub async fn generate_session_token(db: &Database, user_id: u128) -> database::Result<u128> {
    let session_token = new_id();
    db.transact::<()>(SessionTokenDocPut {
        session_token,
        user_id,
        expires_at: SystemTime::now() + SEVEN_DAYS,
    })
    .await?;

    Ok(session_token)
}

pub async fn refresh_session_token_ttl(
    db: &Database,
    session_token_doc: &SessionTokenDoc,
) -> database::Result<()> {
    db.transact::<()>(SessionTokenDocPut {
        user_id: session_token_doc.user_id,
        session_token: session_token_doc.session_token,
        expires_at: SystemTime::now() + SEVEN_DAYS,
    })
    .await?;

    Ok(())
}
