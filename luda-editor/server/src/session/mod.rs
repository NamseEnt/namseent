use rpc::hyper::{Body, Request};
use std::str::FromStr;

#[document_macro::document]
pub struct SessionDocument {
    #[pk]
    pub id: rpc::Uuid,
    pub user_id: rpc::Uuid,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum IdentitySource {
    Github,
}

const SESSION_HEADER_KEY: &str = "session";

pub async fn get_session(req: &Request<Body>) -> Result<Option<SessionDocument>, GetSessionError> {
    let header_value = req.headers().get(SESSION_HEADER_KEY);

    if header_value.is_none() {
        return Ok(None);
    }

    let header_value = header_value.unwrap();

    let session_id = rpc::Uuid::from_str(header_value.to_str().unwrap()).unwrap();

    let result = SessionDocumentGet { pk_id: session_id }.run().await;

    match result {
        Ok(session) => Ok(Some(session)),
        Err(error) => match error {
            crate::storage::dynamo_db::GetItemError::NotFound => {
                return Ok(None);
            }
            crate::storage::dynamo_db::GetItemError::DeserializeFailed(_)
            | crate::storage::dynamo_db::GetItemError::Unknown(_) => {
                eprintln!("fail to get session from dynamo db: {:?}", error);
                Err(GetSessionError::Unknown(error.to_string()))
            }
        },
    }
}

pub async fn create_session(user_id: rpc::Uuid) -> Result<SessionDocument, CreateSessionError> {
    let session = SessionDocument {
        id: rpc::Uuid::new_v4(),
        user_id,
    };

    let result = crate::dynamo_db().create_item(session.clone()).await;
    match result {
        Ok(_) => {
            return Ok(session);
        }
        Err(error) => match error {
            crate::storage::dynamo_db::CreateItemError::SerializeFailed(_) => {
                panic!("fail to serialize session to dynamo db: {:?}", error);
            }
            crate::storage::dynamo_db::CreateItemError::AlreadyExists => {
                panic!("session id already exists: {:?}", error);
            }
            crate::storage::dynamo_db::CreateItemError::Unknown(_) => {
                Err(CreateSessionError::Unknown(error.to_string()))
            }
        },
    }
}

#[derive(Debug)]
pub enum GetSessionError {
    Unknown(String),
}
crate::simple_error_impl!(GetSessionError);

#[derive(Debug)]
pub enum CreateSessionError {
    Unknown(String),
}
crate::simple_error_impl!(CreateSessionError);
