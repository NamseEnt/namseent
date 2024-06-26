use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorMessage {
    #[serde(rename = "relativeFile")]
    pub relative_file: String,
    #[serde(rename = "absoluteFile")]
    pub absolute_file: String,
    pub line: usize,
    pub column: usize,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketMessage {
    #[serde(rename = "buildStart")]
    BuildStart,

    #[serde(rename = "reload")]
    Reload,

    #[serde(rename = "error")]
    Error {
        #[serde(rename = "errorMessages")]
        error_messages: Vec<ErrorMessage>,
    },
}
