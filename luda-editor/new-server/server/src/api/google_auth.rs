use crate::kv_store::HeapArchived;
use crate::*;
use anyhow::Result;
use md5::{Digest, Md5};

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Request {
    access_token: String,
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
    pub async fn put(&self, db: &Db) -> Result<()> {
        let key = format!(
            "GoogleIdentity/google_sub:{google_sub}",
            google_sub = self.google_sub
        );

        let bytes = rkyv::to_bytes::<_, 64>(self)?;
        db.sqlite.put(key, &bytes).await?;
        Ok(())
    }
}

pub struct GoogleIdentityGet {
    pub google_sub: String,
}

impl GoogleIdentityGet {
    pub async fn get(&self, db: &Db) -> Result<Option<HeapArchived<GoogleIdentity>>> {
        let key = format!(
            "GoogleIdentity/google_sub:{google_sub}",
            google_sub = self.google_sub
        );
        Ok(db.sqlite.get(key).await?.map(HeapArchived::new))
    }
}

#[derive(rkyv::Archive, rkyv::Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub async fn put(&self, db: &Db) -> Result<()> {
        let key = format!("User/id:{id}", id = self.id);

        let bytes = rkyv::to_bytes::<_, 64>(self)?;
        db.sqlite.put(key, &bytes).await?;
        Ok(())
    }

    async fn create(&self, db: &Db) -> Result<()> {
        let key = format!("User/id:{id}", id = self.id);

        db.sqlite
            .create(key, || Ok(rkyv::to_bytes::<_, 64>(self)?))
            .await?;
        Ok(())
    }
}

pub async fn google_auth(
    ArchivedRequest { access_token }: &ArchivedRequest,
    db: Db,
    session: Session,
) -> Result<Response> {
    #[derive(serde::Deserialize)]
    struct GoogleUserInfoResponse {
        sub: String,
        name: String,
        // "given_name": "John",
        // "family_name": "Doe",
        // "picture": "<Profile picture URL>",
        // "email": "john.doe@gmail.com",
        // "email_verified": true,
        // "locale": "en"
    }

    let client = reqwest::Client::new();
    let GoogleUserInfoResponse { name, sub } = client
        .get(format!(
            "https://www.googleapis.com/oauth2/v3/userinfo?access_token={access_token}",
        ))
        .send()
        .await?
        .json()
        .await?;

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
