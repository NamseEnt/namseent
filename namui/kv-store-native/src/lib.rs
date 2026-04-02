use std::sync::{mpsc, LazyLock};

enum Request {
    Get { request_id: u32, key: String },
    Put { request_id: u32, key: String, value: Option<Vec<u8>> },
}

static SENDER: LazyLock<mpsc::Sender<Request>> = LazyLock::new(|| {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || worker_thread(rx));
    tx
});

fn worker_thread(rx: mpsc::Receiver<Request>) {
    let conn = rusqlite::Connection::open("namui_kv_store.db")
        .expect("Failed to open kv_store database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS kv_store (key TEXT PRIMARY KEY, value BLOB NOT NULL)",
        [],
    )
    .expect("Failed to create kv_store table");

    for req in rx {
        match req {
            Request::Get { request_id, key } => {
                let result: Option<Vec<u8>> = conn
                    .query_row(
                        "SELECT value FROM kv_store WHERE key = ?1",
                        [&key],
                        |row| row.get(0),
                    )
                    .ok();

                match result {
                    Some(data) => unsafe {
                        _on_kv_store_get_response(
                            request_id,
                            1,
                            data.as_ptr(),
                            data.len() as u32,
                        );
                    },
                    None => unsafe {
                        _on_kv_store_get_response(request_id, 0, std::ptr::null(), 0);
                    },
                }
            }
            Request::Put { request_id, key, value } => {
                match value {
                    Some(v) => {
                        conn.execute(
                            "INSERT OR REPLACE INTO kv_store (key, value) VALUES (?1, ?2)",
                            rusqlite::params![key, v],
                        )
                        .expect("Failed to put kv_store entry");
                    }
                    None => {
                        conn.execute(
                            "DELETE FROM kv_store WHERE key = ?1",
                            [&key],
                        )
                        .expect("Failed to delete kv_store entry");
                    }
                }
                unsafe {
                    _on_kv_store_put_response(request_id);
                }
            }
        }
    }
}

unsafe extern "C" {
    fn _on_kv_store_get_response(request_id: u32, has_data: u32, ptr: *const u8, len: u32);
    fn _on_kv_store_put_response(request_id: u32);
}

#[unsafe(no_mangle)]
pub extern "C" fn _kv_store_get(request_id: u32, key_ptr: *const u8, key_len: u32) {
    let key = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(key_ptr, key_len as usize))
    }
    .to_string();
    SENDER.send(Request::Get { request_id, key }).ok();
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
    SENDER.send(Request::Put { request_id, key, value }).ok();
}
