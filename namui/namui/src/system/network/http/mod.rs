#[cfg(not(target_os = "wasi"))]
mod non_wasi;
#[cfg(target_os = "wasi")]
pub(crate) mod wasi;

#[cfg(not(target_os = "wasi"))]
use non_wasi as inner;
#[cfg(target_os = "wasi")]
use wasi as inner;

use crate::simple_error_impl;
use http::{header::CONTENT_LENGTH, StatusCode};
pub use http::{Request, Response};

#[allow(async_fn_in_trait)]
pub trait RequestExt {
    async fn send(self) -> std::result::Result<Response<impl ResponseBody>, HttpError>;
}

impl RequestExt for Request<()> {
    async fn send(self) -> std::result::Result<Response<impl ResponseBody>, HttpError> {
        inner::send(self.map(|_| http_body_util::Empty::new())).await
    }
}

#[allow(async_fn_in_trait)]
pub trait ResponseExt {
    fn ensure_status_code(self) -> std::result::Result<Self, HttpError>
    where
        Self: Sized;
    async fn bytes(self) -> std::result::Result<Vec<u8>, HttpError>
    where
        Self: Sized;
}
impl<T: ResponseBody> ResponseExt for Response<T> {
    fn ensure_status_code(self) -> std::result::Result<Self, HttpError>
    where
        Self: Sized,
    {
        StatusCode::from_u16(self.status().as_u16())
            .map_err(|err| HttpError::HttpError(err.into()))
            .map(|_| self)
    }
    async fn bytes(self) -> std::result::Result<Vec<u8>, HttpError>
    where
        Self: Sized,
    {
        let content_length = self
            .headers()
            .get(CONTENT_LENGTH)
            .map(|header| {
                header
                    .to_str()
                    .map_err(|_| HttpError::WrongContentLength(Some(header.as_bytes().to_vec())))?
                    .parse::<usize>()
                    .map_err(|_| HttpError::WrongContentLength(Some(header.as_bytes().to_vec())))
            })
            .transpose()?;

        self.into_body().bytes(content_length).await
    }
}

pub enum ReqBody {
    Empty,
    Vec { data: Vec<u8> },
}

impl From<()> for ReqBody {
    fn from(_: ()) -> Self {
        ReqBody::Empty
    }
}

pub trait ResponseBody {
    fn bytes(
        self,
        content_length: Option<usize>,
    ) -> impl std::future::Future<Output = std::result::Result<Vec<u8>, HttpError>> + std::marker::Send;
}

#[derive(Debug)]
pub enum HttpError {
    Disconnected,
    WrongContentLength(Option<Vec<u8>>),
    HyperError(hyper::Error),
    TooManyBytes,
    HttpError(http::Error),
    ReqBodyErr(Box<dyn std::error::Error + Send + Sync>),
    TaskJoinError(tokio::task::JoinError),
    Unknown(String),
}
simple_error_impl!(HttpError);

impl From<hyper::Error> for HttpError {
    fn from(e: hyper::Error) -> Self {
        HttpError::HyperError(e)
    }
}
