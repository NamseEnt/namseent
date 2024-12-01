mod verify_jwt;

use super::generate_session_token;
use crate::*;
use database::schema::*;
use luda_rpc::auth::google_auth::*;
use std::sync::OnceLock;
use verify_jwt::*;

pub async fn google_auth(
    Request { jwt }: Request,
    db: &Database,
    session: Session,
) -> Result<Response> {
    if session.logged_in() {
        bail!(Error::AlreadyLoggedIn)
    }

    let jwks_client = {
        static GOOGLE_JWKS_CLIENT: OnceLock<GoogleJwksClient> = OnceLock::new();
        GOOGLE_JWKS_CLIENT.get_or_init(|| {
            GoogleJwksClient::new(
                "857257861263-96dkj0a5mhihgbsh663qi54ko1us7gf9.apps.googleusercontent.com"
                    .to_string(),
            )
        })
    };

    let Claims { sub, name } = jwks_client.verify(jwt).await?;

    // TODO: Pass &sub instead clone it.
    let google_identity = db.get(GoogleIdentityDocGet { sub: sub.clone() }).await?;

    if let Some(google_identity) = google_identity {
        return done(db, session, google_identity.user_id).await;
    }

    let user_id = new_id();

    db.transact::<()>((
        UserDocCreate {
            id: user_id,
            name: &name,
            team_ids: &Default::default(),
        },
        GoogleIdentityDocCreate { sub: &sub, user_id },
    ))
    .await?;

    done(db, session, user_id).await
}

async fn done(db: &Database, session: Session, user_id: u128) -> Result<Response> {
    session.login(user_id);
    let session_token = generate_session_token(db, user_id).await?;
    Ok(Response { session_token })
}
