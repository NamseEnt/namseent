use std::sync::{mpsc, LazyLock, Mutex};
use std::time::Duration;

// --- Mock callback infrastructure ---

struct GetResponse {
    request_id: u32,
    data: Option<Vec<u8>>,
}

struct PutResponse {
    request_id: u32,
}

static GET_CHANNEL: LazyLock<(mpsc::Sender<GetResponse>, Mutex<mpsc::Receiver<GetResponse>>)> =
    LazyLock::new(|| {
        let (tx, rx) = mpsc::channel();
        (tx, Mutex::new(rx))
    });

static PUT_CHANNEL: LazyLock<(mpsc::Sender<PutResponse>, Mutex<mpsc::Receiver<PutResponse>>)> =
    LazyLock::new(|| {
        let (tx, rx) = mpsc::channel();
        (tx, Mutex::new(rx))
    });

#[unsafe(no_mangle)]
pub extern "C" fn _on_kv_store_get_response(
    request_id: u32,
    has_data: u32,
    ptr: *const u8,
    len: u32,
) {
    let data = if has_data != 0 {
        Some(unsafe { std::slice::from_raw_parts(ptr, len as usize) }.to_vec())
    } else {
        None
    };
    GET_CHANNEL
        .0
        .send(GetResponse { request_id, data })
        .unwrap();
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_kv_store_put_response(request_id: u32) {
    PUT_CHANNEL.0.send(PutResponse { request_id }).unwrap();
}

// --- Helpers ---

const TIMEOUT: Duration = Duration::from_secs(5);

fn wait_get() -> GetResponse {
    GET_CHANNEL
        .1
        .lock()
        .unwrap()
        .recv_timeout(TIMEOUT)
        .expect("Timed out waiting for get response")
}

fn wait_put() -> PutResponse {
    PUT_CHANNEL
        .1
        .lock()
        .unwrap()
        .recv_timeout(TIMEOUT)
        .expect("Timed out waiting for put response")
}

fn call_get(request_id: u32, key: &str) {
    namui_kv_store_native::_kv_store_get(request_id, key.as_ptr(), key.len() as u32);
}

fn call_put(request_id: u32, key: &str, value: Option<&[u8]>) {
    match value {
        Some(v) => namui_kv_store_native::_kv_store_put(
            request_id,
            key.as_ptr(),
            key.len() as u32,
            v.as_ptr(),
            v.len() as u32,
        ),
        None => namui_kv_store_native::_kv_store_put(
            request_id,
            key.as_ptr(),
            key.len() as u32,
            std::ptr::null(),
            0,
        ),
    }
}

// --- Tests ---

#[test]
fn kv_store_integration() {
    // Setup: use a temp dir so the SQLite DB is isolated and auto-cleaned
    let tmp = tempfile::tempdir().expect("Failed to create temp dir");
    std::env::set_current_dir(tmp.path()).expect("Failed to set CWD to temp dir");

    // 1. Get nonexistent key → has_data=0
    call_get(1, "missing_key");
    let resp = wait_get();
    assert_eq!(resp.request_id, 1);
    assert!(resp.data.is_none(), "Expected None for nonexistent key");

    // 2. Put key-value → put callback with correct request_id
    call_put(2, "hello", Some(b"world"));
    let resp = wait_put();
    assert_eq!(resp.request_id, 2);

    // 3. Get existing key → has_data=1, data matches
    call_get(3, "hello");
    let resp = wait_get();
    assert_eq!(resp.request_id, 3);
    assert_eq!(resp.data.as_deref(), Some(b"world".as_slice()));

    // 4. Overwrite key → get returns new value
    call_put(4, "hello", Some(b"updated"));
    let resp = wait_put();
    assert_eq!(resp.request_id, 4);

    call_get(5, "hello");
    let resp = wait_get();
    assert_eq!(resp.request_id, 5);
    assert_eq!(resp.data.as_deref(), Some(b"updated".as_slice()));

    // 5. Delete key (put with null ptr) → get returns has_data=0
    call_put(6, "hello", None);
    let resp = wait_put();
    assert_eq!(resp.request_id, 6);

    call_get(7, "hello");
    let resp = wait_get();
    assert_eq!(resp.request_id, 7);
    assert!(resp.data.is_none(), "Expected None after delete");

    // 6. Put empty value → get returns has_data=1, len=0
    call_put(8, "empty", Some(&[]));
    let resp = wait_put();
    assert_eq!(resp.request_id, 8);

    call_get(9, "empty");
    let resp = wait_get();
    assert_eq!(resp.request_id, 9);
    assert_eq!(resp.data.as_deref(), Some([].as_slice()), "Expected empty vec for empty value");

    // 7. Multiple rapid requests → all responses received with correct request_ids
    let base_id = 100;
    let count = 20;
    for i in 0..count {
        let id = base_id + i;
        call_put(id, &format!("key_{i}"), Some(format!("value_{i}").as_bytes()));
    }
    let mut put_ids: Vec<u32> = Vec::new();
    for _ in 0..count {
        put_ids.push(wait_put().request_id);
    }
    put_ids.sort();
    let expected: Vec<u32> = (base_id..base_id + count).collect();
    assert_eq!(put_ids, expected, "All put request_ids should be received");

    for i in 0..count {
        let id = base_id + count + i;
        call_get(id, &format!("key_{i}"));
    }
    let mut get_results: Vec<(u32, Vec<u8>)> = Vec::new();
    for _ in 0..count {
        let r = wait_get();
        get_results.push((r.request_id, r.data.expect("Expected data for rapid get")));
    }
    get_results.sort_by_key(|(id, _)| *id);
    for i in 0..count {
        let (id, data) = &get_results[i as usize];
        assert_eq!(*id, base_id + count + i);
        assert_eq!(data, format!("value_{i}").as_bytes());
    }
}
