use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    executor::block_on,
    lock::Mutex,
    SinkExt, StreamExt,
};
use namui::build::types::ErrorMessage;
use nanoid::nanoid;
use serde_json::json;
use std::{collections::HashMap, env, path::PathBuf, sync::Arc};
use tokio::spawn;
use warp::ws;
use warp::{http::response, hyper::Uri, ws::Message, Filter};

use crate::build::bundle::Bundle;

type OnConnectedCallback = fn() -> ();

pub struct StartServerOption {
    pub port: u16,
    pub on_connected: OnConnectedCallback,
    pub bundle: Arc<Bundle>,
    pub resource_path: Option<String>,
}

pub struct WebServer {
    sockets: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl WebServer {
    pub async fn start(option: StartServerOption) -> Self {
        let web_server = WebServer {
            sockets: Arc::new(Mutex::new(HashMap::new())),
        };

        let redirect_to_index_html =
            warp::path::end().map(|| warp::redirect(Uri::from_static("index.html")));

        let bundle = option
            .bundle
            .clone();
        let serve_wasm_bundle = warp::path("bundle_bg.wasm").map(
            move || -> Result<warp::hyper::Response<Vec<u8>>, warp::http::Error> {
                let bundle = block_on(
                    bundle
                        .wasm
                        .read(),
                );

                response::Builder::new()
                    .header("Content-Type", "application/wasm")
                    .body(bundle.clone())
            },
        );

        let bundle = option
            .bundle
            .clone();
        let serve_js_bundle = warp::path("bundle.js").map(
            move || -> Result<warp::hyper::Response<Vec<u8>>, warp::http::Error> {
                let bundle = block_on(
                    bundle
                        .js
                        .read(),
                );

                response::Builder::new()
                    .header("Content-Type", "text/javascript")
                    .body(bundle.clone())
            },
        );

        let serve_engine = warp::path("engine").and(warp::fs::dir(get_engine_dir()));

        let serve_resources = match option.resource_path {
            Some(resource_path) => {
                Some(warp::path("resources").and(warp::fs::dir(PathBuf::from(resource_path))))
            }
            None => None,
        };

        let sockets = web_server
            .sockets
            .clone();
        let handle_websocket = warp::path("hotReload")
            .and(warp::ws())
            .map(move |ws: ws::Ws| {
                let sockets = sockets.clone();
                ws.on_upgrade(|websocket| async move {
                    let id = nanoid!();
                    let (sender, mut receiver) = unbounded::<Message>();
                    {
                        let mut sockets = sockets
                            .lock()
                            .await;
                        (*sockets).insert(id.clone(), sender);
                    }

                    let (mut tx, mut rx) = websocket.split();
                    spawn(async move {
                        loop {
                            match receiver
                                .next()
                                .await
                            {
                                Some(message) => {
                                    if let Err(error) = tx
                                        .send(message)
                                        .await
                                    {
                                        eprintln!("websocket send fail.\n  {:?}", error);
                                    }
                                }
                                None => {
                                    receiver.close();
                                    break;
                                }
                            }
                        }
                    });

                    loop {
                        match rx
                            .next()
                            .await
                        {
                            Some(_) => (),
                            None => break,
                        }
                    }

                    let mut sockets = sockets
                        .lock()
                        .await;
                    (*sockets).remove(&id);
                })
            });

        let serve_static = warp::get().and(warp::fs::dir(PathBuf::from(get_static_dir())));

        let routes = redirect_to_index_html
            .or(serve_engine)
            .or(serve_wasm_bundle)
            .or(serve_js_bundle)
            .or(handle_websocket);

        let _ = match serve_resources {
            Some(serve_resources) => spawn(
                warp::serve(
                    routes
                        .or(serve_resources)
                        .or(serve_static),
                )
                .run(([0, 0, 0, 0], option.port)),
            ),
            None => spawn(warp::serve(routes.or(serve_static)).run(([0, 0, 0, 0], option.port))),
        };

        web_server
    }

    pub async fn send_error_messages(&self, error_messages: &Vec<ErrorMessage>) {
        let mut sockets = self
            .sockets
            .lock()
            .await;
        for (id, socket) in sockets.iter_mut() {
            if let Err(error) = socket
                .send(Message::text(
                    json!({
                        "type": "error",
                        "errorMessages": error_messages,
                    })
                    .to_string(),
                ))
                .await
            {
                eprintln!("channel send error while(channel -> websocket:{}).\n  {:?}", id, error);
            }
        }
    }
}

fn get_cli_root_path() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn get_static_dir() -> PathBuf {
    get_cli_root_path().join("www")
}

fn get_engine_dir() -> PathBuf {
    get_cli_root_path().join("engine")
}
