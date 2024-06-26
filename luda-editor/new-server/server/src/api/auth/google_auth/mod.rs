mod verify_jwt;

use crate::*;
use database::schema::*;
use luda_rpc::auth::google_auth::*;
use std::sync::OnceLock;
use verify_jwt::*;

pub async fn google_auth(
    ArchivedRequest { jwt }: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response, Error> {
    let jwks_client = {
        static GOOGLE_JWKS_CLIENT: OnceLock<GoogleJwksClient> = OnceLock::new();
        GOOGLE_JWKS_CLIENT.get_or_init(|| {
            GoogleJwksClient::new(
                "595497537052-2ah859bei8e1ugcdglrkim5b279euhpt.apps.googleusercontent.com"
                    .to_string(),
            )
        })
    };

    let Claims { sub, name } = jwks_client.verify(jwt).await.map_err(|err| {
        eprintln!("Failed to verify JWT: {:?}", err);
        Error::InternalServerError {
            err: "Failed to verify JWT".to_string(),
        }
    })?;

    let google_identity = db.get(GoogleIdentityDocGet { sub: &sub }).await?;

    if let Some(google_identity) = google_identity {
        session.login(&google_identity.user_id);
        return Ok(Response {});
    }

    let user_id = uuid::Uuid::new_v4().to_string();

    db.transact((
        UserDocCreate {
            id: &user_id,
            name: &name,
            ttl: None,
        },
        GoogleIdentityDocCreate {
            sub: &sub,
            user_id: &user_id,
            ttl: None,
        },
    ))
    .await?;

    session.login(user_id);

    Ok(Response {})
}
