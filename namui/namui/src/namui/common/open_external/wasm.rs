use anyhow::Result;
use namui_cfg::namui_cfg;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi"], js_name = openExternal)]
    fn open_external_(url: &str);
}
#[namui_cfg(target_env = "electron")]
pub fn open_external(url: &str) -> Result<()> {
    open_external_(url);
    Ok(())
}

#[namui_cfg(not(target_env = "electron"))]
pub fn open_external(url: &str) -> Result<()> {
    web_sys::window()
        .unwrap()
        .open_with_url_and_target(url, "_blank")
        .map_err(|_| anyhow::anyhow!("Failed to open url: {}", url))?;
    Ok(())
}
