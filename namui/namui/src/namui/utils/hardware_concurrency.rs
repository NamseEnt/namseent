#[cfg(target_os = "wasi")]
pub(crate) async fn hardware_concurrency() -> u32 {
    crate::wasi::hardware_concurrency().await
}

#[cfg(not(target_os = "wasi"))]
pub(crate) async fn hardware_concurrency() -> u32 {
    std::thread::available_parallelism().unwrap_or(1) as u32
}
