use crate::{
    build::{
        bundle::Bundle,
        types::{ErrorMessage, WebsocketMessage},
    },
    debug_println,
};
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
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
        let serve_wasm_bundle = warp::path("bundle_bg.wasm").and_then(move || {
            let bundle = bundle.clone();
            async move {
                debug_println!("serve_wasm_bundle: locking web_server.bundle...");
                let wasm = bundle.read().await.wasm.clone();
                debug_println!("serve_wasm_bundle: web_server.bundle locked");

                match response::Builder::new()
                    .header("Content-Type", "application/wasm")
                    .body(wasm)
                {
                    Ok(reply) => Ok(reply),
                    Err(_) => Err(warp::reject()),
                }
            }
        });

        let bundle = option.bundle.clone();
        let serve_js_bundle = warp::path("bundle.js").and_then(move || {
            let bundle = bundle.clone();
            async move {
                debug_println!("serve_js_bundle: locking web_server.bundle...");
                let js = bundle.read().await.js.clone();
                debug_println!("serve_js_bundle: web_server.bundle locked");

                match response::Builder::new()
                    .header("Content-Type", "text/javascript")
                    .body(js)
                {
                    Ok(reply) => Ok(reply),
                    Err(_) => Err(warp::reject()),
                }
            }
        });

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
                        debug_println!("handle_websocket(open): locking web_server.sockets...");
                        let mut sockets = web_server.sockets.lock().await;
                        debug_println!("handle_websocket(open): web_server.sockets locked");
                        (*sockets).insert(id.clone(), sender);
                        debug_println!(
                            "handle_websocket(open): {} added to web_server.sockets",
                            id
                        );
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

                    debug_println!("handle_websocket(close): locking web_server.sockets...");
                    let mut sockets = web_server.sockets.lock().await;
                    debug_println!("handle_websocket(close): web_server.sockets locked");
                    (*sockets).remove(&id);
                    debug_println!(
                        "handle_websocket(close): {} removed from web_server.sockets",
                        id
                    );
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
        debug_println!("send_error_messages: locking web_server.sockets...");
        let mut sockets = self.sockets.lock().await;
        debug_println!("send_error_messages: web_server.sockets locked");

        debug_println!("send_error_messages: locking web_server.cached_error_messages...");
        let mut cached_error_messages = self.cached_error_messages.write().await;
        debug_println!("send_error_messages: web_server.cached_error_messages locked");
        *cached_error_messages = error_messages.clone();
        for (id, socket) in sockets.iter_mut() {
            debug_println!("send_error_messages: sending to {}...", id);
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
            debug_println!("send_error_messages: sended to {}", id);
        }
    }

    pub async fn send_cached_error_messages(&self, id: &String) {
        debug_println!("send_cached_error_messages: locking web_server.sockets...");
        let mut sockets = self.sockets.lock().await;
        debug_println!("send_cached_error_messages: web_server.sockets locked");

        debug_println!("send_cached_error_messages: locking web_server.cached_error_messages...");
        let error_messages = self.cached_error_messages.read().await;
        debug_println!("send_cached_error_messages: web_server.cached_error_messages locked");
        match sockets.get_mut(id) {
            Some(socket) => {
                debug_println!("send_cached_error_messages: sending to {}...", id);
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
        debug_println!("send_cached_error_messages: sended to {}", id);
    }

    pub async fn request_reload(&self) {
        debug_println!("request_reload: locking web_server.sockets...");
        let mut sockets = self.sockets.lock().await;
        debug_println!("request_reload: web_server.sockets locked");

        for (id, socket) in sockets.iter_mut() {
            debug_println!("request_reload: sending to {}...", id);
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
            debug_println!("request_reload: sended to {}", id);
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
