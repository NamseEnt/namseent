#[cfg(target_os = "wasi")]
mod wasi;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use non_wasm as inner;
#[cfg(target_os = "wasi")]
use wasi as inner;

use super::InitResult;
use anyhow::Result;
pub(crate) use inner::*;
use namui_skia::*;
use namui_type::*;
use std::sync::*;

static SK_CALCULATE: OnceLock<Arc<dyn SkCalculate + Send + Sync>> = OnceLock::new();

#[derive(Debug)]
enum DrawingCommand {
    Draw { rendering_tree: RenderingTree },
    Resize { wh: Wh<IntPx> },
}

pub(super) fn init() -> InitResult {
    inner::init()?;

    let calculate = namui_skia::init_calculate()?;
    SK_CALCULATE
        .set(calculate)
        .map_err(|_| unreachable!())
        .unwrap();

    // let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    // DRAW_COMMAND_TX.set(tx).map_err(|_| unreachable!()).unwrap();
    // spawn_drawing_task(rx);

    Ok(())
}

pub async fn load_typeface(typeface_name: String, bytes: Vec<u8>) -> Result<()> {
    sk_calculate()
        .load_typeface(typeface_name.to_string(), bytes.to_vec())
        .await??;

    Ok(())
}

pub async fn load_image_from_url(url: impl AsRef<str>) -> Result<Image> {
    let bytes = crate::system::network::http::get_bytes(url.as_ref()).await?;
    let image = sk_calculate()
        .load_image_from_encoded(bytes.as_ref())
        .await?;

    Ok(image)
}

pub async fn load_image_from_raw(image_info: ImageInfo, bytes: &[u8]) -> Result<Image> {
    let image = sk_calculate()
        .load_image_from_raw(image_info, bytes)
        .await?;

    Ok(image)
}

pub(crate) fn sk_calculate() -> &'static dyn SkCalculate {
    SK_CALCULATE.get().unwrap().as_ref()
}

pub(crate) fn group_glyph(font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
    sk_calculate().group_glyph(font, paint)
}

pub(crate) fn font_metrics(font: &Font) -> Option<FontMetrics> {
    sk_calculate().font_metrics(font)
}

pub(crate) fn on_window_resize(wh: Wh<IntPx>) {
    send_command(DrawingCommand::Resize { wh });
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    send_command(DrawingCommand::Draw { rendering_tree });
}

fn send_command(command: DrawingCommand) {
    inner::send_command(command);
}
