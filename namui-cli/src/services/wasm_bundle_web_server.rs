use crate::{
    debug_println,
    types::{ErrorMessage, WebsocketMessage},
};
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future::join_all,
    lock::Mutex,
    SinkExt, StreamExt,
};
use nanoid::nanoid;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env::current_exe,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{spawn, sync::RwLock};
use warp::ws;
use warp::{hyper::Uri, ws::Message, Filter};

use super::rust_build_service::CargoBuildResult;

pub struct WasmBundleWebServer {
    sockets: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
    cached_error_messages: RwLock<Vec<ErrorMessage>>,
}

impl WasmBundleWebServer {
    pub(crate) fn start(port: u16, bundle_dir_path: &Path) -> Arc<Self> {
        let web_server = Arc::new(WasmBundleWebServer {
            sockets: Arc::new(Mutex::new(HashMap::new())),
            cached_error_messages: RwLock::new(Vec::new()),
        });

        let redirect_to_index_html =
            warp::path::end().map(|| warp::redirect(Uri::from_static("index.html")));

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

        let bundle_static = warp::get().and(warp::fs::dir(bundle_dir_path.to_path_buf()));
        let serve_static = warp::get().and(warp::fs::dir(PathBuf::from(get_static_dir())));

        let routes = redirect_to_index_html
            .or(bundle_static)
            .or(serve_static)
            .or(handle_websocket);

        let _ = tokio::spawn(warp::serve(routes).run(([0, 0, 0, 0], port)));

        web_server
    }

    pub async fn on_build_done(&self, result: &CargoBuildResult) {
        {
            debug_println!("on_build_done: locking web_server.cached_error_messages...");
            let mut cached_error_messages = self.cached_error_messages.write().await;
            debug_println!("on_build_done: web_server.cached_error_messages locked");
            *cached_error_messages = result.error_messages.clone();
        }

        let messages = if result.is_successful {
            [Message::text(
                serde_json::to_string(&WebsocketMessage::Reload {}).unwrap(),
            )]
        } else {
            [Message::text(
                serde_json::to_string(&WebsocketMessage::Error {
                    error_messages: result.error_messages.clone(),
                })
                .unwrap(),
            )]
        };

        debug_println!("on_build_done: locking web_server.sockets...");
        let mut sockets = self.sockets.lock().await;
        debug_println!("on_build_done: web_server.sockets locked");

        join_all(sockets.iter_mut().map(|(id, socket)| {
            let messages = messages.clone();
            async move {
                debug_println!("send_error_messages: sending to {}...", id);
                for message in &messages {
                    if let Err(error) = socket.send(message.clone()).await {
                        eprintln!("websocket send fail.\n  {:?}", error);
                    } else {
                        debug_println!("on_build_done: sended to {}", id);
                    }
                }
            }
        }))
        .await;
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
}

fn get_cli_root_path() -> PathBuf {
    let mut exe_path = get_exe_path_from_config()
        .or(current_exe())
        .expect("Current exe path not found.");
    exe_path.pop();
    exe_path.pop();
    exe_path.pop();
    exe_path
}

#[derive(Deserialize)]
struct Config {
    exe_path: String,
}

fn get_exe_path_from_config() -> Result<PathBuf, String> {
    let mut exe_or_symlink_path = current_exe().map_err(|error| error.to_string())?;
    exe_or_symlink_path.set_file_name("namui.config.json");
    let config_path = exe_or_symlink_path;

    let config_file = File::open(config_path).map_err(|error| error.to_string())?;
    let reader = BufReader::new(config_file);
    let config: Config = serde_json::from_reader(reader).map_err(|error| error.to_string())?;
    let exe_path = PathBuf::from(config.exe_path);
    Ok(exe_path)
}

fn get_static_dir() -> PathBuf {
    get_cli_root_path().join("www")
}
