use super::{HttpError, ResponseBody};
use anyhow::Result;
use bytes::{Buf, Bytes, BytesMut};
use dashmap::DashMap;
use futures::StreamExt;
use http::{HeaderName, HeaderValue, Request, Response, StatusCode};
use std::{ptr::null, str::FromStr, sync::OnceLock};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot,
};

pub(crate) async fn send<ReqBody, ReqBodyErr>(
    request: Request<ReqBody>,
) -> std::result::Result<Response<impl ResponseBody>, HttpError>
where
    ReqBody: http_body::Body<Error = ReqBodyErr> + Send + std::marker::Unpin + 'static,
    ReqBody::Data: Send,
    ReqBodyErr: std::error::Error + Send + Sync + 'static,
{
    let (parts, body) = request.into_parts();

    let uri = parts.uri.to_string();
    let method = parts.method.to_string();
    let mut headers = BytesMut::new();

    for key in parts.headers.keys() {
        let values = parts.headers.get_all(key);
        let value_string = values
            .iter()
            .map(|value| value.to_str().unwrap())
            .collect::<Vec<&str>>()
            .join(", ");

        let key_bytes = key.as_str().as_bytes();
        let value_bytes = value_string.as_bytes();

        headers.extend_from_slice(&key_bytes.len().to_le_bytes());
        headers.extend_from_slice(key_bytes);
        headers.extend_from_slice(&value_bytes.len().to_le_bytes());
        headers.extend_from_slice(value_bytes);
    }

    // TODO: send body

    let fetch_id = unsafe {
        _http_fetch_start(
            uri.as_ptr(),
            uri.len(),
            method.as_ptr(),
            method.len(),
            headers.as_ptr(),
            headers.len(),
            null(),
            0,
        )
    };

    let (error_from_js_tx, error_from_js_rx) = oneshot::channel::<String>();

    error_from_js_txs().insert(fetch_id, error_from_js_tx);

    let (response_sender, response_receiver) =
        oneshot::channel::<Result<Response<ResBody>, HttpError>>();
    RESPONSE_WAITERS
        .get_or_init(Default::default)
        .insert(fetch_id, response_sender);

    tokio::select! {
        error_from_js = error_from_js_rx => {
            response_waiters().remove(&fetch_id);
            error_from_js_txs().remove(&fetch_id);
            Err(HttpError::Unknown(format!("error_from_js: {error_from_js:?}")))
        },
        response = response_receiver => {
            response_waiters().remove(&fetch_id);
            error_from_js_txs().remove(&fetch_id);
            response.unwrap()
        },
    }
}

unsafe extern "C" {
    fn _http_fetch_start(
        uri_ptr: *const u8,
        uri_byte_len: usize,
        method_ptr: *const u8,
        method_byte_len: usize,
        headers_ptr: *const u8,
        headers_byte_len: usize,
        body_ptr: *const u8,
        body_byte_len: usize,
    ) -> u32;
}

#[unsafe(no_mangle)]
pub extern "C" fn _http_fetch_response(
    fetch_id: u32,
    status: u16,
    headers_ptr: *const u8,
    headers_byte_len: usize,
    body_ptr: *const u8,
    body_byte_len: usize,
) {
    // headers 파싱: key-len(u32), key-bytes, value-len(u32), value-bytes 형식
    let headers_slice = unsafe { std::slice::from_raw_parts(headers_ptr, headers_byte_len) };
    let mut headers = Vec::new();
    let mut offset = 0;

    while offset < headers_byte_len {
        // key length 읽기 (little-endian u32)
        if offset + 4 > headers_byte_len {
            break;
        }
        let key_len = u32::from_le_bytes([
            headers_slice[offset],
            headers_slice[offset + 1],
            headers_slice[offset + 2],
            headers_slice[offset + 3],
        ]) as usize;
        offset += 4;

        // key bytes 읽기
        if offset + key_len > headers_byte_len {
            break;
        }
        let key = String::from_utf8_lossy(&headers_slice[offset..offset + key_len]).to_string();
        offset += key_len;

        // value length 읽기 (little-endian u32)
        if offset + 4 > headers_byte_len {
            break;
        }
        let value_len = u32::from_le_bytes([
            headers_slice[offset],
            headers_slice[offset + 1],
            headers_slice[offset + 2],
            headers_slice[offset + 3],
        ]) as usize;
        offset += 4;

        // value bytes 읽기
        if offset + value_len > headers_byte_len {
            break;
        }
        let value = String::from_utf8_lossy(&headers_slice[offset..offset + value_len]).to_string();
        offset += value_len;

        headers.push((key, value));
    }

    let (_, response_sender) = response_waiters().remove(&fetch_id).unwrap();

    let response = 'outer: {
        let status_code = match StatusCode::from_u16(status) {
            Ok(status_code) => status_code,
            Err(error) => break 'outer Err(HttpError::HttpError(error.into())),
        };
        let mut builder = Response::builder().status(status_code);

        let headers_mut = builder.headers_mut().unwrap();
        for (key, value) in headers {
            let header_name = match HeaderName::from_str(&key) {
                Ok(header_name) => header_name,
                Err(error) => break 'outer Err(HttpError::HttpError(error.into())),
            };
            let header_value = match HeaderValue::from_str(&value) {
                Ok(header_value) => header_value,
                Err(error) => break 'outer Err(HttpError::HttpError(error.into())),
            };
            headers_mut.insert(header_name, header_value);
        }

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        res_body_txs().insert(fetch_id, tx);

        let response = match builder.body(ResBody {
            instance: ResponseBodyInstance { fetch_id },
            rx,
        }) {
            Ok(response) => response,
            Err(error) => break 'outer Err(HttpError::HttpError(error)),
        };

        Ok(response)
    };

    let _ = response_sender.send(response);

    // body 전송
    if body_byte_len > 0 {
        let body_slice = unsafe { std::slice::from_raw_parts(body_ptr, body_byte_len) };
        let body_bytes = Bytes::copy_from_slice(body_slice);

        let Some(tx) = res_body_txs().get(&fetch_id) else {
            return;
        };
        let _ = tx.send(Ok(body_bytes));
    }
    let _ = res_body_txs().remove(&fetch_id);
}

type ResBodyChunk = Result<Bytes, HttpError>;
static RES_BODY_TXS: OnceLock<DashMap<u32, UnboundedSender<ResBodyChunk>>> = OnceLock::new();
fn res_body_txs() -> &'static DashMap<u32, UnboundedSender<ResBodyChunk>> {
    RES_BODY_TXS.get_or_init(Default::default)
}

type ResponseMakingResult = Result<Response<ResBody>, HttpError>;
static RESPONSE_WAITERS: OnceLock<DashMap<u32, oneshot::Sender<ResponseMakingResult>>> =
    OnceLock::new();
fn response_waiters() -> &'static DashMap<u32, oneshot::Sender<ResponseMakingResult>> {
    RESPONSE_WAITERS.get_or_init(Default::default)
}

static ERROR_FROM_JS_TXS: OnceLock<DashMap<u32, oneshot::Sender<String>>> = OnceLock::new();
fn error_from_js_txs() -> &'static DashMap<u32, oneshot::Sender<String>> {
    ERROR_FROM_JS_TXS.get_or_init(Default::default)
}

struct ResBody {
    instance: ResponseBodyInstance,
    rx: UnboundedReceiver<ResBodyChunk>,
}

struct ResponseBodyInstance {
    fetch_id: u32,
}

impl Drop for ResponseBodyInstance {
    fn drop(&mut self) {
        res_body_txs().remove(&self.fetch_id);
    }
}

impl ResponseBody for ResBody {
    async fn bytes(
        mut self,
        content_length: Option<usize>,
    ) -> std::result::Result<Vec<u8>, HttpError> {
        if let Some(content_length) = content_length {
            let mut bytes = Vec::with_capacity(content_length);
            while bytes.len() < content_length {
                let chunk = self.rx.recv().await.ok_or(HttpError::Disconnected)??;
                bytes.extend_from_slice(&chunk);
            }
            if bytes.len() > content_length {
                todo!("cancel fetch");
                // res_body_txs().remove(&self.instance.fetch_id);
                // return Err(HttpError::TooManyBytes);
            }
            Ok(bytes)
        } else {
            let mut bytes = Vec::new();
            while let Some(chunk) = self.rx.recv().await {
                bytes.extend_from_slice(&chunk?);
            }
            Ok(bytes)
        }
    }

    fn stream(
        self,
    ) -> impl futures::Stream<Item = std::result::Result<bytes::Bytes, HttpError>>
    + std::marker::Send
    + Unpin {
        ResStreamBody {
            _instance: self.instance,
            stream: tokio_stream::wrappers::UnboundedReceiverStream::new(self.rx),
        }
    }
}

struct ResStreamBody {
    _instance: ResponseBodyInstance,
    stream: tokio_stream::wrappers::UnboundedReceiverStream<ResBodyChunk>,
}

impl futures::Stream for ResStreamBody {
    type Item = std::result::Result<bytes::Bytes, HttpError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx)
    }
}

pub(crate) fn http_fetch_on_error(fetch_id: u32, message: String) {
    eprintln!("http_fetch_on_error: {message}");
    if let Some((_, tx)) = error_from_js_txs().remove(&fetch_id) {
        let _ = tx.send(message.clone());
    };

    if let Some((_, res_body_tx)) = res_body_txs().remove(&fetch_id) {
        let _ = res_body_tx.send(Err(HttpError::Unknown(format!(
            "http_fetch_on_error: {message}"
        ))));
    }
}
