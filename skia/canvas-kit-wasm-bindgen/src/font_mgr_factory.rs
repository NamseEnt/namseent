use super::*;

unsafe impl Sync for FontMgrFactory {}
unsafe impl Send for FontMgrFactory {}

#[wasm_bindgen]
extern "C" {
    pub type FontMgrFactory;
}
