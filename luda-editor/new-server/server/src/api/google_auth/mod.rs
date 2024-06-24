mod verify_jwt;

use crate::*;
use anyhow::Result;
use md5::{Digest, Md5};
use std::sync::OnceLock;
use verify_jwt::*;

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Request {
    pub jwt: String,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Response {}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub struct GoogleIdentity {
    pub google_sub: String,
    pub user_id: String,
}

impl GoogleIdentity {
    #[allow(dead_code)]
    pub async fn put(&self, db: &Database) -> Result<()> {
        let key = format!(
            "GoogleIdentity/google_sub:{google_sub}",
            google_sub = self.google_sub
        );

        let bytes = rkyv::to_bytes::<_, 64>(self)?;
        db.put(key, &bytes, None).await?;
        Ok(())
    }
}

pub struct GoogleIdentityGet {
    pub google_sub: String,
}

impl GoogleIdentityGet {
    #[allow(dead_code)]
    pub async fn get(&self, db: &Database) -> Result<Option<HeapArchived<GoogleIdentity>>> {
        let key = format!(
            "GoogleIdentity/google_sub:{google_sub}",
            google_sub = self.google_sub
        );
        Ok(db.get(key).await?.map(HeapArchived::new))
    }
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    #[allow(dead_code)]
    pub async fn put(&self, db: &Database) -> Result<()> {
        let key = format!("User/id:{id}", id = self.id);

        let bytes = rkyv::to_bytes::<_, 64>(self)?;
        db.put(key, &bytes, None).await?;
        Ok(())
    }
    #[allow(dead_code)]
    async fn create(&self, db: &Database) -> Result<()> {
        let key = format!("User/id:{id}", id = self.id);

        db.create(key, || Ok(rkyv::to_bytes::<_, 64>(self)?), None)
            .await?;
        Ok(())
    }
}

pub async fn google_auth(
    ArchivedRequest { jwt }: &ArchivedRequest,
    db: Database,
    session: Session,
) -> Result<Response> {
    static GOOGLE_JWKS_CLIENT: OnceLock<GoogleJwksClient> = OnceLock::new();

    let jwks_client = GOOGLE_JWKS_CLIENT.get_or_init(|| {
        GoogleJwksClient::new(
            "595497537052-2ah859bei8e1ugcdglrkim5b279euhpt.apps.googleusercontent.com".to_string(),
        )
    });

    let Claims { sub, name } = jwks_client.verify(jwt).await?;

    let google_identity = GoogleIdentityGet {
        google_sub: sub.clone(),
    }
    .get(&db)
    .await?;

    if let Some(google_identity) = google_identity {
        session.login(&google_identity.user_id);
        return Ok(Response {});
    }

    let user_id: String = deterministic_user_id(&sub);
    let user = User {
        id: user_id.clone(),
        name,
    };
    let google_identity = GoogleIdentity {
        google_sub: sub,
        user_id,
    };

    user.create(&db).await?;
    google_identity.put(&db).await?;

    session.login(&google_identity.user_id);

    Ok(Response {})
}

fn deterministic_user_id(from: impl AsRef<str>) -> String {
    let mut hasher = Md5::new();
    hasher.update(from.as_ref());
    let result = hasher.finalize();
    hex::encode(result)
}
