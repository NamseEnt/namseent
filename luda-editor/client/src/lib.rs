mod app;
mod components;
mod late_init;
mod pages;
mod setting;
mod storage;
mod viewer;

use namui::prelude::*;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

static SETTING: late_init::LateInit<setting::Setting> =
    late_init::LateInit::<setting::Setting>::new();
static RPC: late_init::LateInit<rpc::Rpc> = late_init::LateInit::<rpc::Rpc>::new();

pub async fn main() {
    let search = web_sys::window().unwrap().location().search().unwrap();
    let is_auth_callback = search.starts_with("?code=");
    if is_auth_callback {
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

    let view_request = parse_view_request(&search);

    match view_request {
        Some(view_request) => {
            namui::start(
                namui_context,
                &mut viewer::Viewer::new(view_request.sequence_id),
                &(),
            )
            .await
        }
        None => namui::start(namui_context, &mut app::App::new(), &()).await,
    }
}

struct ViewRequest {
    sequence_id: Uuid,
}
fn parse_view_request(search: &str) -> Option<ViewRequest> {
    if search.len() < 2 {
        return None;
    }

    let query_tuples = search[1..].split('&').map(|s| {
        let mut iter = s.split('=');
        let key = iter.next().unwrap_or_default();
        let value = iter.next().unwrap_or_default();
        (key, value)
    });
    let mut sequence_id = None;
    let mut is_view_request = false;
    for (key, value) in query_tuples {
        if key == "sequence_id" {
            sequence_id = Some(value);
        } else if key == "view" {
            is_view_request = true;
        }
    }

    if is_view_request && sequence_id.is_some() {
        Some(ViewRequest {
            sequence_id: sequence_id.unwrap().parse().unwrap(),
        })
    } else {
        None
    }
}
