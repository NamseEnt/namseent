mod api;
mod s3;
mod session;
mod ws_handler;

use anyhow::{anyhow, bail, Result};
use axum::{
    extract::{ConnectInfo, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use database::Database;
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

async fn start_server() -> Result<()> {
    let database = database::init(
        s3().clone(),
        database_bucket_name().to_string(),
        !is_on_aws(),
    )
    .await?;

    let app = Router::new()
        .route("/turn_on_memory_cache", get(turn_on_memory_cache))
        .route("/turn_off_memory_cache", get(turn_off_memory_cache))
        .route("/health", get(|| async { "Good" }))
        .route("/ws", get(ws_handler::ws_handler))
        .with_state(database);

    let port = if is_on_aws() {
        std::env::var("PORT").unwrap()
    } else {
        "8080".to_string()
    };

    let addr = format!("0.0.0.0:{port}").parse()?;

    println!("Listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

fn is_on_aws() -> bool {
    std::env::var("IS_ON_AWS").is_ok()
}

async fn turn_on_memory_cache(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(db): State<Database>,
) -> impl IntoResponse {
    if !addr.ip().is_loopback() {
        return "Not allowed";
    }

    db.set_memory_cache(true);
    "ok"
}

async fn turn_off_memory_cache(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(db): State<Database>,
) -> impl IntoResponse {
    if !addr.ip().is_loopback() {
        return "Not allowed";
    }

    db.set_memory_cache(false);
    "ok"
}
