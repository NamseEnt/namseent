use super::*;
use crate::documents::*;
use crate::storage::dynamo_db::{GetItemError, NoCancel, TransactError};

pub async fn get_or_create_user(
    user_identity: UserIdentity,
) -> Result<UserDocument, GetOrCreateUserError> {
    let username = user_identity.username().to_string();
    let identity_id = user_identity.into_document_id();

    match (IdentityDocumentGet {
        pk_id: identity_id.clone(),
    })
    .run()
    .await
    {
        Ok(identity_document) => {
            let user_document = UserDocumentGet {
                pk_id: identity_document.user_id,
            }
            .run()
            .await
            .map_err(GetOrCreateUserError::GetUser)?;
            Ok(user_document)
        }
        Err(error) => match error {
            GetItemError::NotFound => {
                let user_id = rpc::Uuid::new_v4();
                let user_document = UserDocument {
                    id: user_id,
                    name: username,
                };

                crate::dynamo_db()
                    .transact()
                    .create_item(user_document.clone())
                    .create_item(IdentityDocument {
                        id: identity_id,
                        user_id,
                    })
                    .send()
                    .await
                    .map_err(GetOrCreateUserError::Create)?;

                Ok(user_document)
            }
            GetItemError::DeserializeFailed(_) | GetItemError::Unknown(_) => {
                Err(GetOrCreateUserError::GetIdentity(error))
            }
        },
    }
}

#[derive(Debug)]
pub enum GetOrCreateUserError {
    GetIdentity(GetItemError),
    GetUser(GetItemError),
    Create(TransactError<NoCancel>),
}
crate::simple_error_impl!(GetOrCreateUserError);
