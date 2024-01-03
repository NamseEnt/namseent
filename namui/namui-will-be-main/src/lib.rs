// use this in user application side.

type NamuiStartType =
    unsafe extern "Rust" fn(&(dyn 'static + Send + Sync + Fn() -> (dyn Component)));
#[no_mangle]
pub extern "Rust" fn set_namui_start(f: NamuiStartType) {
    unsafe {
        NAMUI_START = f;
    }
}
unsafe extern "Rust" fn uninit_namui_start(
    _component: &(dyn 'static + Send + Sync + Fn() -> (dyn Component)),
) {
    panic!("not initialized");
}
static mut NAMUI_START: NamuiStartType = uninit_namui_start;

pub fn start(component: &(dyn 'static + Send + Sync + Fn() -> (dyn Component))) {
    unsafe { NAMUI_START(component) }
}

pub trait Component {}
