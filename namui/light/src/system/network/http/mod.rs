pub(crate) mod wasi;

use wasi as inner;

use crate::simple_error_impl;
pub use http::{Request, Response};
use http::{StatusCode, header::CONTENT_LENGTH};

#[allow(async_fn_in_trait)]
pub trait RequestExt {
    async fn send(self) -> std::result::Result<Response<impl ResponseBody>, HttpError>;
}

impl RequestExt for Request<()> {
    async fn send(self) -> std::result::Result<Response<impl ResponseBody>, HttpError> {
        inner::send(self.map(|_| http_body_util::Empty::<bytes::Bytes>::new())).await
    }
}

impl RequestExt for Request<Vec<u8>> {
    async fn send(self) -> std::result::Result<Response<impl ResponseBody>, HttpError> {
        inner::send(self.map(|body| http_body_util::Full::new(bytes::Bytes::from(body)))).await
    }
}

impl RequestExt for Request<Box<[u8]>> {
    async fn send(self) -> std::result::Result<Response<impl ResponseBody>, HttpError> {
        inner::send(self.map(|body| http_body_util::Full::new(bytes::Bytes::from(body)))).await
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
    fn stream(self) -> impl futures::Stream<Item = std::result::Result<bytes::Bytes, HttpError>>
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

    fn stream(self) -> impl futures::Stream<Item = std::result::Result<bytes::Bytes, HttpError>>
    where
        Self: Sized,
    {
        self.into_body().stream()
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
    fn stream(
        self,
    ) -> impl futures::Stream<Item = std::result::Result<bytes::Bytes, HttpError>>
    + std::marker::Send
    + Unpin;
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
    TrailerNotSupported,
}
simple_error_impl!(HttpError);

impl From<hyper::Error> for HttpError {
    fn from(e: hyper::Error) -> Self {
        HttpError::HyperError(e)
    }
}

impl From<http::Error> for HttpError {
    fn from(e: http::Error) -> Self {
        HttpError::HttpError(e)
    }
}
