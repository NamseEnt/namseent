use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex};
use tokio::sync::oneshot;

unsafe extern "C" {
    fn _kv_store_get(request_id: u32, key_ptr: *const u8, key_len: u32);
    fn _kv_store_put(request_id: u32, key_ptr: *const u8, key_len: u32, value_ptr: *const u8, value_len: u32);
}

static NEXT_ID: AtomicU32 = AtomicU32::new(1);
static PENDING_GET: LazyLock<Mutex<HashMap<u32, oneshot::Sender<Option<Vec<u8>>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static PENDING_PUT: LazyLock<Mutex<HashMap<u32, oneshot::Sender<()>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub async fn get(key: impl AsRef<str>) -> Option<Vec<u8>> {
    let key = key.as_ref();
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let (tx, rx) = oneshot::channel();
    PENDING_GET.lock().unwrap().insert(id, tx);
    unsafe {
        _kv_store_get(id, key.as_ptr(), key.len() as u32);
    }
    rx.await.unwrap()
}

pub async fn put(key: impl AsRef<str>, value: Option<&[u8]>) {
    let key = key.as_ref();
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let (tx, rx) = oneshot::channel();
    PENDING_PUT.lock().unwrap().insert(id, tx);
    match value {
        Some(v) => unsafe {
            _kv_store_put(id, key.as_ptr(), key.len() as u32, v.as_ptr(), v.len() as u32);
        },
        None => unsafe {
            _kv_store_put(id, key.as_ptr(), key.len() as u32, std::ptr::null(), 0);
        },
    }
    rx.await.unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_kv_store_get_response(request_id: u32, has_data: u32, ptr: *const u8, len: u32) {
    let data = if has_data != 0 {
        if len > 0 {
            Some(unsafe { std::slice::from_raw_parts(ptr, len as usize) }.to_vec())
        } else {
            Some(Vec::new())
        }
    } else {
        None
    };
    if let Some(tx) = PENDING_GET.lock().unwrap().remove(&request_id) {
        let _ = tx.send(data);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_kv_store_put_response(request_id: u32) {
    if let Some(tx) = PENDING_PUT.lock().unwrap().remove(&request_id) {
        let _ = tx.send(());
    }
}
