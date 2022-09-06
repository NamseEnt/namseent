use super::*;
use std::{hash::Hash, sync::Arc};

unsafe impl Sync for CanvasKitTypeface {}
unsafe impl Send for CanvasKitTypeface {}
#[derive(Clone)]
pub struct Typeface {
    pub id: u64,
    pub canvas_kit_typeface: Arc<CanvasKitTypeface>,
}
impl Typeface {
    pub fn new(bytes: &impl AsRef<[u8]>) -> Typeface {
        let bytes = bytes.as_ref();
        let id = bytes.iter().fold(0, |acc, x| acc + *x as u64);

        let array_buffer = js_sys::ArrayBuffer::new(bytes.len() as u32);

        let array_buffer_view = js_sys::Uint8Array::new(&array_buffer);
        array_buffer_view.copy_from(bytes);

        let typeface = canvas_kit()
            .FontMgr()
            .RefDefault()
            .MakeTypefaceFromData(array_buffer);

        Typeface {
            id,
            canvas_kit_typeface: Arc::new(typeface),
        }
    }
}
impl Drop for Typeface {
    fn drop(&mut self) {
        self.canvas_kit_typeface.delete();
    }
}
impl Hash for Typeface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for Typeface {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Typeface {}
