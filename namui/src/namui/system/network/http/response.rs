pub use super::*;
use reqwest::StatusCode;

pub struct Response {
    inner: FetchResponse,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct FetchResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: ByteBuf,
}

impl Response {
    pub(super) fn new(response: FetchResponse) -> Response {
        Response { inner: response }
    }

    /// Get the `StatusCode` of this `Response`.
    #[inline]
    pub fn status(&self) -> StatusCode {
        StatusCode::from_u16(self.inner.status).unwrap()
    }

    #[inline]
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.inner.headers
    }

    #[inline]
    pub fn headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.inner.headers
    }

    pub fn content_length(&self) -> Option<u64> {
        self.inner
            .headers
            .get("content-length")
            .and_then(|s| s.parse().ok())
    }

    pub async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T, HttpError> {
        let full = self.inner.body.to_vec();

        serde_json::from_slice(&full).map_err(|error| HttpError::JsonParseError(error))
    }

    /// Get the response text.
    pub async fn text(self) -> Result<String, HttpError> {
        let full = self.inner.body.to_vec();

        String::from_utf8(full).map_err(|error| HttpError::Decode {
            message: error.to_string(),
        })
    }

    /// Get the response as bytes
    pub async fn bytes(self) -> Result<impl AsRef<[u8]>, HttpError> {
        Ok(self.inner.body)
    }

    pub async fn error_for_400599(self) -> Result<Self, HttpError> {
        let status_code = self.status().as_u16();
        if 400 <= status_code && status_code <= 599 {
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
