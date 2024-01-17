use super::{pressing_code_set, record_key_down, record_key_up, KeyboardSystem};
use crate::{Code, RawEvent, RawKeyboardEvent};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

impl KeyboardSystem {
    pub fn new() -> Self {
        let pressing_code_set = Arc::new(RwLock::new(HashSet::new()));

        KeyboardSystem { pressing_code_set }
    }
}

pub(crate) fn on_keyboard_input(event: winit::event::KeyEvent) {
    let Some(code) = Code::from_winit_key(&event.physical_key) else {
        return;
    };
    match event.state {
        winit::event::ElementState::Pressed => {
            if !event.repeat {
                record_key_down(code);
            }

            crate::hooks::on_raw_event(RawEvent::KeyDown {
                event: RawKeyboardEvent {
                    code,
                    pressing_codes: pressing_code_set(),
                    prevent_default: Box::new(|| {}),
                },
            });
        }
        winit::event::ElementState::Released => {
            record_key_up(code);

            crate::hooks::on_raw_event(RawEvent::KeyUp {
                event: RawKeyboardEvent {
                    code,
                    pressing_codes: pressing_code_set(),
                    prevent_default: Box::new(|| {}),
                },
            });
        }
    }
}
