mod response;
mod simple;

use crate::simple_error_impl;
use reqwest::IntoUrl;
pub use reqwest::Method;
pub use response::*;
pub use simple::*;
use url::*;

pub async fn fetch(
    url: impl IntoUrl,
    method: Method,
    build: impl FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
) -> Result<Response, HttpError> {
    let builder = reqwest::Client::new().request(method, url);
    Ok(Response::new(build(builder).send().await?))
}

pub async fn fetch_bytes(
    url: impl IntoUrl,
    method: Method,
    build: impl FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
) -> Result<impl AsRef<[u8]>, HttpError> {
    let response = fetch(url, method, build).await?;
    response.error_for_400599().await?.bytes().await
}

pub async fn fetch_serde<T, TDeserializeError, TDeserialize>(
    url: impl IntoUrl,
    method: Method,
    build: impl FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
    deserialize: TDeserialize,
) -> Result<T, HttpError>
where
    T: serde::de::DeserializeOwned,
    TDeserializeError: serde::de::Error,
    TDeserialize: FnOnce(&[u8]) -> Result<T, TDeserializeError>,
{
    deserialize(fetch_bytes(url, method, build).await?.as_ref()).map_err(|error| {
        HttpError::Deserialize {
            message: error.to_string(),
        }
    })
}

pub async fn fetch_json<T: serde::de::DeserializeOwned>(
    url: impl IntoUrl,
    method: Method,
    build: impl FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
) -> Result<T, HttpError> {
    fetch_serde(url, method, build, |slice| serde_json::from_slice(slice)).await
}

#[derive(Debug)]
pub enum HttpError {
    Status { status: u16, message: String },
    Timeout { message: String },
    Request { message: String },
    RedirectPolicy { message: String },
    Connection { message: String },
    Body { message: String },
    Decode { message: String },
    Unknown(Box<dyn std::error::Error>),
    Deserialize { message: String },
    UrlParseError(url::ParseError),
    JsonParseError(serde_json::Error),
    TextParseError { message: String },
}
simple_error_impl!(HttpError);

impl From<reqwest::Error> for HttpError {
    fn from(error: reqwest::Error) -> Self {
        let is_connect = {
            #[cfg(not(target_arch = "wasm32"))]
            fn is_connect(error: &reqwest::Error) -> bool {
                error.is_connect()
            }
            #[cfg(target_arch = "wasm32")]
            fn is_connect(_: &reqwest::Error) -> bool {
                false
            }
            is_connect(&error)
        };

        if error.is_timeout() {
            HttpError::Timeout {
                message: format!("{:?}", error),
            }
        } else if error.is_request() {
            HttpError::Request {
                message: format!("{:?}", error),
            }
        } else if error.is_redirect() {
            HttpError::RedirectPolicy {
                message: format!("{:?}", error),
            }
        } else if is_connect {
            HttpError::Connection {
                message: format!("{:?}", error),
            }
        } else if error.is_decode() {
            HttpError::Decode {
                message: format!("{:?}", error),
            }
        } else if error.is_body() {
            HttpError::Body {
                message: format!("{:?}", error),
            }
        } else {
            HttpError::Unknown(error.into())
        }
    }
}
impl From<url::ParseError> for HttpError {
    fn from(error: url::ParseError) -> Self {
        HttpError::UrlParseError(error)
    }
}
