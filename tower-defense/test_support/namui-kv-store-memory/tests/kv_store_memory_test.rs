use std::sync::{LazyLock, Mutex};

static RESPONSES: LazyLock<Mutex<Vec<Response>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Debug, PartialEq, Eq)]
enum Response {
    Get {
        request_id: u32,
        has_data: u32,
        data: Vec<u8>,
    },
    Put {
        request_id: u32,
    },
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_kv_store_get_response(
    request_id: u32,
    has_data: u32,
    ptr: *const u8,
    len: u32,
) {
    let data = if has_data == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(ptr, len as usize) }.to_vec()
    };
    RESPONSES.lock().unwrap().push(Response::Get {
        request_id,
        has_data,
        data,
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_kv_store_put_response(request_id: u32) {
    RESPONSES.lock().unwrap().push(Response::Put { request_id });
}

fn take_responses() -> Vec<Response> {
    std::mem::take(&mut *RESPONSES.lock().unwrap())
}

fn call_get(request_id: u32, key: &str) {
    namui_kv_store_memory::_kv_store_get(request_id, key.as_ptr(), key.len() as u32);
}

fn call_put(request_id: u32, key: &str, value: Option<&[u8]>) {
    match value {
        Some(value) => namui_kv_store_memory::_kv_store_put(
            request_id,
            key.as_ptr(),
            key.len() as u32,
            value.as_ptr(),
            value.len() as u32,
        ),
        None => namui_kv_store_memory::_kv_store_put(
            request_id,
            key.as_ptr(),
            key.len() as u32,
            std::ptr::null(),
            0,
        ),
    }
}

#[test]
fn stores_reads_overwrites_and_deletes_values() {
    let _guard = namui_kv_store_memory::lock_and_clear();

    call_get(1, "missing");
    call_put(2, "key", Some(b"value"));
    call_get(3, "key");
    call_put(4, "key", Some(b"updated"));
    call_get(5, "key");
    call_put(6, "key", None);
    call_get(7, "key");
    call_put(8, "empty", Some(&[]));
    call_get(9, "empty");

    assert_eq!(
        take_responses(),
        vec![
            Response::Get {
                request_id: 1,
                has_data: 0,
                data: Vec::new(),
            },
            Response::Put { request_id: 2 },
            Response::Get {
                request_id: 3,
                has_data: 1,
                data: b"value".to_vec(),
            },
            Response::Put { request_id: 4 },
            Response::Get {
                request_id: 5,
                has_data: 1,
                data: b"updated".to_vec(),
            },
            Response::Put { request_id: 6 },
            Response::Get {
                request_id: 7,
                has_data: 0,
                data: Vec::new(),
            },
            Response::Put { request_id: 8 },
            Response::Get {
                request_id: 9,
                has_data: 1,
                data: Vec::new(),
            },
        ]
    );
}
