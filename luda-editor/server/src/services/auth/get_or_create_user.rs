use super::{
    documents::{IdentityDocument, UserDocument},
    *,
};
use crate::storage::dynamo_db::{GetItemError, TransactError};

pub async fn get_or_create_user(
    user_identity: UserIdentity,
) -> Result<UserDocument, GetOrCreateUserError> {
    let username = user_identity.username().to_string();
    let identity_id = user_identity.into_document_id();

    match crate::dynamo_db()
        .get_item::<IdentityDocument>(&identity_id, Option::<String>::None)
        .await
    {
        Ok(identity_document) => {
            let user_document = crate::dynamo_db()
                .get_item::<UserDocument>(&identity_document.user_id, Option::<String>::None)
                .await
                .map_err(|error| GetOrCreateUserError::GetUserError(error))?;
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
                    .map_err(|error| GetOrCreateUserError::CreateError(error))?;

                Ok(user_document)
            }
            GetItemError::DeserializeFailed(_) | GetItemError::Unknown(_) => {
                Err(GetOrCreateUserError::GetIdentityError(error))
            }
        },
    }
}

#[derive(Debug)]
pub enum GetOrCreateUserError {
    GetIdentityError(GetItemError),
    GetUserError(GetItemError),
    CreateError(TransactError),
}
crate::simple_error_impl!(GetOrCreateUserError);
