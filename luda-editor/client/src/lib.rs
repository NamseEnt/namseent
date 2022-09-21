mod app;
mod components;
mod late_init;
mod pages;
mod setting;
mod storage;
mod sync;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub async fn main() {
    let is_auth_callback = web_sys::window()
        .unwrap()
        .location()
        .search()
        .unwrap()
        .is_empty()
        == false;
    if is_auth_callback {
        // It's auth callback
        return;
    }

    let namui_context = namui::init().await;

    let setting = {
        match namui::file::bundle::read("setting.json").await {
            Ok(buffer) => serde_json::from_slice::<setting::Setting>(buffer.as_ref())
                .expect("Failed to parse setting.json"),
            Err(error) => {
                if let namui::file::bundle::ReadError::FileNotFound(_) = error {
                    setting::Setting::default()
                } else {
                    panic!("fail to read setting.json, {}", error);
                }
            }
        }
    };

    SETTING.init(setting);
    RPC.init(rpc::Rpc::new(SETTING.rpc_endpoint.clone()));

    namui::start(namui_context, &mut app::App::new(), &()).await
}

static SETTING: late_init::LateInit<setting::Setting> =
    late_init::LateInit::<setting::Setting>::new();
static RPC: late_init::LateInit<rpc::Rpc> = late_init::LateInit::<rpc::Rpc>::new();
