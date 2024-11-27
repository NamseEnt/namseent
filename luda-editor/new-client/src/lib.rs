mod asset_manage_page;
mod audio_util;
mod episode_editor;
mod home;
mod network;
mod new_episode_page;
mod new_id;
mod new_project_page;
mod new_team_page;
mod psd_sprite_util;
mod router;
mod rpc;
mod simple_button;
mod toast;

use namui::*;
use namui_prebuilt::{table::*, *};
use network::*;
use simple_button::*;

pub fn main() {
    namui::start(|ctx| {
        ctx.effect("init server connection", || {
            ServerConnection::init("ws://localhost:8080/ws")
        });
        let (logged_in, set_logged_in) = ctx.state(|| false);
        if !*logged_in {
            ctx.add(Login { set_logged_in });
            return;
        }

        let screen_wh = namui::screen::size().map(|x| x.into_px());

        ctx.add(toast::Toast);
        ctx.add(router::Router);
        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::grayscale_f01(0.8),
        ));
    });
}

struct Login {
    set_logged_in: SetState<bool>,
}
impl Component for Login {
    fn render(self, ctx: &RenderCtx) {
        let Self { set_logged_in } = self;
        let (error, set_error) = ctx.state(|| None::<String>);

        const KV_STORE_SESSION_TOKEN_KEY: &str = "session_token";

        ctx.effect("Insert gsi html api", || {
            ctx.spawn(async move {
                let result: Result<()> = async move {
                    let session_token =
                        namui::system::file::kv_store::get(KV_STORE_SESSION_TOKEN_KEY)?;

                    println!("Session token: {:?}", session_token);

                    if let Some(session_token) = session_token {
                        let session_token_string = String::from_utf8(session_token).unwrap();
                        let session_token = session_token_string.parse::<u128>()?;

                        use rpc::auth::session_token_auth::*;
                        match server_connection()
                            .session_token_auth(RefRequest { session_token })
                            .await
                        {
                            Ok(_) => {
                                return Ok(());
                            }
                            Err(err) => match err {
                                Error::AlreadyLoggedIn => {
                                    return Ok(());
                                }
                                Error::SessionTokenNotExists => {
                                    // ok, let's continue below.
                                }
                                Error::InternalServerError { err } => {
                                    return Err(anyhow!(
                                        "session_token_auth | Internal server error: {err}"
                                    ));
                                }
                            },
                        }
                    }

                    let jwt = take_google_gsi_jwt().await;
                    {
                        use rpc::auth::google_auth::*;
                        match server_connection()
                            .google_auth(RefRequest { jwt: &jwt })
                            .await
                        {
                            Ok(response) => {
                                namui::system::file::kv_store::set(
                                    KV_STORE_SESSION_TOKEN_KEY,
                                    response.session_token.to_string().as_bytes(),
                                )?;
                                Ok(())
                            }
                            Err(err) => match err {
                                Error::AlreadyLoggedIn => Ok(()),
                                Error::InternalServerError { err } => {
                                    Err(anyhow!("google_auth | Internal server error: {err}"))
                                }
                            },
                        }
                    }
                }
                .await;

                match result {
                    Ok(_) => {
                        set_logged_in.set(true);
                    }
                    Err(err) => {
                        set_error.set(Some(err.to_string()));
                    }
                }
            });
        });

        let (color, text) = if let Some(error) = error.as_ref() {
            (Color::RED, error.to_string())
        } else {
            (Color::BLACK, "login...".to_string())
        };
        let wh = namui::screen::size().map(|x| x.into_px());
        ctx.add(namui::text(TextParam {
            text,
            x: wh.width / 2.0,
            y: wh.height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font: Font {
                name: "NotoSansKR-Regular".to_string(),
                size: 20.int_px(),
            },
            style: TextStyle {
                color,
                ..Default::default()
            },
            max_width: Some(wh.width),
        }));
    }
}

async fn take_google_gsi_jwt() -> String {
    let (data_tx, mut data_rx) = tokio::sync::mpsc::unbounded_channel();

    let js_handle = namui::wasi::insert_js(
        include_str!("login.js"),
        Some(move |data: &[u8]| {
            let str = std::str::from_utf8(data).unwrap();
            data_tx.send(str.to_string()).unwrap();
        }),
    );

    let data = data_rx.recv().await.unwrap();

    #[derive(serde::Deserialize)]
    struct GsiResponse {
        /// jwt
        credential: String,
    }

    let response = serde_json::from_str::<GsiResponse>(&data).unwrap();

    drop(js_handle);

    response.credential
}
