use namui_type::RingBuffer;
use std::sync::{atomic::AtomicBool, Arc};

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
pub async fn insert_js<OnData: FnMut(&[u8]) + 'static + Send>(
    js: impl ToString,
    on_data: Option<OnData>,
) -> JsHandle {
    let js = js.to_string();
    let dropped = Arc::new(AtomicBool::new(false));

    let Some(mut on_data) = on_data else {
        return JsHandle::WithoutOnData {
            id: tokio::task::spawn_blocking(move || unsafe { _insert_js(js.as_ptr(), js.len()) })
                .await
                .unwrap(),
        };
    };

    std::thread::spawn({
        let dropped = dropped.clone();
        move || {
            let mut buffer = RingBuffer::new(1024 * 1024);

            let id = unsafe {
                _insert_js_with_data_callback(js.as_ptr(), js.len(), buffer.ptr(), buffer.size())
            };

            while !dropped.load(std::sync::atomic::Ordering::Relaxed) {
                if unsafe { _insert_js_data_poll(33) } == 0 {
                    continue;
                }
                let data_byte_length = buffer.read_u16() as usize;
                let data = buffer.read_bytes(data_byte_length);
                on_data(&data);
                unsafe { _insert_js_data_commit(buffer.take_read_count()) }
            }

            unsafe { _insert_js_drop(id) }
        }
    });
    JsHandle::WithOnData { dropped }
}

pub enum JsHandle {
    WithoutOnData { id: u32 },
    WithOnData { dropped: Arc<AtomicBool> },
}

impl Drop for JsHandle {
    fn drop(&mut self) {
        match self {
            JsHandle::WithoutOnData { id } => {
                unsafe { _insert_js_drop(*id) };
            }
            JsHandle::WithOnData { dropped } => {
                dropped.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }
}

pub(crate) async fn hardware_concurrency() -> u32 {
    tokio::task::spawn_blocking(|| unsafe { _hardware_concurrency() })
        .await
        .unwrap()
}

// # data callback protocol
// [data byte length: u16][message data: ...]
extern "C" {
    fn _insert_js(js_ptr: *const u8, js_len: usize) -> u32;
    fn _insert_js_with_data_callback(
        js_ptr: *const u8,
        js_len: usize,
        ring_buffer_ptr: *const u8,
        ring_buffer_len: usize,
    ) -> u32;
    fn _insert_js_drop(js_id: u32);
    fn _insert_js_data_poll(timeout_ms: u32) -> u32;
    fn _insert_js_data_commit(byte_length: usize);
    fn _hardware_concurrency() -> u32;
}
