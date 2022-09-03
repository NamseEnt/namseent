use super::*;
use crate::storage::dynamo_db::{CreateItemError, GetItemError};

pub async fn get_or_create_user(
    user_identity: UserIdentity,
) -> Result<UserDocument, GetOrCreateUserError> {
    let user_document_id = user_identity.into_document_id();

    match crate::dynamo_db()
        .get_item::<UserDocument>(&user_document_id, None)
        .await
    {
        Ok(user_document) => Ok(user_document),
        Err(error) => match error {
            GetItemError::NotFound => {
                let user_document = UserDocument {
                    id: user_document_id,
                };
                crate::dynamo_db()
                    .create_item(user_document.clone())
                    .await?;
                Ok(user_document)
            }
            GetItemError::DeserializeFailed(_) | GetItemError::Unknown(_) => {
                Err(GetOrCreateUserError::GetUserError(error))
            }
        },
    }
}

#[derive(Debug)]
pub enum GetOrCreateUserError {
    GetUserError(GetItemError),
    CreateUserError(CreateItemError),
}
crate::simple_error_impl!(GetOrCreateUserError);

impl From<CreateItemError> for GetOrCreateUserError {
    fn from(error: CreateItemError) -> Self {
        GetOrCreateUserError::CreateUserError(error)
    }
}
