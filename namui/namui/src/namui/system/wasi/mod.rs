pub fn insert_js(js: impl AsRef<str>) -> JsHandle {
    let js = js.as_ref();
    let id = unsafe { _insert_js(js.as_ptr(), js.len()) };
    JsHandle { id }
}

pub struct JsHandle {
    id: u32,
}

impl Drop for JsHandle {
    fn drop(&mut self) {
        unsafe { _drop_js(self.id) };
    }
}

extern "C" {
    fn _insert_js(js_ptr: *const u8, js_len: usize) -> u32;
    fn _drop_js(js_id: u32);
}
