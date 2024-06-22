mod data_fetch;
mod home;
mod network;
mod new_team_page;
mod router;
mod simple_button;
mod toast;

use data_fetch::*;
use namui::*;
use namui_prebuilt::{table::*, *};
use simple_button::*;

static SERVER_CONNECTION_ATOM: Atom<ServerConnection> = Atom::uninitialized();

pub fn main() {
    namui::start(|ctx| {
        if !SERVER_CONNECTION_ATOM.is_initialized() {
            ctx.add(Login);
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

struct Login;
impl Component for Login {
    fn render(self, ctx: &RenderCtx) {
        let (error, set_error) = ctx.state(|| None::<String>);

        ctx.effect("Insert gsi html api", || {
            let set_error = set_error.cloned();
            let handle = tokio::spawn(async move {
                let jwt = take_google_gsi_jwt().await;
                let connection = match connect_to_server(jwt).await {
                    Ok(connection) => connection,
                    Err(err) => {
                        set_error.set(Some(format!("Failed to connect to server: {}", err)));
                        return;
                    }
                };
                SERVER_CONNECTION_ATOM.set(connection);
            });

            move || handle.abort()
        });

        if let Some(error) = error.as_ref() {
            ctx.add(typography::center_text(
                namui::screen::size().map(|x| x.into_px()),
                error,
                Color::RED,
                20.int_px(),
            ));
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ServerConnection {
    sender: namui::network::ws::WsSender,
    // receiver: namui::network::ws::WsReceiver,
}
impl ServerConnection {
    pub async fn request<Response, Error>(
        &self,
        request_packet: RequestPacket,
    ) -> Result<Response, Error> {
        todo!()
    }
}
trait Request {
    fn as_packet(&self) -> RequestPacket;
}

struct RequestPacket {}

async fn connect_to_server(jwt: String) -> Result<ServerConnection> {
    let (sender, mut receiver) = namui::network::ws::connect("ws://localhost:8080").await?;

    sender.send(jwt.as_bytes());

    let data = receiver
        .recv()
        .await
        .ok_or(anyhow!("Failed to get auth response"))?;

    Ok(ServerConnection {
        sender,
        // receiver,
    })
}

async fn take_google_gsi_jwt() -> String {
    let (data_tx, mut data_rx) = tokio::sync::mpsc::unbounded_channel();

    let js_handle = namui::wasi::insert_js(
        include_str!("login.js"),
        Some(move |data: &[u8]| {
            let str = std::str::from_utf8(data).unwrap();
            data_tx.send(str.to_string()).unwrap();
        }),
    )
    .await;

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
