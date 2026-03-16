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
pub extern "C" fn namui_init_system() {
    ffi_catch!(crate::system::init_system().unwrap());
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_set_screen_size(width: u16, height: u16) {
    ffi_catch!(crate::system::screen::set_size(width, height));
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_shutdown() {
    ffi_catch!({
        // Drop LOOPER first while TOKIO_RUNTIME is still alive,
        // to avoid TLS destruction order issues.
        crate::LOOPER.take();
        crate::TOKIO_RUNTIME.take().unwrap().shutdown_background();
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_screen_redraw(out_ptr: *mut *const u8, out_len: *mut usize) -> u64 {
    ffi_catch!({
        let result = crate::on_event(RawEvent::ScreenRedraw);
        if result == 1 {
            unsafe {
                *out_ptr = crate::LAST_EVENT_RESULT_PTR.get() as *const u8;
                *out_len = crate::LAST_EVENT_RESULT_LEN.get();
            }
        }
        result
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_screen_resize(width: u16, height: u16) -> u64 {
    ffi_catch!({
        crate::system::screen::set_size(width, height);
        crate::on_event(RawEvent::ScreenResize {
            wh: Wh::new(int_px(width as i32), int_px(height as i32)),
        })
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_mouse_move(x: f32, y: f32) -> u64 {
    ffi_catch!({
        let raw_event = crate::system::mouse::non_wasm::on_mouse_move(x, y);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_mouse_down(button: u8, x: f32, y: f32) -> u64 {
    ffi_catch!({
        let Some(btn) = crate::system::mouse::button_from_u8(button) else {
            return 0;
        };
        crate::system::mouse::non_wasm::on_mouse_move(x, y);
        let raw_event = crate::system::mouse::non_wasm::on_mouse_input(true, btn);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_mouse_up(button: u8, x: f32, y: f32) -> u64 {
    ffi_catch!({
        let Some(btn) = crate::system::mouse::button_from_u8(button) else {
            return 0;
        };
        crate::system::mouse::non_wasm::on_mouse_move(x, y);
        let raw_event = crate::system::mouse::non_wasm::on_mouse_input(false, btn);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_mouse_wheel(delta_x: f32, delta_y: f32, mouse_x: f32, mouse_y: f32) -> u64 {
    ffi_catch!({
        crate::system::mouse::non_wasm::on_mouse_move(mouse_x, mouse_y);
        let raw_event = crate::system::mouse::non_wasm::on_mouse_wheel(delta_x, delta_y);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_key_down(code: u8) -> u64 {
    ffi_catch!({
        let raw_event = crate::system::keyboard::key_down(code);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_key_up(code: u8) -> u64 {
    ffi_catch!({
        let raw_event = crate::system::keyboard::key_up(code);
        crate::on_event(raw_event)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_on_blur() -> u64 {
    ffi_catch!(crate::on_event(RawEvent::Blur))
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_get_result_ptr() -> usize {
    ffi_catch!(crate::LAST_EVENT_RESULT_PTR.get())
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_get_result_len() -> usize {
    ffi_catch!(crate::LAST_EVENT_RESULT_LEN.get())
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_freeze_world() {
    ffi_catch!({
        let looper = crate::LOOPER.with_borrow_mut(|looper| looper.take().unwrap());
        let frozen_states = looper.world.freeze_states();
        crate::FROZEN_STATES.with_borrow_mut(|bytes| {
            *bytes = frozen_states.into_boxed_slice();
        });
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_get_frozen_ptr() -> usize {
    ffi_catch!(crate::FROZEN_STATES.with_borrow(|bytes| bytes.as_ptr() as usize))
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_get_frozen_len() -> usize {
    ffi_catch!(crate::FROZEN_STATES.with_borrow(|bytes| bytes.len()))
}

#[unsafe(no_mangle)]
pub extern "C" fn namui_set_freeze_states(ptr: *const u8, len: usize) {
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
