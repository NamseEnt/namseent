use super::InitResult;
use crate::*;
use dashmap::DashSet;
use std::{collections::HashSet, sync::OnceLock};

static PRESSING_CODE_SET: OnceLock<DashSet<Code>> = OnceLock::new();

pub(super) fn init() -> InitResult {
    let _ = PRESSING_CODE_SET.set(DashSet::new());
    Ok(())
}

pub fn any_code_press(codes: impl IntoIterator<Item = Code>) -> bool {
    let pressing_code_set = PRESSING_CODE_SET.get().unwrap();
    for code in codes {
        if pressing_code_set.contains(&code) {
            return true;
        }
    }
    false
}

pub fn shift_press() -> bool {
    any_code_press([Code::ShiftLeft, Code::ShiftRight])
}
pub fn ctrl_press() -> bool {
    any_code_press([Code::ControlLeft, Code::ControlRight])
}
pub fn alt_press() -> bool {
    any_code_press([Code::AltLeft, Code::AltRight])
}

pub(crate) fn record_key_down(code: Code) {
    let pressing_code_set = PRESSING_CODE_SET.get().unwrap();
    pressing_code_set.insert(code);
}

pub(crate) fn record_key_up(code: Code) {
    let pressing_code_set = PRESSING_CODE_SET.get().unwrap();
    pressing_code_set.remove(&code);
}

pub(crate) fn pressing_code_set() -> HashSet<Code> {
    let pressing_code_set = PRESSING_CODE_SET.get().unwrap();
    pressing_code_set.iter().map(|code| *code).collect()
}

pub(crate) fn key_down(code: u8) -> RawEvent {
    let code = Code::try_from(code).unwrap_or_else(|_| panic!("invalid code {code}"));
    record_key_down(code);
    RawEvent::KeyDown {
        event: RawKeyboardEvent {
            code,
            pressing_codes: pressing_code_set(),
        },
    }
}

pub(crate) fn key_up(code: u8) -> RawEvent {
    let code = Code::try_from(code).unwrap_or_else(|_| panic!("invalid code {code}"));
    record_key_up(code);
    RawEvent::KeyUp {
        event: RawKeyboardEvent {
            code,
            pressing_codes: pressing_code_set(),
        },
    }
}
