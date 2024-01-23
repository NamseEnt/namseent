use super::*;
use aws_sdk_dynamodb::model::AttributeValue;
use futures::Future;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, error::Error, fmt::Debug};

impl DynamoDb {
    /// Do not use this directly.
    pub(super) async fn get_raw_item(
        &self,
        partition_key: impl ToString,
        sort_key: Option<impl ToString>,
    ) -> Result<RawItem, GetItemInternalError> {
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
            None => Err(GetItemInternalError::NotFound),
        }
    }

    /// Do not use this function directly.
    pub async fn get_item<'a, TDocument: Document>(
        &self,
        partition_key_without_prefix: impl ToString,
        sort_key: Option<impl ToString>,
    ) -> Result<WithVersion<TDocument>, GetItemError> {
        let partition_key = get_partition_key::<TDocument>(partition_key_without_prefix);
        let raw_item = self.get_raw_item(partition_key, sort_key).await?;
        let lock_version_key = raw_item
            .get(LOCK_VERSION_KEY)
            .unwrap()
            .as_n()
            .unwrap()
            .parse::<u128>()
            .unwrap();
        let migration_version = raw_item
            .get(MIGRATION_VERSION_KEY)
            .unwrap()
            .as_n()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let bytes = raw_item.get(BYTES_KEY).unwrap().as_b().unwrap().as_ref();

        let document: TDocument = migration::Migration::deserialize(bytes, migration_version)
            .map_err(|error| GetItemError::DeserializeFailed(error.to_string()))?;

        Ok(WithVersion {
            document,
            lock_version_key,
        })
    }

    /// Do not use this function directly.
    pub async fn put_item(&self, item: impl Document) -> Result<(), PutItemError> {
        let result = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item.as_dynamodb_item()?))
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(PutItemError::Unknown(error.to_string())),
        }
    }

    /// Do not use this function directly.
    pub async fn create_item(&self, item: impl Document) -> Result<(), CreateItemError> {
        let mut raw_item = item.as_dynamodb_item().unwrap();
        raw_item.insert(
            LOCK_VERSION_KEY.to_string(),
            AttributeValue::N("0".to_string()),
        );

        let result = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(raw_item))
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

    /// Do not use this function directly.
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
            .get_raw_item(partition_key.clone(), Some(sort_key.clone()))
            .await?;

        let version = {
            let version_value = item.get(LOCK_VERSION_KEY).unwrap().clone();
            let version_n = version_value.as_n().unwrap();
            str::parse::<u128>(version_n).unwrap()
        };
        let bytes = item.get(BYTES_KEY).unwrap().as_b().unwrap().as_ref();
        let migration_version = item
            .get(MIGRATION_VERSION_KEY)
            .unwrap()
            .as_n()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let document: TDocument = migration::Migration::deserialize(bytes, migration_version)?;
        let result = update(document).await;
        if let Err(error) = result {
            return Err(UpdateItemError::Canceled(error));
        }
        let document = result.unwrap();

        let mut raw_item: RawItem = document.as_dynamodb_item()?;
        let next_version = version + 1;
        raw_item.insert(
            LOCK_VERSION_KEY.to_string(),
            AttributeValue::N(next_version.to_string()),
        );

        let result = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(raw_item))
            .condition_expression("#LOCK_VERSION = :lock_version")
            .expression_attribute_names("#LOCK_VERSION", LOCK_VERSION_KEY)
            .expression_attribute_values(":lock_version", AttributeValue::N(version.to_string()))
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

    /// Do not use this function directly.
    pub async fn update_or_create_item<
        TDocument: Document,
        TCancelError: std::error::Error,
        TUpdateFuture: Future<Output = Result<TDocument, TCancelError>>,
        TCreateFuture: Future<Output = Result<TDocument, TCancelError>>,
    >(
        &self,
        partition_key_without_prefix: impl ToString,
        sort_key: Option<impl ToString>,
        update: impl FnOnce(TDocument) -> TUpdateFuture,
        create: impl FnOnce() -> TCreateFuture,
    ) -> Result<(), UpdateItemError<TCancelError>> {
        let partition_key = get_partition_key::<TDocument>(partition_key_without_prefix);
        let sort_key = sort_key
            .map(|sort_key| sort_key.to_string())
            .unwrap_or(DEFAULT_SORT_KEY.to_string());

        let result = match self
            .get_raw_item(partition_key.clone(), Some(sort_key.clone()))
            .await
        {
            Ok(item) => {
                let version = {
                    let version_value = item.get(LOCK_VERSION_KEY).unwrap().clone();
                    let version_n = version_value.as_n().unwrap();
                    str::parse::<u128>(version_n).unwrap()
                };
                let bytes = item.get(BYTES_KEY).unwrap().as_b().unwrap().as_ref();
                let migration_version = item
                    .get(MIGRATION_VERSION_KEY)
                    .unwrap()
                    .as_n()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();

                let document: TDocument =
                    migration::Migration::deserialize(bytes, migration_version)?;
                let document = update(document)
                    .await
                    .map_err(|error| UpdateItemError::Canceled(error))?;

                let mut raw_item: RawItem = document.as_dynamodb_item()?;
                let next_version = version + 1;
                raw_item.insert(
                    LOCK_VERSION_KEY.to_string(),
                    AttributeValue::N(next_version.to_string()),
                );

                self.client
                    .put_item()
                    .table_name(&self.table_name)
                    .set_item(Some(raw_item))
                    .condition_expression("#LOCK_VERSION = :lock_version")
                    .expression_attribute_names("#LOCK_VERSION", LOCK_VERSION_KEY)
                    .expression_attribute_values(
                        ":lock_version",
                        AttributeValue::N(version.to_string()),
                    )
                    .send()
                    .await
            }
            Err(GetItemInternalError::NotFound) => {
                let mut raw_item = create()
                    .await
                    .map_err(|error| UpdateItemError::Canceled(error))?
                    .as_dynamodb_item()
                    .unwrap(); // TODO: Remove unwrap
                raw_item.insert(
                    LOCK_VERSION_KEY.to_string(),
                    AttributeValue::N("0".to_string()),
                );

                self.client
                    .put_item()
                    .table_name(&self.table_name)
                    .set_item(Some(raw_item))
                    .condition_expression("attribute_not_exists(#PARTITION)")
                    .expression_attribute_names("#PARTITION", PARTITION_KEY)
                    .send()
                    .await
            }
            Err(error) => return Err(UpdateItemError::Unknown(error.to_string())),
        };

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

    /// Do not use this function directly.
    pub async fn query<TDocument: Document + Send>(
        &self,
        partition_key_without_prefix: impl ToString,
        next_page_key: Option<impl ToString>,
    ) -> Result<QueryOutput<TDocument>, QueryError> {
        let partition_key = get_partition_key::<TDocument>(partition_key_without_prefix);
        let query_result = self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("#PARTITION = :partition")
            .expression_attribute_names("#PARTITION", PARTITION_KEY)
            .expression_attribute_values(":partition", AttributeValue::S(partition_key.clone()))
            .set_exclusive_start_key(next_page_key.map(|last_sk| {
                let mut item = HashMap::new();
                item.insert(PARTITION_KEY.to_string(), AttributeValue::S(partition_key));
                item.insert(SORT_KEY.to_string(), AttributeValue::S(last_sk.to_string()));
                item
            }))
            .send()
            .await;

        match query_result {
            Ok(query_output) => {
                let items = query_output.items.unwrap();
                let documents = items
                    .into_par_iter()
                    .map(|item| {
                        migration::Migration::deserialize(
                            item.get(BYTES_KEY).unwrap().as_b().unwrap().as_ref(),
                            item.get(MIGRATION_VERSION_KEY)
                                .unwrap()
                                .as_n()
                                .unwrap()
                                .parse::<u64>()
                                .unwrap(),
                        )
                    })
                    .collect::<Result<_, _>>()?;

                Ok(QueryOutput {
                    documents,
                    next_page_key: query_output
                        .last_evaluated_key
                        .map(|mut last_evaluated_key| {
                            match last_evaluated_key.remove(SORT_KEY).unwrap() {
                                AttributeValue::S(value) => value,
                                _ => unreachable!(),
                            }
                        }),
                })
            }
            Err(error) => Err(QueryError::Unknown(error.to_string())),
        }
    }

    /// Do not use this function directly.
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
pub struct QueryOutput<TDocument: Document> {
    pub documents: Vec<TDocument>,
    pub next_page_key: Option<String>,
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

impl From<bincode::Error> for CreateItemError {
    fn from(error: bincode::Error) -> Self {
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

impl<TCancelError: Error> From<bincode::Error> for UpdateItemError<TCancelError> {
    fn from(error: bincode::Error) -> Self {
        UpdateItemError::SerializationFailed(error.to_string())
    }
}

#[derive(Debug)]
pub enum QueryError {
    SerializeFailed(String),
    Unknown(String),
}
crate::simple_error_impl!(QueryError);

impl From<bincode::Error> for QueryError {
    fn from(error: bincode::Error) -> Self {
        QueryError::SerializeFailed(error.to_string())
    }
}

#[derive(Debug)]
pub enum PutItemError {
    SerializeFailed(String),
    Unknown(String),
}
crate::simple_error_impl!(PutItemError);

impl From<bincode::Error> for PutItemError {
    fn from(error: bincode::Error) -> Self {
        PutItemError::SerializeFailed(error.to_string())
    }
}

#[derive(Debug)]
pub enum DeleteItemError {
    Unknown(String),
}
crate::simple_error_impl!(DeleteItemError);

#[cfg(test)]
mod test {
    use anyhow::Result;

    #[tokio::test]
    async fn test_query() -> Result<()> {
        crate::set_local_storage();

        #[derive(PartialEq)]
        #[document_macro::document]
        pub struct Item {
            #[pk]
            pub pk: usize,
            #[sk]
            pub sk: usize,
        }

        let items = vec![
            Item { pk: 1, sk: 1 },
            Item { pk: 1, sk: 2 },
            Item { pk: 1, sk: 3 },
            Item { pk: 2, sk: 1 },
            Item { pk: 2, sk: 2 },
            Item { pk: 2, sk: 3 },
            Item { pk: 2, sk: 4 },
        ];

        for item in items {
            crate::dynamo_db().put_item(item).await?;
        }

        let query_output = ItemQuery {
            pk_pk: 1,
            last_sk: None,
        }
        .run()
        .await?;

        assert_eq!(
            query_output.documents,
            vec![
                Item { pk: 1, sk: 1 },
                Item { pk: 1, sk: 2 },
                Item { pk: 1, sk: 3 },
            ]
        );
        assert_eq!(query_output.next_page_key, None);

        let query_output = ItemQuery {
            pk_pk: 2,
            last_sk: Some(ItemSortKey { sk: 2 }),
        }
        .run()
        .await?;

        assert_eq!(
            query_output.documents,
            vec![Item { pk: 2, sk: 3 }, Item { pk: 2, sk: 4 },]
        );
        assert_eq!(query_output.next_page_key, None);

        Ok(())
    }
}
