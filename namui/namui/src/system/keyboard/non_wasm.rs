use super::{pressing_code_set, record_key_down, record_key_up};
use crate::*;

pub(crate) fn on_keyboard_input(event: winit::event::KeyEvent) -> Option<RawEvent> {
    let code = Code::from_winit_key(&event.physical_key)?;

    match event.state {
        winit::event::ElementState::Pressed => {
            if !event.repeat {
                record_key_down(code);
            }

            Some(RawEvent::KeyDown {
                event: RawKeyboardEvent {
                    code,
                    pressing_codes: pressing_code_set(),
                },
            })
        }
        winit::event::ElementState::Released => {
            record_key_up(code);

            Some(RawEvent::KeyUp {
                event: RawKeyboardEvent {
                    code,
                    pressing_codes: pressing_code_set(),
                },
            })
        }
    }
}
