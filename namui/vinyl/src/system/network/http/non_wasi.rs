use super::{HttpError, ResponseBody};
use futures::StreamExt;
use http::{Request, Response};
use hyper_rustls::HttpsConnector;
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::TokioExecutor,
};
use std::sync::OnceLock;

pub(crate) async fn send<ReqBody, ReqBodyErr>(
    request: Request<ReqBody>,
) -> std::result::Result<Response<impl ResponseBody>, HttpError>
where
    ReqBody: http_body::Body<Error = ReqBodyErr> + Send + std::marker::Unpin + 'static,
    ReqBody::Data: Send,
    ReqBodyErr: std::error::Error + Send + Sync + 'static,
{
    let http_connector = {
        static HTTP_CONNECTOR: OnceLock<HttpsConnector<HttpConnector>> = OnceLock::new();
        HTTP_CONNECTOR
            .get_or_init(|| {
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .expect("failed to load native roots")
                    .https_or_http()
                    .enable_all_versions()
                    .build()
            })
            .clone()
    };

    let client = Client::builder(TokioExecutor::new()).build(http_connector);

    let response: Response<hyper::body::Incoming> = client
        .request(request)
        .await
        .map_err(|error| HttpError::Unknown(error.to_string()))?;

    Ok(response.map(http_body_util::BodyStream::new))
}

impl ResponseBody for http_body_util::BodyStream<hyper::body::Incoming> {
    async fn bytes(
        mut self,
        content_length: Option<usize>,
    ) -> std::result::Result<Vec<u8>, HttpError> {
        if let Some(content_length) = content_length {
            let mut bytes = Vec::with_capacity(content_length);
            while let Some(frame) = self.next().await {
                let frame = frame?;
                bytes.extend_from_slice(frame.into_data().unwrap().as_ref());
            }
            Ok(bytes)
        } else {
            let mut bytes = Vec::new();
            while let Some(frame) = self.next().await {
                let frame = frame?;
                bytes.extend_from_slice(frame.into_data().unwrap().as_ref());
            }
            Ok(bytes)
        }
    }

    fn stream(
        self,
    ) -> impl futures::Stream<Item = std::result::Result<bytes::Bytes, HttpError>>
    + std::marker::Send
    + Unpin {
        self.map(|result| {
            let frame = result?;
            if frame.is_trailers() {
                return Err(HttpError::TrailerNotSupported);
            }
            Ok(frame.into_data().unwrap())
        })
    }
}
