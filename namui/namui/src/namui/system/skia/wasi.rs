use super::*;
use namui_skia::*;

pub(crate) fn init() -> Result<()> {
    while DRAW_COMMAND_TX.get().is_none() {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    Ok(())
}

pub(super) fn init_skia() -> Result<NativeSkia> {
    let skia = namui_skia::init_skia(crate::system::screen::size())?;
    Ok(skia)
}

extern "C" {
    fn take_bitmap(width: i32, height: i32);
}

pub(super) fn after_draw(screen_size: Wh<IntPx>) {
    unsafe { take_bitmap(screen_size.width.as_i32(), screen_size.height.as_i32()) };
}
