use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, MutexGuard};

static STORE: LazyLock<Mutex<HashMap<String, Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static ISOLATION_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

#[allow(dead_code)]
pub struct IsolationGuard(MutexGuard<'static, ()>);

pub fn lock_and_clear() -> IsolationGuard {
    let guard = ISOLATION_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    clear();
    IsolationGuard(guard)
}

pub fn clear() {
    STORE
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clear();
}

unsafe extern "C" {
    fn _on_kv_store_get_response(request_id: u32, has_data: u32, ptr: *const u8, len: u32);
    fn _on_kv_store_put_response(request_id: u32);
}

#[unsafe(no_mangle)]
pub extern "C" fn _kv_store_get(request_id: u32, key_ptr: *const u8, key_len: u32) {
    let key = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(key_ptr, key_len as usize))
    };
    let value = STORE
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(key)
        .cloned();

    match value {
        Some(value) => unsafe {
            _on_kv_store_get_response(request_id, 1, value.as_ptr(), value.len() as u32);
        },
        None => unsafe {
            _on_kv_store_get_response(request_id, 0, std::ptr::null(), 0);
        },
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _kv_store_put(
    request_id: u32,
    key_ptr: *const u8,
    key_len: u32,
    value_ptr: *const u8,
    value_len: u32,
) {
    let key = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(key_ptr, key_len as usize))
    }
    .to_string();
    let value = if value_ptr.is_null() {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(value_ptr, value_len as usize) }.to_vec())
    };

    let mut store = STORE.lock().unwrap_or_else(|error| error.into_inner());
    match value {
        Some(value) => {
            store.insert(key, value);
        }
        None => {
            store.remove(&key);
        }
    }
    drop(store);

    unsafe {
        _on_kv_store_put_response(request_id);
    }
}
