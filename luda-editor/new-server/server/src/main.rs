mod api;
mod new_id;
mod s3;
mod session;
mod ws_handler;

use anyhow::{Result, anyhow, bail};
use axum::{Router, routing::get};
use database::Database;
use new_id::new_id;
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
    let database = database::init(".db").await?;

    let app = Router::new()
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
