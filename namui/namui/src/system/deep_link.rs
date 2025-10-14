use super::InitResult;
use namui_cfg::namui_cfg;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
unsafe extern "C" {
    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "deepLink"], js_name = getRecentlyOpenedUrl)]
    fn get_recently_opened_url_() -> Option<String>;

    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "deepLink"], js_name = addDeepLinkOpenedEventListener)]
    fn add_deep_link_opened_event_listener(callback: &js_sys::Function);
}

#[namui_cfg(target_env = "electron")]
pub(crate) fn init() -> InitResult {
    use crate::{DeepLinkOpenedEvent, NamuiEvent};
    use wasm_bindgen::{JsCast, prelude::Closure};
    let callback = Closure::wrap(Box::new(|url: String| {
        crate::event::send(NamuiEvent::DeepLinkOpened(DeepLinkOpenedEvent { url }));
    }) as Box<dyn Fn(String)>);
    add_deep_link_opened_event_listener(callback.into_js_value().unchecked_ref());
    Ok(())
}

#[namui_cfg(not(target_env = "electron"))]
pub(crate) fn init() -> InitResult {
    Ok(())
}

#[namui_cfg(target_env = "electron")]
pub fn get_recently_opened_url() -> Option<String> {
    get_recently_opened_url_()
}
