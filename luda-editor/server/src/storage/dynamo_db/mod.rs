mod singular;
mod transact;

use aws_sdk_dynamodb::model::AttributeValue;
use lambda_web::is_running_on_lambda;
pub use singular::*;
pub use transact::*;

#[derive(Debug, Clone)]
pub struct DynamoDb {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
}

const VERSION_KEY: &str = "__version__";
const PARTITION_KEY: &str = "__partition_key__";
const SORT_KEY: &str = "__sort_key__";
const DEFAULT_SORT_KEY: &str = "_";

type Item = std::collections::HashMap<std::string::String, AttributeValue>;

pub trait Document: serde::Serialize + serde::de::DeserializeOwned {
    fn partition_key_prefix() -> &'static str;
    fn partition_key_without_prefix(&self) -> String;
    fn sort_key(&self) -> Option<String>;
    fn partition_key(&self) -> String {
        get_partition_key::<Self>(self.partition_key_without_prefix())
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
    format!(
        "{}.{}",
        TDocument::partition_key_prefix(),
        partition_key_without_prefix.to_string()
    )
}

// TODO: Make every call idempotent using Transaction Id
