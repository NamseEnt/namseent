use super::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct CkTypeface {
    pub canvas_kit_typeface: Arc<CanvasKitTypeface>,
}

static TYPEFACE_MAP: StaticHashMap<String, CkTypeface> = StaticHashMap::new();

impl CkTypeface {
    pub(crate) fn get(name: impl AsRef<str>) -> Option<Arc<Self>> {
        TYPEFACE_MAP.get(&name.as_ref().to_string())
    }
    pub(crate) fn load(name: impl AsRef<str>, bytes: &[u8]) -> Result<()> {
        let array_buffer = js_sys::ArrayBuffer::new(bytes.len() as u32);

        let array_buffer_view = js_sys::Uint8Array::new(&array_buffer);
        array_buffer_view.copy_from(bytes);

        let typeface = canvas_kit()
            .Typeface()
            .MakeFreeTypeFaceFromData(array_buffer);

        TYPEFACE_MAP.insert(
            name.as_ref().to_string(),
            CkTypeface {
                canvas_kit_typeface: Arc::new(typeface),
            },
        );

        Ok(())
    }
    pub fn canvas_kit(&self) -> &CanvasKitTypeface {
        &self.canvas_kit_typeface
    }
}
impl Drop for CkTypeface {
    fn drop(&mut self) {
        self.canvas_kit_typeface.delete();
    }
}
