use bytes::Bytes;
use dashmap::DashMap;
use std::sync::OnceLock;

static SENT_TO_JS: OnceLock<DashMap<usize, Bytes>> = OnceLock::new();

const K32: usize = 32 * 1024;

pub(crate) fn send_new_buffer_to_js() {
    let bytes = Bytes::from(vec![0; K32]);

    let ptr = bytes.as_ptr() as usize;
    SENT_TO_JS
        .get_or_init(Default::default)
        .insert(ptr, bytes.clone());

    unsafe {
        _buffer_pool_new_buffer(ptr as *const u8, bytes.len());
    }
}

pub(crate) fn take_buffer_from_js(ptr: *const u8) -> Bytes {
    SENT_TO_JS.get().unwrap().remove(&(ptr as usize)).unwrap().1
}

// # data callback protocol
// [data byte length: u16][message data: ...]
unsafe extern "C" {
    fn _buffer_pool_new_buffer(buffer_ptr: *const u8, buffer_len: usize);
}
