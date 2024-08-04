mod app;
mod clipboard;
mod color;
mod components;
mod late_init;
mod pages;
mod setting;
mod storage;

// TODO
// mod share_preview;
// mod viewer;

use namui::*;

static SETTING: late_init::LateInit<setting::Setting> =
    late_init::LateInit::<setting::Setting>::new();
static RPC: late_init::LateInit<rpc::Rpc> = late_init::LateInit::<rpc::Rpc>::new();

pub async fn main() {
    let namui_context = namui::init().await;

    namui_context.start(|| Init {}).await;
}

struct Init {}

impl namui::Component for Init {
    fn render(self, ctx: &RenderCtx) {
        let (loaded, set_loaded) = ctx.state(|| false);

        ctx.effect("Init", || {
            spawn_local(async move {
                let search: String =
                    namui::web::execute_function("return document.location.search;").run();

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

                // TODO
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

                set_loaded.set(true)
            })
        });

        ctx.component(loaded.then(|| app::App {}));
    }
}
