mod singular;
mod transact;

use aws_sdk_dynamodb::{model::AttributeValue, types::Blob};
use lambda_web::is_running_on_lambda;
pub use singular::*;
pub use transact::*;

#[derive(Debug, Clone)]
pub struct DynamoDb {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
}

const LOCK_VERSION_KEY: &str = "l";
const PARTITION_KEY: &str = "p";
const SORT_KEY: &str = "s";
const DEFAULT_SORT_KEY: &str = "_";
const BYTES_KEY: &str = "b";
const MIGRATION_VERSION_KEY: &str = "m";

type RawItem = std::collections::HashMap<std::string::String, AttributeValue>;

pub trait Document: serde::Serialize + serde::de::DeserializeOwned + migration::Migration {
    fn partition_key_prefix() -> &'static str;
    fn partition_key_without_prefix(&self) -> String;
    fn sort_key(&self) -> Option<String>;
    fn partition_key(&self) -> String {
        get_partition_key::<Self>(self.partition_key_without_prefix())
    }

    fn as_dynamodb_item(&self) -> Result<RawItem, bincode::Error> {
        let partition_key = self.partition_key();
        let sort_key = self.sort_key().unwrap_or(DEFAULT_SORT_KEY.to_string());

        let mut raw_item: RawItem = RawItem::new();
        raw_item.insert(
            LOCK_VERSION_KEY.to_string(),
            AttributeValue::N("0".to_string()),
        );
        raw_item.insert(PARTITION_KEY.to_string(), AttributeValue::S(partition_key));
        raw_item.insert(SORT_KEY.to_string(), AttributeValue::S(sort_key));
        raw_item.insert(
            BYTES_KEY.to_string(),
            AttributeValue::B(Blob::new(bincode::serialize(&self)?)),
        );
        raw_item.insert(
            MIGRATION_VERSION_KEY.to_string(),
            AttributeValue::N(Self::migration_version().to_string()),
        );

        Ok(raw_item)
    }
}

impl DynamoDb {
    pub fn new(config: &aws_config::SdkConfig) -> Self {
        let table_name = {
            if is_running_on_lambda() {
                std::env::var("DYNAMODB_TABLE_NAME").expect("DYNAMODB_TABLE_NAME is not set")
            } else {
                "one-for-all".to_string()
            }
        };

        DynamoDb {
            client: aws_sdk_dynamodb::Client::new(config),
            table_name,
        }
    }
}

fn get_partition_key<TDocument: Document>(partition_key_without_prefix: impl ToString) -> String {
    concat_partition_key(
        TDocument::partition_key_prefix(),
        partition_key_without_prefix,
    )
}

fn concat_partition_key(
    partition_prefix: impl ToString,
    partition_key_without_prefix: impl ToString,
) -> String {
    format!(
        "{}.{}",
        partition_prefix.to_string(),
        partition_key_without_prefix.to_string()
    )
}

pub struct WithVersion<T: Document> {
    document: T,
    lock_version_key: u128,
}

impl<T: Document> WithVersion<T> {
    pub fn new(document: T, version: u128) -> Self {
        WithVersion {
            document,
            lock_version_key: version,
        }
    }
    pub fn version(&self) -> u128 {
        self.lock_version_key
    }
    pub fn into_inner(self) -> T {
        self.document
    }
}

impl<T: Document> std::ops::Deref for WithVersion<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.document
    }
}

impl<T: Document> std::ops::DerefMut for WithVersion<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.document
    }
}

// TODO: Make every call idempotent using Transaction Id
