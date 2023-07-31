mod app;
mod color;
// mod components;
mod late_init;
mod pages;
mod setting;
// mod share_preview;
mod storage;
// mod viewer;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

static SETTING: late_init::LateInit<setting::Setting> =
    late_init::LateInit::<setting::Setting>::new();
static RPC: late_init::LateInit<rpc::Rpc> = late_init::LateInit::<rpc::Rpc>::new();

pub async fn main() {
    let namui_context = namui::init();

    let search = namui::web::location_search();
    let is_auth_callback = search.starts_with("?code=");
    if is_auth_callback {
        return;
    }

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

    // let share_preview = share_preview::SharePreview::from_search(&search);

    // match share_preview {
    //     Some(share_preview) => {
    //         todo!()
    //         // namui::start(
    //         //     namui_context,
    //         //     &mut viewer::Viewer::new(share_preview.sequence_id, share_preview.index),
    //         //     &(),
    //         // )
    //     }
    //     None => namui_context.start(&app::App),
    // }

    namui_context.start(&app::App);
}
