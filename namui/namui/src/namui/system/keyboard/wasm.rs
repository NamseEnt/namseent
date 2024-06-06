use super::{clear_pressing_code_set, pressing_code_set, record_key_down, record_key_up};
use crate::*;

#[no_mangle]
pub extern "C" fn on_key_down(code_u8: u8) {
    let code = Code::try_from(code_u8).unwrap();
    record_key_down(code);

    crate::hooks::on_raw_event(RawEvent::KeyDown {
        event: RawKeyboardEvent {
            code,
            pressing_codes: pressing_code_set(),
        },
    });
}

#[no_mangle]
pub extern "C" fn on_key_up(code_u8: u8) {
    let code = Code::try_from(code_u8).unwrap();
    record_key_up(code);

    crate::hooks::on_raw_event(RawEvent::KeyUp {
        event: RawKeyboardEvent {
            code,
            pressing_codes: pressing_code_set(),
        },
    });
}

#[no_mangle]
pub extern "C" fn on_blur() {
    clear_pressing_code_set();
}

#[no_mangle]
pub extern "C" fn on_visibility_change() {
    clear_pressing_code_set();
}
