use std::sync::Arc;
use uuid::Uuid;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Clone, serde::Serialize)]
pub struct File {
    id: Uuid,
    #[serde(skip_serializing)]
    inner: Arc<web_sys::File>,
}
unsafe impl Send for File {}
unsafe impl Sync for File {}

impl std::hash::Hash for File {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::ptr_eq(&self.inner, &self.inner).hash(state);
    }
}

impl std::cmp::PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl std::cmp::Eq for File {}

impl File {
    pub(crate) fn new(inner: web_sys::File) -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: Arc::new(inner),
        }
    }
    pub fn name(&self) -> String {
        self.inner.name()
    }
    pub async fn content(&self) -> Box<[u8]> {
        let array_buffer = JsFuture::from(self.inner.array_buffer()).await.unwrap();
        let typed_array = js_sys::Uint8Array::new(&array_buffer);
        typed_array.to_vec().into_boxed_slice()
    }
}
