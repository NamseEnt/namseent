#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    server_core::init().await;
    return server_core::run_server().await;
}
