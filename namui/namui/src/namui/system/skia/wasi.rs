use namui_skia::*;

pub(super) fn init_skia() -> Result<NativeSkia> {
    let skia = namui_skia::init_skia(crate::system::screen::size())?;

    Ok(skia)
}
