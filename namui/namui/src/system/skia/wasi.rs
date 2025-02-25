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

unsafe extern "C" {
    fn take_bitmap();
}

pub(super) fn after_draw() {
    unsafe { take_bitmap() };
}
