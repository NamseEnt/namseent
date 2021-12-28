use crate::build::{
    bundle::Bundle,
    types::{ErrorMessage, WebsocketMessage},
};
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    executor::block_on,
    lock::Mutex,
    SinkExt, StreamExt,
};
use nanoid::nanoid;
use std::{collections::HashMap, env::current_exe, path::PathBuf, sync::Arc};
use tokio::{spawn, sync::RwLock};
use warp::ws;
use warp::{http::response, hyper::Uri, ws::Message, Filter};

pub struct StartServerOption {
    pub port: u16,
    pub bundle: Arc<RwLock<Bundle>>,
}

pub struct WebServer {
    sockets: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
    cached_error_messages: RwLock<Vec<ErrorMessage>>,
}

impl WebServer {
    pub async fn start(option: StartServerOption) -> Arc<Self> {
        let web_server = Arc::new(WebServer {
            sockets: Arc::new(Mutex::new(HashMap::new())),
            cached_error_messages: RwLock::new(Vec::new()),
        });

        let redirect_to_index_html =
            warp::path::end().map(|| warp::redirect(Uri::from_static("index.html")));

        let bundle = option.bundle.clone();
        let serve_wasm_bundle = warp::path("bundle_bg.wasm").map(
            move || -> Result<warp::hyper::Response<Vec<u8>>, warp::http::Error> {
                let bundle = block_on(bundle.read()).wasm.clone();

                response::Builder::new()
                    .header("Content-Type", "application/wasm")
                    .body(bundle.clone())
            },
        );

        let bundle = option.bundle.clone();
        let serve_js_bundle = warp::path("bundle.js").map(
            move || -> Result<warp::hyper::Response<Vec<u8>>, warp::http::Error> {
                let bundle = block_on(bundle.read()).js.clone();

                response::Builder::new()
                    .header("Content-Type", "text/javascript")
                    .body(bundle.clone())
            },
        );

        let serve_engine = warp::path("engine").and(warp::fs::dir(get_engine_dir()));

        let web_server_clone = web_server.clone();
        let handle_websocket = warp::path("hotReload")
            .and(warp::ws())
            .map(move |ws: ws::Ws| {
                let web_server = web_server_clone.clone();
                ws.on_upgrade(|websocket| async move {
                    let id = nanoid!();
                    let (sender, mut receiver) = unbounded::<Message>();
                    {
                        let mut sockets = web_server.sockets.lock().await;
                        (*sockets).insert(id.clone(), sender);
                    }
                    web_server.send_cached_error_messages(&id).await;

                    let (mut tx, mut rx) = websocket.split();
                    spawn(async move {
                        loop {
                            match receiver.next().await {
                                Some(message) => {
                                    if let Err(error) = tx.send(message).await {
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
                        match rx.next().await {
                            Some(_) => (),
                            None => break,
                        }
                    }

                    let mut sockets = web_server.sockets.lock().await;
                    (*sockets).remove(&id);
                })
            });

        let serve_static = warp::get().and(warp::fs::dir(PathBuf::from(get_static_dir())));

        let routes = redirect_to_index_html
            .or(serve_engine)
            .or(serve_wasm_bundle)
            .or(serve_js_bundle)
            .or(handle_websocket)
            .or(serve_static);

        let _ = spawn(warp::serve(routes).run(([0, 0, 0, 0], option.port)));

        web_server
    }

    pub async fn send_error_messages(&self, error_messages: &Vec<ErrorMessage>) {
        let mut sockets = self.sockets.lock().await;
        let mut cached_error_messages = self.cached_error_messages.write().await;
        *cached_error_messages = error_messages.clone();
        for (id, socket) in sockets.iter_mut() {
            if let Err(error) = socket
                .send(Message::text(
                    serde_json::to_string(&WebsocketMessage::Error {
                        error_messages: error_messages.clone(),
                    })
                    .unwrap(),
                ))
                .await
            {
                eprintln!(
                    "channel send error while(channel -> websocket:{}).\n  {:?}",
                    id, error
                );
            }
        }
    }

    pub async fn send_cached_error_messages(&self, id: &String) {
        let mut sockets = self.sockets.lock().await;
        let error_messages = self.cached_error_messages.read().await;
        match sockets.get_mut(id) {
            Some(socket) => {
                let _ = socket
                    .send(Message::text(
                        serde_json::to_string(&WebsocketMessage::Error {
                            error_messages: error_messages.clone(),
                        })
                        .unwrap(),
                    ))
                    .await;
            }
            None => eprintln!("socket id {} not found", id),
        }
    }

    pub async fn request_reload(&self) {
        let mut sockets = self.sockets.lock().await;
        for (id, socket) in sockets.iter_mut() {
            if let Err(error) = socket
                .send(Message::text(
                    serde_json::to_string(&WebsocketMessage::Reload {}).unwrap(),
                ))
                .await
            {
                eprintln!(
                    "channel send error while(channel -> websocket:{}).\n  {:?}",
                    id, error
                );
            }
        }
    }
}

fn get_cli_root_path() -> PathBuf {
    let mut exe_path = current_exe().expect("Current exe path not found.");
    exe_path.pop();
    exe_path.pop();
    exe_path.pop();
    exe_path
}

fn get_static_dir() -> PathBuf {
    get_cli_root_path().join("www")
}

fn get_engine_dir() -> PathBuf {
    get_cli_root_path().join("engine")
}
