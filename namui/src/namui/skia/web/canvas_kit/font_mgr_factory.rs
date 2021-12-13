use super::*;

#[wasm_bindgen]
extern "C" {
    pub type FontMgrFactory;

    ///
    /// Return the default FontMgr. This will generally have 0 or 1 fonts in it, depending on if
    /// the demo monospace font was compiled in.
    ///

    #[wasm_bindgen(structural, method)]
    pub fn RefDefault(this: &FontMgrFactory) -> FontMgr;
}
