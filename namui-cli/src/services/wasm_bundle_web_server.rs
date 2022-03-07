pub struct WasmBundleWebServer {}
impl WasmBundleWebServer {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn request_reload(&self) {
        println!("request_reload!");
    }
}
