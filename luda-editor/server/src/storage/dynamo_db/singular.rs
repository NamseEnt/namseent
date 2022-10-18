use super::*;
use aws_sdk_dynamodb::model::AttributeValue;
use futures::Future;
use std::{error::Error, fmt::Debug};

impl DynamoDb {
    pub(super) async fn get_item_internal(
        &self,
        partition_key: impl ToString,
        sort_key: Option<impl ToString>,
    ) -> Result<Item, GetItemInternalError> {
        let result = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key(
                PARTITION_KEY,
                aws_sdk_dynamodb::model::AttributeValue::S(partition_key.to_string()),
            )
            .key(
                SORT_KEY,
                aws_sdk_dynamodb::model::AttributeValue::S(
                    sort_key
                        .map(|sort_key| sort_key.to_string())
                        .unwrap_or(DEFAULT_SORT_KEY.to_string()),
                ),
            )
            .send()
            .await;

        if let Err(error) = result {
            eprintln!("error on get_item_internal: {:?}", error);
            return Err(GetItemInternalError::Unknown(error.to_string()));
        }
        let item = result.unwrap().item;
        match item {
            Some(item) => Ok(item),
            None => {
                return Err(GetItemInternalError::NotFound);
            }
        }
    }

    pub async fn get_item<'a, TDocument: Document>(
        &self,
        partition_key_without_prefix: impl ToString,
        sort_key: Option<impl ToString>,
    ) -> Result<TDocument, GetItemError> {
        let partition_key = get_partition_key::<TDocument>(partition_key_without_prefix);
        let item = self.get_item_internal(partition_key, sort_key).await?;

        serde_dynamo::from_item(item)
            .map_err(|error| GetItemError::DeserializeFailed(error.to_string()))
    }

    pub async fn put_item(&self, item: impl Document) -> Result<(), PutItemError> {
        let partition_key = item.partition_key();
        let sort_key = item.sort_key().unwrap_or(DEFAULT_SORT_KEY.to_string());

        let mut item: Item = serde_dynamo::to_item(item)?;
        item.insert(VERSION_KEY.to_string(), AttributeValue::N("0".to_string()));
        item.insert(PARTITION_KEY.to_string(), AttributeValue::S(partition_key));
        item.insert(SORT_KEY.to_string(), AttributeValue::S(sort_key));

        let result = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(PutItemError::Unknown(error.to_string())),
        }
    }

    pub async fn create_item(&self, item: impl Document) -> Result<(), CreateItemError> {
        let partition_key = item.partition_key();
        let sort_key = item.sort_key().unwrap_or(DEFAULT_SORT_KEY.to_string());

        let mut item: Item = serde_dynamo::to_item(item)?;
        item.insert(VERSION_KEY.to_string(), AttributeValue::N("0".to_string()));
        item.insert(PARTITION_KEY.to_string(), AttributeValue::S(partition_key));
        item.insert(SORT_KEY.to_string(), AttributeValue::S(sort_key));

        let result = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(#PARTITION)")
            .expression_attribute_names("#PARTITION", PARTITION_KEY)
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => {
                let error: aws_sdk_dynamodb::Error = error.into();
                if let aws_sdk_dynamodb::Error::ConditionalCheckFailedException(_) = error {
                    Err(CreateItemError::AlreadyExists)
                } else {
                    Err(CreateItemError::Unknown(error.to_string()))
                }
            }
        }
    }

    pub async fn update_item<
        TDocument: Document,
        TCancelError: std::error::Error,
        TUpdateFuture: Future<Output = Result<TDocument, TCancelError>>,
    >(
        &self,
        partition_key_without_prefix: impl ToString,
        sort_key: Option<impl ToString>,
        update: impl FnOnce(TDocument) -> TUpdateFuture,
    ) -> Result<(), UpdateItemError<TCancelError>> {
        let partition_key = get_partition_key::<TDocument>(partition_key_without_prefix);
        let sort_key = sort_key
            .map(|sort_key| sort_key.to_string())
            .unwrap_or(DEFAULT_SORT_KEY.to_string());
        let item = self
            .get_item_internal(partition_key.clone(), Some(sort_key.clone()))
            .await?;

        let version = {
            let version_value = item.get(VERSION_KEY).unwrap().clone();
            let version_n = version_value.as_n().unwrap();
            str::parse::<u128>(version_n).unwrap()
        };

        let document: TDocument = serde_dynamo::from_item(item)?;
        let result = update(document).await;
        if let Err(error) = result {
            return Err(UpdateItemError::Canceled(error));
        }
        let document = result.unwrap();

        let mut item: Item = serde_dynamo::to_item(document)?;
        let next_version = version + 1;
        item.insert(
            VERSION_KEY.to_string(),
            AttributeValue::N(next_version.to_string()),
        );
        item.insert(PARTITION_KEY.to_string(), AttributeValue::S(partition_key));
        item.insert(SORT_KEY.to_string(), AttributeValue::S(sort_key));

        let result = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .condition_expression("#VERSION = :version")
            .expression_attribute_names("#VERSION", VERSION_KEY)
            .expression_attribute_values(":version", AttributeValue::N(version.to_string()))
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => {
                let error: aws_sdk_dynamodb::Error = error.into();
                if let aws_sdk_dynamodb::Error::ConditionalCheckFailedException(_) = error {
                    Err(UpdateItemError::Conflict)
                } else {
                    Err(UpdateItemError::Unknown(error.to_string()))
                }
            }
        }
    }

    pub async fn query<TDocument: Document>(
        &self,
        partition_key_without_prefix: impl ToString,
    ) -> Result<Vec<TDocument>, QueryError> {
        let partition_key = get_partition_key::<TDocument>(partition_key_without_prefix);
        let query_result = self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("#PARTITION = :partition")
            .expression_attribute_names("#PARTITION", PARTITION_KEY)
            .expression_attribute_values(":partition", AttributeValue::S(partition_key))
            .send()
            .await;

        match query_result {
            Ok(query_result) => {
                let items = query_result.items.unwrap();
                let mut documents = Vec::new();
                for item in items {
                    let document = serde_dynamo::from_item(item).unwrap();
                    documents.push(document);
                }
                Ok(documents)
            }
            Err(error) => Err(QueryError::Unknown(error.to_string())),
        }
    }

    pub async fn delete_item<TDocument: Document>(
        &self,
        partition_key_without_prefix: impl ToString,
        sort_key: Option<impl ToString>,
    ) -> Result<(), DeleteItemError> {
        let partition_key = get_partition_key::<TDocument>(partition_key_without_prefix);
        let sort_key = sort_key
            .map(|sort_key| sort_key.to_string())
            .unwrap_or(DEFAULT_SORT_KEY.to_string());

        let result = self
            .client
            .delete_item()
            .table_name(&self.table_name)
            .key(PARTITION_KEY, AttributeValue::S(partition_key))
            .key(SORT_KEY, AttributeValue::S(sort_key))
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(DeleteItemError::Unknown(error.to_string())),
        }
    }
}

#[derive(Debug)]
pub(super) enum GetItemInternalError {
    NotFound,
    Unknown(String),
}
crate::simple_error_impl!(GetItemInternalError);

#[derive(Debug)]
pub enum GetItemError {
    NotFound,
    DeserializeFailed(String),
    Unknown(String),
}
crate::simple_error_impl!(GetItemError);

impl From<GetItemInternalError> for GetItemError {
    fn from(error: GetItemInternalError) -> Self {
        match error {
            GetItemInternalError::NotFound => GetItemError::NotFound,
            GetItemInternalError::Unknown(error) => GetItemError::Unknown(error),
        }
    }
}

#[derive(Debug)]
pub enum CreateItemError {
    SerializeFailed(String),
    AlreadyExists,
    Unknown(String),
}
crate::simple_error_impl!(CreateItemError);

impl From<serde_dynamo::Error> for CreateItemError {
    fn from(error: serde_dynamo::Error) -> Self {
        CreateItemError::SerializeFailed(error.to_string())
    }
}

#[derive(Debug)]
pub enum UpdateItemError<TCancelError: Error> {
    NotFound,
    /// Serialize or deserialize failed
    SerializationFailed(String),
    Conflict, // TODO: Remove this and retry internally
    Canceled(TCancelError),
    Unknown(String),
}
impl<TCancelError: Error> std::fmt::Display for UpdateItemError<TCancelError> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<TCancelError: Error> std::error::Error for UpdateItemError<TCancelError> {}

impl<TCancelError: Error> From<GetItemInternalError> for UpdateItemError<TCancelError> {
    fn from(error: GetItemInternalError) -> Self {
        match error {
            GetItemInternalError::NotFound => UpdateItemError::NotFound,
            GetItemInternalError::Unknown(error) => UpdateItemError::Unknown(error),
        }
    }
}

impl<TCancelError: Error> From<serde_dynamo::Error> for UpdateItemError<TCancelError> {
    fn from(error: serde_dynamo::Error) -> Self {
        UpdateItemError::SerializationFailed(error.to_string())
    }
}

#[derive(Debug)]
pub enum QueryError {
    Unknown(String),
}
crate::simple_error_impl!(QueryError);

#[derive(Debug)]
pub enum PutItemError {
    SerializeFailed(String),
    #[allow(dead_code)]
    Unknown(String),
}
crate::simple_error_impl!(PutItemError);

impl From<serde_dynamo::Error> for PutItemError {
    fn from(error: serde_dynamo::Error) -> Self {
        PutItemError::SerializeFailed(error.to_string())
    }
}

#[derive(Debug)]
pub enum DeleteItemError {
    Unknown(String),
}
crate::simple_error_impl!(DeleteItemError);
