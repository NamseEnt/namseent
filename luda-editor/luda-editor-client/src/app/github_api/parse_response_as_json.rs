use serde::Deserialize;
use std::fmt::Display;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

pub async fn parse_response_as_json<ResponseBody>(
    response: Response,
) -> Result<ResponseBody, ResponseParseError>
where
    ResponseBody: for<'de> Deserialize<'de>,
{
    Ok(JsFuture::from(response.json()?)
        .await?
        .into_serde::<ResponseBody>()?)
}

#[derive(Debug)]
pub enum ResponseParseError {
    CouldNotReadJsValueAsJson(JsValue),
    JsonParseError(serde_json::Error),
}
impl Display for ResponseParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "response parse error: {}",
            match self {
                ResponseParseError::CouldNotReadJsValueAsJson(error) => format!("{error:#?}"),
                ResponseParseError::JsonParseError(error) => format!("{error:#?}"),
            }
        )
    }
}
impl std::error::Error for ResponseParseError {}

impl From<JsValue> for ResponseParseError {
    fn from(error: JsValue) -> Self {
        Self::CouldNotReadJsValueAsJson(error)
    }
}

impl From<serde_json::Error> for ResponseParseError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonParseError(error)
    }
}
