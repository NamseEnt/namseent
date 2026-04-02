use crate::*;

/// Catches panics inside extern "C" FFI functions.
/// Without this, panics crossing the extern "C" boundary are UB and cause
/// silent aborts with no error message.
macro_rules! ffi_catch {
    ($body:expr) => {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(val) => val,
            Err(e) => {
                let msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "unknown panic".to_string()
                };
                eprintln!("[namui FFI panic] {msg}");
                std::process::abort();
            }
        }
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn _init_system() {
    ffi_catch!(crate::system::init_system().unwrap());
}

#[unsafe(no_mangle)]
pub extern "C" fn _set_screen_size(width: u16, height: u16) {
    ffi_catch!(crate::system::screen::set_size(width, height));
}

#[unsafe(no_mangle)]
pub extern "C" fn _shutdown() {
    ffi_catch!({
        crate::LOOPER.take();
        crate::TOKIO_RUNTIME.take().unwrap().shutdown_background();
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_animation_frame() -> *const u8 {
    ffi_catch!(crate::on_event(RawEvent::ScreenRedraw))
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_screen_resize(width: u16, height: u16) -> *const u8 {
    ffi_catch!({
        crate::system::screen::set_size(width, height);
        crate::on_event(RawEvent::ScreenResize {
            wh: Wh::new(int_px(width as i32), int_px(height as i32)),
        })
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_mouse_down(x: f32, y: f32, button: u8, buttons: u8) -> *const u8 {
    ffi_catch!({
        let raw_event = crate::system::mouse::on_mouse_down(x, y, button, buttons);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_mouse_move(x: f32, y: f32, _button: u8, buttons: u8) -> *const u8 {
    ffi_catch!({
        let raw_event = crate::system::mouse::on_mouse_move(x, y, buttons);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_mouse_up(x: f32, y: f32, button: u8, buttons: u8) -> *const u8 {
    ffi_catch!({
        let raw_event = crate::system::mouse::on_mouse_up(x, y, button, buttons);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_mouse_wheel(delta_x: f32, delta_y: f32, x: f32, y: f32) -> *const u8 {
    ffi_catch!({
        let raw_event = crate::system::mouse::on_mouse_wheel(delta_x, delta_y, x, y);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_key_down(code: u8) -> *const u8 {
    ffi_catch!({
        let raw_event = crate::system::keyboard::key_down(code);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_key_up(code: u8) -> *const u8 {
    ffi_catch!({
        let raw_event = crate::system::keyboard::key_up(code);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_blur() -> *const u8 {
    ffi_catch!(crate::on_event(RawEvent::Blur))
}

#[unsafe(no_mangle)]
pub extern "C" fn _on_visibility_change() -> *const u8 {
    ffi_catch!(crate::on_event(RawEvent::VisibilityChange))
}

#[unsafe(no_mangle)]
pub extern "C" fn _freeze_world() -> *const u8 {
    ffi_catch!({
        let looper = crate::LOOPER.with_borrow_mut(|looper| looper.take().unwrap());
        let frozen_states = looper.world.freeze_states();
        crate::write_response(&frozen_states)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn _set_freeze_states(ptr: *const u8, len: usize) {
    ffi_catch!({
        crate::LOOPER.with_borrow_mut(|looper| {
            looper
                .as_mut()
                .unwrap()
                .world
                .set_frozen_states(unsafe { std::slice::from_raw_parts(ptr, len) });
        });
    });
}
