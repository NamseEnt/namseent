pub(super) async fn init() -> std::sync::Arc<dyn SkSkia + Send + Sync> {
    namui_skia::init_skia(None)
}
