mod error;

pub use error::*;
use serde::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Encoding {
    Base64,
    Utf8,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GitFileType {
    Blob,
    Tree,
    Commit,
}
