use bytes::Bytes;
use dashmap::DashMap;
use std::sync::{OnceLock, atomic::AtomicUsize};
use tokio::sync::mpsc::UnboundedSender;

type JsId = usize;
type RequestId = usize;
static DATA_BUFFER_MAP: OnceLock<DashMap<(JsId, RequestId), Bytes>> = OnceLock::new();
static DATA_TX_MAP: OnceLock<DashMap<JsId, UnboundedSender<Bytes>>> = OnceLock::new();

/// Insert JavaScript code into the WASI runtime.
///
/// You should keep returned JsHandle to keep running the JavaScript code.
///
/// ### Special function on js
/// - `namui_sendData(data: ArrayBuffer)`
///     - Send data to the WASI runtime.
///     - CAUTION: data must not be TypedArray. It must be ArrayBuffer.
///     - example) `const buffer = new Uint8Array(...); namui_sendData(buffer.buffer);`
/// - `namui_onDrop()`
///    - Called when the handle is dropped. You might implement cleanup logic here.
///    - example) `const namui_onDrop = () => { ...remove event listener... }`
pub fn insert_js<OnData: FnMut(&[u8]) + 'static + Send>(
    js: impl ToString,
    on_data: Option<OnData>,
) -> JsHandle {
    let js = js.to_string();
    let js_id = {
        static JS_ID: AtomicUsize = AtomicUsize::new(0);
        JS_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    };

    if let Some(mut on_data) = on_data {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        DATA_TX_MAP.get_or_init(Default::default).insert(js_id, tx);
        tokio::task::spawn(async move {
            while let Some(data) = rx.recv().await {
                on_data(&data);
            }
        });
    }

    let js = js.as_bytes();
    unsafe {
        _insert_js(js.as_ptr(), js.len(), js_id);
    }

    JsHandle {
        js_id,
        send_data_next_id: Default::default(),
    }
}

pub struct JsHandle {
    js_id: usize,
    send_data_next_id: AtomicUsize,
}

impl JsHandle {
    /// Sending data to js is guaranteed to be ordered.
    pub fn send_data(&self, data: impl AsRef<[u8]>) {
        let send_data_id = self
            .send_data_next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let data = data.as_ref();
        unsafe {
            _insert_js_send_data_from_rust(self.js_id, send_data_id, data.as_ptr(), data.len());
        }
    }
}

impl Drop for JsHandle {
    fn drop(&mut self) {
        unsafe {
            _insert_js_drop(self.js_id);
        }
        DATA_TX_MAP.get().unwrap().remove(&self.js_id);
        DATA_BUFFER_MAP
            .get()
            .unwrap()
            .retain(|key, _| key.0 != self.js_id);
    }
}

unsafe extern "C" {
    fn _insert_js(js_ptr: *const u8, js_len: usize, js_id: usize) -> usize;
    fn _insert_js_drop(js_id: usize);
    fn _insert_js_data_buffer(js_id: usize, request_id: usize, buffer_ptr: *const u8);
    fn _insert_js_send_data_from_rust(
        js_id: usize,
        send_data_id: usize,
        buffer_ptr: *const u8,
        buffer_len: usize,
    );
}

pub(crate) fn on_request_data_buffer(js_id: usize, request_id: usize, buffer_len: usize) {
    let buffer = Bytes::from(vec![0; buffer_len]);
    let buffer_ptr = buffer.as_ptr();

    DATA_BUFFER_MAP
        .get_or_init(Default::default)
        .insert((js_id, request_id), buffer);

    unsafe {
        _insert_js_data_buffer(js_id, request_id, buffer_ptr);
    }
}

pub(crate) fn on_data(js_id: usize, request_id: usize) {
    let Some((_, buffer)) = DATA_BUFFER_MAP.get().unwrap().remove(&(js_id, request_id)) else {
        return;
    };
    let Some(tx) = DATA_TX_MAP.get().unwrap().get(&js_id) else {
        return;
    };
    let _ = tx.send(buffer);
}
