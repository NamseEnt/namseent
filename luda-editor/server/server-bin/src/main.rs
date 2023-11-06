#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    server_core::init().await;
    server_core::run_server().await
}
