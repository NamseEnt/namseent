use super::{pressing_code_set, record_key_down, record_key_up, KeyboardSystem};
use crate::*;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

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
                },
            });
        }
        winit::event::ElementState::Released => {
            record_key_up(code);

            crate::hooks::on_raw_event(RawEvent::KeyUp {
                event: RawKeyboardEvent {
                    code,
                    pressing_codes: pressing_code_set(),
                },
            });
        }
    }
}
