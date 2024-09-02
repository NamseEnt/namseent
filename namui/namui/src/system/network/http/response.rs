pub use super::*;
use reqwest::{header::HeaderMap, StatusCode};

pub struct Response {
    reqwest_response: reqwest::Response,
}
impl Response {
    pub(super) fn new(reqwest_response: reqwest::Response) -> Response {
        Response { reqwest_response }
    }

    /// Get the `StatusCode` of this `Response`.
    #[inline]
    pub fn status(&self) -> StatusCode {
        self.reqwest_response.status()
    }

    /// Get the `Headers` of this `Response`.
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        self.reqwest_response.headers()
    }

    /// Get a mutable reference to the `Headers` of this `Response`.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        self.reqwest_response.headers_mut()
    }

    /// Get the content-length of this response, if known.
    ///
    /// Reasons it may not be known:
    ///
    /// - The server didn't send a `content-length` header.
    /// - The response is compressed and automatically decoded (thus changing
    ///   the actual decoded length).
    pub fn content_length(&self) -> Option<u64> {
        self.reqwest_response.content_length()
    }

    /// Get the final `Url` of this `Response`.
    #[inline]
    pub fn url(&self) -> &Url {
        self.reqwest_response.url()
    }

    pub async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T, HttpError> {
        let full = self.reqwest_response.bytes().await?;

        serde_json::from_slice(&full).map_err(HttpError::JsonParseError)
    }

    /// Get the response text.
    pub async fn text(self) -> Result<String, HttpError> {
        self.reqwest_response
            .text()
            .await
            .map_err(|error| HttpError::TextParseError {
                message: error.to_string(),
            })
    }

    /// Get the response as bytes
    pub async fn bytes(self) -> Result<impl AsRef<[u8]>, HttpError> {
        self.reqwest_response
            .bytes()
            .await
            .map_err(|error| HttpError::Decode {
                message: error.to_string(),
            })
    }

    pub async fn error_for_400599(self) -> Result<Self, HttpError> {
        let status_code = self.status().as_u16();
        if (400..=599).contains(&status_code) {
            Err(HttpError::Status {
                status: status_code,
                message: self
                    .text()
                    .await
                    .unwrap_or("Fail to get response text".to_string()),
            })
        } else {
            Ok(self)
        }
    }
}
