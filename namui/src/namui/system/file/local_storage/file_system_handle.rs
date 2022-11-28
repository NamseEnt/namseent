use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum FileSystemHandleKind {
    File = "file",
    Directory = "directory",
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct GetHandleOption {
    pub create: bool,
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends=js_sys::Object, js_name=FileSystemHandle)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle)"]
    pub type FileSystemHandle;

    # [wasm_bindgen (method, getter, js_class="FileSystemHandle", js_name=kind)]
    #[doc = "The `kind` properties."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/kind)"]
    pub fn kind(this: &FileSystemHandle) -> FileSystemHandleKind;

    # [wasm_bindgen (method, getter, js_class="FileSystemHandle", js_name=name)]
    #[doc = "The `name` properties."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/name)"]
    pub fn name(this: &FileSystemHandle) -> String;
}
