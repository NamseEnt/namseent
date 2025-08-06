mod manage_server;

use anyhow::Result;
use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use manage_server::keep_server_updated;
use std::{path::PathBuf, sync::atomic::AtomicU16};

static SERVER_PORT: AtomicU16 = AtomicU16::new(0);
fn update_server_port(port: u16) {
    SERVER_PORT.store(port, std::sync::atomic::Ordering::Relaxed);
}

const CERT_DIR: &str = "/etc/letsencrypt/live/visual-novel.namseent.com";
fn cert_dir() -> PathBuf {
    PathBuf::from(CERT_DIR)
}

#[tokio::main]
async fn main() -> Result<()> {
    real_main().await
}

async fn real_main() -> Result<()> {
    keep_server_updated();
    start_port_server().await
}

async fn start_port_server() -> Result<()> {
    let config =
        RustlsConfig::from_pem_file(cert_dir().join("cert.pem"), cert_dir().join("key.pem"))
            .await?;

    tokio::task::spawn({
        let config = config.clone();
        async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(24 * 3600)).await;
                if let Err(err) = config
                    .reload_from_pem_file(cert_dir().join("cert.pem"), cert_dir().join("key.pem"))
                    .await
                {
                    eprintln!("Failed to reload cert: {err}");
                }
            }
        }
    });

    let app = Router::new().route(
        "/",
        get(|| async {
            SERVER_PORT
                .load(std::sync::atomic::Ordering::Relaxed)
                .to_string()
        }),
    );

    let addr = "[::]:443".parse()?;
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
