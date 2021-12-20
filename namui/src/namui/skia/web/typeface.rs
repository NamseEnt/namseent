use super::*;
use std::sync::Arc;

unsafe impl Sync for CanvasKitTypeface {}
unsafe impl Send for CanvasKitTypeface {}
pub struct Typeface(pub Arc<CanvasKitTypeface>);
impl Typeface {
    pub fn new(bytes: &Vec<u8>) -> Typeface {
        let array_buffer = js_sys::ArrayBuffer::new(bytes.len() as u32);

        let array_buffer_view = js_sys::Uint8Array::new(&array_buffer);
        array_buffer_view.copy_from(bytes);

        let typeface = canvas_kit()
            .FontMgr()
            .RefDefault()
            .MakeTypefaceFromData(array_buffer);

        Typeface(Arc::new(typeface))
    }
}
impl Drop for Typeface {
    fn drop(&mut self) {
        self.0
            .delete();
    }
}
