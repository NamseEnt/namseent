mod api;
mod kv_store;
mod s3;
mod session;

use anyhow::Result;
use axum::{
    extract::{ConnectInfo, State, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use kv_store::{InMemoryCachedKsStore, KvStore, SqliteKvStore};
use s3::*;
use session::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    real_main().await
}

async fn real_main() -> Result<()> {
    tracing_subscriber::fmt::init();
    println!("is_on_aws: {}", is_on_aws());
    init_s3().await?;
    start_server().await
}

#[derive(Clone)]
pub(crate) struct Db {
    // pub(crate) s3: InMemoryCachedKsStore<S3KsStore>,
    pub(crate) sqlite: InMemoryCachedKsStore<SqliteKvStore>,
}

async fn start_server() -> Result<()> {
    let sqlite_kv_store = SqliteKvStore::new().await?;
    let in_memory_cached_sqlite = InMemoryCachedKsStore::new(sqlite_kv_store, !is_on_aws());

    // let s3_kv_store = S3KsStore::new(s3().clone(), bucket_name());
    // let in_memory_cached_s3 = InMemoryCachedKsStore::new(s3_kv_store, !is_on_aws());

    let db = Db {
        // s3: in_memory_cached_s3,
        sqlite: in_memory_cached_sqlite,
    };

    let app = Router::new()
        .route("/turn_on_memory_cache", get(turn_on_memory_cache))
        .route("/turn_off_memory_cache", get(turn_off_memory_cache))
        .route("/health", get(|| async { "Good" }))
        .route("/ws", get(ws_handler))
        .with_state(db);

    let port = if is_on_aws() {
        std::env::var("PORT").unwrap()
    } else {
        "8080".to_string()
    };

    let addr = format!("[::]:{port}").parse()?;

    println!("Listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade, State(db): State<Db>) -> Response {
    // TODO: Multiplexing. Don't wait the previous request to finish before starting the next one.
    // But sometime it should be waited. Make a rule for that.
    ws.on_upgrade(|mut socket| async move {
        use axum::extract::ws::Message;

        let session = Session::new();
        while let Some(msg) = socket.recv().await {
            let Ok(msg) = msg else {
                // client disconnected
                return;
            };

            let mut in_packet = match msg {
                Message::Binary(buffer) => buffer,
                Message::Ping(_) => {
                    if socket.send(Message::Pong(Vec::new())).await.is_err() {
                        return;
                    };
                    continue;
                }
                Message::Text(_) | Message::Pong(_) | Message::Close(_) => return,
            };

            // in packet = [payload][2byte packet_type]
            if in_packet.len() < 2 {
                return;
            }
            let packet_type: u16 =
                u16::from_be_bytes([in_packet.pop().unwrap(), in_packet.pop().unwrap()]);
            let in_payload = &in_packet[..];

            enum Status {
                Ok = 0,
                ServerError = 1,
            }
            let response: Option<(Vec<u8>, Status)> = match packet_type {
                0 => {
                    let Ok(request) = rkyv::validation::validators::check_archived_root::<
                        api::google_auth::Request,
                    >(in_payload) else {
                        return;
                    };
                    match api::google_auth::google_auth(request, db.clone(), session.clone())
                        .await
                        .and_then(|response| Ok(rkyv::to_bytes::<_, 64>(&response)?))
                    {
                        Ok(bytes) => Some((bytes.into_vec(), Status::Ok)),
                        Err(error) => {
                            eprintln!("Error on google_auth: {:?}", error);
                            Some((Vec::new(), Status::ServerError))
                        }
                    }
                }
                _ => return,
            };
            let Some((mut out_payload, status)) = response else {
                continue;
            };

            /*
                out packet = [payload][1byte status][2byte packet_type]
            */

            out_payload.extend_from_slice(&(status as u8).to_be_bytes());
            out_payload.extend_from_slice(&packet_type.to_be_bytes());

            if socket.send(Message::Binary(out_payload)).await.is_err() {
                // client disconnected
                return;
            }
        }
    })
}

fn is_on_aws() -> bool {
    std::env::var("IS_ON_AWS").is_ok()
}

async fn turn_on_memory_cache(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(db): State<Db>,
) -> impl IntoResponse {
    if !addr.ip().is_loopback() {
        return "Not allowed";
    }

    // db.s3.set_enabled(true);
    db.sqlite.set_enabled(true);
    "ok"
}

async fn turn_off_memory_cache(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(db): State<Db>,
) -> impl IntoResponse {
    if !addr.ip().is_loopback() {
        return "Not allowed";
    }

    // db.s3.set_enabled(false);
    db.sqlite.set_enabled(false);
    "ok"
}
