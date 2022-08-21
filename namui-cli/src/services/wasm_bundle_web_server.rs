use super::{bundle::NamuiBundleManifest, rust_build_service::CargoBuildResult};
use crate::{
    debug_println,
    types::{ErrorMessage, WebsocketMessage},
    util::get_cli_root_path,
};
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future::join_all,
    SinkExt, StreamExt,
};
use nanoid::nanoid;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tokio::{spawn, sync::RwLock};
use tokio_util::codec::{BytesCodec, FramedRead};
use urlencoding::decode;
use warp::{
    http::HeaderValue,
    hyper::{header::CONTENT_TYPE, Body, Uri},
    reject, reply,
    ws::Message,
    Filter,
};
use warp::{path::Tail, ws};

pub struct WasmBundleWebServer {
    sockets: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
    cached_error_messages: RwLock<Vec<ErrorMessage>>,
    namui_bundle_manifest: Arc<Mutex<Option<NamuiBundleManifest>>>,
}

impl WasmBundleWebServer {
    pub(crate) fn start(port: u16, wasm_bundle_dir_path: &Path) -> Arc<Self> {
        let web_server = Arc::new(WasmBundleWebServer {
            sockets: Arc::new(Mutex::new(HashMap::new())),
            cached_error_messages: RwLock::new(Vec::new()),
            namui_bundle_manifest: Arc::new(Mutex::new(None)),
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
                    let (mut sender, mut receiver) = unbounded::<Message>();
                    {
                        debug_println!("handle_websocket(open): locking web_server.sockets...");
                        let mut sockets = web_server.sockets.lock().unwrap();
                        debug_println!("handle_websocket(open): web_server.sockets locked");
                        (*sockets).insert(id.clone(), sender.clone());
                        debug_println!(
                            "handle_websocket(open): {} added to web_server.sockets",
                            id
                        );
                    }
                    web_server.send_cached_error_messages(&mut sender).await;

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

                    {
                        debug_println!("handle_websocket(close): locking web_server.sockets...");
                        let mut sockets = web_server.sockets.lock().unwrap();
                        debug_println!("handle_websocket(close): web_server.sockets locked");
                        (*sockets).remove(&id);
                        debug_println!(
                            "handle_websocket(close): {} removed from web_server.sockets",
                            id
                        );
                    }
                })
            });

        let wasm_bundle_static = warp::get().and(warp::fs::dir(wasm_bundle_dir_path.to_path_buf()));
        let serve_static = warp::get().and(warp::fs::dir(PathBuf::from(get_static_dir())));
        let bundle_metadata_static =
            create_bundle_metadata_static(web_server.namui_bundle_manifest.clone());
        let bundle_static = create_bundle_static(web_server.namui_bundle_manifest.clone());

        let routes = redirect_to_index_html
            .or(wasm_bundle_static)
            .or(serve_static)
            .or(bundle_metadata_static)
            .or(bundle_static)
            .or(handle_websocket)
            .map(|reply| warp::reply::with_header(reply, "cache-control", "no-cache"));

        let _ = tokio::spawn(warp::serve(routes).run(([0, 0, 0, 0], port)));

        web_server
    }

    pub async fn on_build_done(
        &self,
        result: &CargoBuildResult,
        bundle_manifest: Option<NamuiBundleManifest>,
    ) {
        {
            *self.namui_bundle_manifest.lock().unwrap() = bundle_manifest;
        }
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
        let mut sockets = { self.sockets.lock().unwrap().clone() };
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

    pub async fn send_cached_error_messages(&self, socket: &mut UnboundedSender<Message>) {
        debug_println!("send_cached_error_messages: locking web_server.cached_error_messages...");
        let error_messages = self.cached_error_messages.read().await;
        debug_println!("send_cached_error_messages: web_server.cached_error_messages locked");
        let result = socket
            .send(Message::text(
                serde_json::to_string(&WebsocketMessage::Error {
                    error_messages: error_messages.clone(),
                })
                .unwrap(),
            ))
            .await;
        if let Err(error) = result {
            eprintln!("send_cached_error_messages fail.\n  {:?}", error);
        }
    }
}

fn get_static_dir() -> PathBuf {
    get_cli_root_path().join("www")
}

fn create_bundle_metadata_static(
    namui_bundle_manifest: Arc<Mutex<Option<NamuiBundleManifest>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let bundle_metadata_static = warp::get()
        .and(warp::path("bundle_metadata.json"))
        .and_then(move || {
            let namui_bundle_manifest = namui_bundle_manifest.clone();
            let result = {
                let namui_bundle_manifest = namui_bundle_manifest.lock().unwrap();
                match namui_bundle_manifest.as_ref() {
                    Some(namui_bundle_manifest) => {
                        json_response(namui_bundle_manifest.metadata_json().to_string())
                    }
                    None => Err(reject::reject()),
                }
            };
            async move { result }
        });
    bundle_metadata_static
}

fn create_bundle_static(
    namui_bundle_manifest: Arc<Mutex<Option<NamuiBundleManifest>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let bundle_static = warp::get()
        .and(warp::path("bundle"))
        .and(warp::path::tail())
        .and_then(move |tail: Tail| {
            let namui_bundle_manifest = namui_bundle_manifest.clone();
            let src_path = (|| {
                let url = {
                    let tail = decode(tail.as_str());
                    if tail.is_err() {
                        return Err(reject::reject());
                    } else {
                        let tail = tail.unwrap().into_owned();
                        PathBuf::from(&tail)
                    }
                };

                let namui_bundle_manifest = namui_bundle_manifest.lock().unwrap();
                match namui_bundle_manifest.as_ref() {
                    Some(namui_bundle_manifest) => {
                        match namui_bundle_manifest
                            .get_src_path(&url)
                            .map_err(|_| reject::reject())
                        {
                            Ok(src_path) => match src_path {
                                Some(src_path) => Ok(src_path),
                                None => Err(reject::not_found()),
                            },
                            Err(_) => Err(reject::reject()),
                        }
                    }
                    None => Err(reject::reject()),
                }
            })();

            async move {
                match src_path {
                    Ok(src_path) => file_response(&src_path).await,
                    Err(err) => Err(err),
                }
            }
        });
    bundle_static
}

async fn file_response(src_path: &PathBuf) -> Result<reply::Response, warp::Rejection> {
    match tokio::fs::File::open(src_path).await {
        Ok(file) => {
            let frame_reader = FramedRead::new(file, BytesCodec::new());
            let response = reply::Response::new(Body::wrap_stream(frame_reader));
            Ok(response)
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => Err(reject::not_found()),
            _ => Err(reject::reject()),
        },
    }
}

fn json_response(json_string: String) -> Result<reply::Response, warp::Rejection> {
    let mut response = reply::Response::new(Body::from(json_string));
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    Ok(response)
}
