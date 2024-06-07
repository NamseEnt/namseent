use super::{clear_pressing_code_set, pressing_code_set, record_key_down, record_key_up};
use crate::*;
use std::str::FromStr;

pub(crate) fn on_key_down(code_str: &str) -> RawEvent {
    let code = Code::from_str(code_str).unwrap();
    record_key_down(code);

    RawEvent::KeyDown {
        event: RawKeyboardEvent {
            code,
            pressing_codes: pressing_code_set(),
        },
    }
}

pub(crate) fn on_key_up(code_str: &str) -> RawEvent {
    let code = Code::from_str(code_str).unwrap();
    record_key_up(code);

    RawEvent::KeyUp {
        event: RawKeyboardEvent {
            code,
            pressing_codes: pressing_code_set(),
        },
    }
}

pub(crate) fn on_blur() {
    clear_pressing_code_set();
}

pub(crate) fn on_visibility_change() {
    clear_pressing_code_set();
}
