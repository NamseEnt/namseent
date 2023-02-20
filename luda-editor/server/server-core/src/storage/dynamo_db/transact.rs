use super::*;
use aws_sdk_dynamodb::model;
use std::{future::Future, pin::Pin};

impl DynamoDb {
    pub fn transact(&self) -> Transact<NoCancel> {
        Transact {
            client: self.client.clone(),
            table_name: self.table_name.clone(),
            items: Vec::new(),
            dynamo_db: self.clone(),
            update_items: Vec::new(),
        }
    }
    pub fn transact_with_cancel<TCancelError: std::error::Error + Send>(
        &self,
    ) -> Transact<TCancelError> {
        Transact {
            client: self.client.clone(),
            table_name: self.table_name.clone(),
            items: Vec::new(),
            dynamo_db: self.clone(),
            update_items: Vec::new(),
        }
    }
}

pub struct Transact<TCancelError: std::error::Error + Send> {
    dynamo_db: DynamoDb,
    client: aws_sdk_dynamodb::Client,
    table_name: String,
    items: Vec<model::TransactWriteItem>,
    update_items: Vec<UpdateItem<TCancelError>>,
}
unsafe impl<TCancelError: std::error::Error + Send> Send for Transact<TCancelError> {}

struct UpdateItem<TCancelError: std::error::Error + Send> {
    future: Pin<
        Box<
            dyn Future<Output = Result<model::TransactWriteItem, TransactUpdateError<TCancelError>>>
                + Send,
        >,
    >,
}

impl<TCancelError: std::error::Error + Send> Transact<TCancelError> {
    pub async fn send(self) -> Result<(), TransactError> {
        let mut items = self.items;

        for update_item in self.update_items {
            let result = update_item.future.await;
            match result {
                Ok(write_item) => {
                    items.push(write_item);
                }
                Err(error) => {
                    return Err(TransactError::Unknown(error.to_string()));
                }
            }
        }

        let result = self
            .client
            .transact_write_items()
            .set_transact_items(Some(items))
            .send()
            .await;
        if let Err(error) = result {
            eprintln!("error on transact: {:?}", error);
            return Err(TransactError::Unknown(error.to_string()));
        }
        Ok(())
    }
    pub fn create_item<TDocument: Document>(mut self, item: TDocument) -> Self {
        let partition_key = item.partition_key();
        let sort_key = item.sort_key().unwrap_or(DEFAULT_SORT_KEY.to_string());

        let mut item: Item = serde_dynamo::to_item(item).unwrap();
        item.insert(VERSION_KEY.to_string(), AttributeValue::N("0".to_string()));
        item.insert(PARTITION_KEY.to_string(), AttributeValue::S(partition_key));
        item.insert(SORT_KEY.to_string(), AttributeValue::S(sort_key));

        let item = model::TransactWriteItem::builder()
            .put(
                model::Put::builder()
                    .table_name(&self.table_name)
                    .set_item(Some(item))
                    .condition_expression("attribute_not_exists(#PARTITION)")
                    .expression_attribute_names("#PARTITION", PARTITION_KEY)
                    .build(),
            )
            .build();

        self.items.push(item);

        self
    }
    pub fn delete_item(self, command: impl Into<TransactDeleteCommand>) -> Self {
        let command: TransactDeleteCommand = command.into();
        self.delete_item_internal(
            concat_partition_key(
                command.partition_prefix,
                command.partition_key_without_prefix,
            ),
            command.sort_key,
        )
    }
    pub fn update_item<
        Update: FnOnce(TDocument) -> TUpdateFuture + 'static + Send,
        TDocument: Document + std::marker::Send,
        TUpdateFuture: Future<Output = Result<TDocument, TCancelError>> + Send,
    >(
        self,
        command: impl Into<TransactUpdateCommand<TDocument, Update, TCancelError, TUpdateFuture>>,
    ) -> Self {
        let command: TransactUpdateCommand<TDocument, Update, TCancelError, TUpdateFuture> =
            command.into();
        self.update_item_internal(
            concat_partition_key(
                command.partition_prefix,
                command.partition_key_without_prefix,
            ),
            command.sort_key,
            command.update,
        )
    }
    fn delete_item_internal(
        mut self,
        partition_key: impl ToString,
        sort_key: Option<impl ToString>,
    ) -> Self {
        let partition_key = partition_key.to_string();
        let sort_key = sort_key
            .map(|sort_key| sort_key.to_string())
            .unwrap_or(DEFAULT_SORT_KEY.to_string());

        let item = model::TransactWriteItem::builder()
            .delete(
                model::Delete::builder()
                    .table_name(&self.table_name)
                    .key(PARTITION_KEY.to_string(), AttributeValue::S(partition_key))
                    .key(SORT_KEY.to_string(), AttributeValue::S(sort_key))
                    .build(),
            )
            .build();

        self.items.push(item);

        self
    }
    fn update_item_internal<
        TDocument: Document + std::marker::Send,
        TUpdateFuture: Future<Output = Result<TDocument, TCancelError>> + Send,
    >(
        mut self,
        partition_key_without_prefix: impl ToString,
        sort_key: Option<impl ToString>,
        update: impl FnOnce(TDocument) -> TUpdateFuture + 'static + Send,
    ) -> Self {
        let partition_key = get_partition_key::<TDocument>(partition_key_without_prefix);
        let sort_key = sort_key
            .map(|sort_key| sort_key.to_string())
            .unwrap_or(DEFAULT_SORT_KEY.to_string())
            .to_string();
        let dynamo_db = self.dynamo_db.clone();
        let table_name = self.table_name.clone();

        let future = Box::pin(async move {
            let item = dynamo_db
                .get_item_internal(partition_key.clone(), Some(sort_key.clone()))
                .await;
            if let Err(error) = item {
                return Err(TransactUpdateError::Unknown(error.to_string()));
            }
            let item = item.unwrap();

            let version = {
                let version_value = item.get(VERSION_KEY).unwrap().clone();
                let version_n = version_value.as_n().unwrap();
                str::parse::<u128>(version_n).unwrap()
            };

            let document: TDocument = serde_dynamo::from_item(item).unwrap();
            let result = update(document).await;
            if let Err(error) = result {
                return Err(TransactUpdateError::Canceled(error));
            }
            let document = result.unwrap();

            let mut item: Item = serde_dynamo::to_item(document).unwrap();
            let next_version = version + 1;
            item.insert(
                VERSION_KEY.to_string(),
                AttributeValue::N(next_version.to_string()),
            );
            item.insert(PARTITION_KEY.to_string(), AttributeValue::S(partition_key));
            item.insert(SORT_KEY.to_string(), AttributeValue::S(sort_key));

            Ok(model::TransactWriteItem::builder()
                .put(
                    model::Put::builder()
                        .table_name(table_name)
                        .set_item(Some(item))
                        .condition_expression("#VERSION = :version")
                        .expression_attribute_names("#VERSION", VERSION_KEY)
                        .expression_attribute_values(
                            ":version",
                            AttributeValue::N(version.to_string()),
                        )
                        .build(),
                )
                .build())
        });

        self.update_items.push(UpdateItem { future });

        self
    }
}

pub struct TransactDeleteCommand {
    pub partition_prefix: String,
    pub partition_key_without_prefix: String,
    pub sort_key: Option<String>,
}

pub struct TransactUpdateCommand<TDocument, Update, TCancelError, TUpdateFuture>
where
    Update: FnOnce(TDocument) -> TUpdateFuture + 'static + Send,
    TCancelError: std::error::Error + Send,
    TUpdateFuture: std::future::Future<Output = Result<TDocument, TCancelError>> + Send,
{
    pub partition_prefix: String,
    pub partition_key_without_prefix: String,
    pub sort_key: Option<String>,
    pub update: Update,
    pub _phantom: std::marker::PhantomData<(TDocument, TUpdateFuture)>,
}

#[derive(Debug)]
pub enum TransactError {
    Unknown(String),
}
crate::simple_error_impl!(TransactError);

#[derive(Debug)]
pub enum TransactUpdateError<TCancelError: std::error::Error + Send> {
    Canceled(TCancelError),
    Unknown(String),
}
impl<TCancelError: std::error::Error + Send> std::fmt::Display
    for TransactUpdateError<TCancelError>
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<TCancelError: std::error::Error + Send> std::error::Error
    for TransactUpdateError<TCancelError>
{
}

#[derive(Debug)]
pub enum NoCancel {}
impl std::fmt::Display for NoCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for NoCancel {}
